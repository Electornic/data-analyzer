use crate::read_file::DataSet;
use crate::analyze::select_columns;
use anyhow::Result;
use statrs::statistics::{Statistics, OrderStatistics};
use statrs::distribution::{StudentsT, ContinuousCDF};
use std::io::{self, Write};

pub fn perform_independent_samples_t_test(dataset: &DataSet) -> Result<()> {
    println!("\n=== 독립표본 t 검정 ===");
    
    // 숫자형 열만 필터링
    let numeric_headers: Vec<String> = dataset.headers.iter()
        .filter(|header| dataset.get_numeric_column(header).is_ok())
        .cloned()
        .collect();
    
    if numeric_headers.len() < 2 {
        println!("독립표본 t 검정을 위해서는 최소 2개의 숫자형 열이 필요합니다.");
        return Ok(());
    }
    
    println!("두 개의 독립된 그룹을 비교합니다.");
    
    // 첫 번째 그룹 선택
    let group1_columns = select_columns(&numeric_headers, "첫 번째 그룹의 열을 선택하세요:", false)?;
    let group1_column = &group1_columns[0];
    
    // 두 번째 그룹 선택 (첫 번째 그룹과 다른 열)
    let remaining_headers: Vec<String> = numeric_headers.iter()
        .filter(|header| *header != group1_column)
        .cloned()
        .collect();
    
    if remaining_headers.is_empty() {
        println!("두 번째 그룹으로 선택할 수 있는 열이 없습니다.");
        return Ok(());
    }
    
    let group2_columns = select_columns(&remaining_headers, "두 번째 그룹의 열을 선택하세요:", false)?;
    let group2_column = &group2_columns[0];
    
    // 유의수준 입력받기
    println!("유의수준을 입력하세요 (기본값: 0.05):");
    print!("유의수준 (α): ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let alpha = if input.trim().is_empty() {
        0.05
    } else {
        input.trim().parse().unwrap_or(0.05)
    };
    
    // 대립가설 선택
    println!("대립가설을 선택하세요:");
    println!("1. μ₁ ≠ μ₂ (양측검정)");
    println!("2. μ₁ > μ₂ (우측검정)");
    println!("3. μ₁ < μ₂ (좌측검정)");
    print!("선택 (1-3): ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let test_type = input.trim();
    
    // 등분산 가정 여부
    println!("등분산을 가정하시겠습니까? (y/n, 기본값: y):");
    print!("등분산 가정: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let equal_variance = input.trim().to_lowercase() != "n";
    
    // 데이터 추출 및 검정 수행
    match (dataset.get_numeric_column(group1_column), dataset.get_numeric_column(group2_column)) {
        (Ok(data1), Ok(data2)) => {
            perform_independent_t_test(&data1, &data2, alpha, test_type, equal_variance, group1_column, group2_column)?;
        }
        (Err(e), _) => {
            println!("'{}' 열의 숫자 데이터 추출 실패: {}", group1_column, e);
        }
        (_, Err(e)) => {
            println!("'{}' 열의 숫자 데이터 추출 실패: {}", group2_column, e);
        }
    }
    
    Ok(())
}

fn perform_independent_t_test(
    data1: &[f64], 
    data2: &[f64], 
    alpha: f64, 
    test_type: &str, 
    equal_variance: bool,
    group1_name: &str,
    group2_name: &str
) -> Result<()> {
    if data1.len() < 2 || data2.len() < 2 {
        return Err(anyhow::anyhow!("각 그룹은 최소 2개의 데이터가 필요합니다."));
    }
    
    // 기본 통계량 계산
    let n1 = data1.len() as f64;
    let n2 = data2.len() as f64;
    let mean1 = data1.mean();
    let mean2 = data2.mean();
    let std1 = data1.std_dev();
    let std2 = data2.std_dev();
    let var1 = std1 * std1;
    let var2 = std2 * std2;
    
    // t 통계량과 자유도 계산
    let (t_statistic, df) = if equal_variance {
        // 등분산 가정: pooled variance 사용
        let pooled_variance = ((n1 - 1.0) * var1 + (n2 - 1.0) * var2) / (n1 + n2 - 2.0);
        let standard_error = (pooled_variance * (1.0/n1 + 1.0/n2)).sqrt();
        let t = (mean1 - mean2) / standard_error;
        let degrees_of_freedom = n1 + n2 - 2.0;
        (t, degrees_of_freedom)
    } else {
        // 등분산 가정하지 않음: Welch's t-test
        let standard_error = (var1/n1 + var2/n2).sqrt();
        let t = (mean1 - mean2) / standard_error;
        // Welch-Satterthwaite equation for degrees of freedom
        let numerator = (var1/n1 + var2/n2).powi(2);
        let denominator = (var1/n1).powi(2)/(n1-1.0) + (var2/n2).powi(2)/(n2-1.0);
        let degrees_of_freedom = numerator / denominator;
        (t, degrees_of_freedom)
    };
    
    // t 분포 생성
    let t_dist = StudentsT::new(0.0, 1.0, df).unwrap();
    
    // p-value 계산
    let p_value = match test_type {
        "1" => 2.0 * (1.0 - t_dist.cdf(t_statistic.abs())), // 양측검정
        "2" => 1.0 - t_dist.cdf(t_statistic), // 우측검정
        "3" => t_dist.cdf(t_statistic), // 좌측검정
        _ => return Err(anyhow::anyhow!("잘못된 검정 유형입니다.")),
    };
    
    // 임계값 계산
    let critical_value = match test_type {
        "1" => t_dist.inverse_cdf(1.0 - alpha / 2.0), // 양측검정
        "2" => t_dist.inverse_cdf(1.0 - alpha), // 우측검정
        "3" => t_dist.inverse_cdf(alpha), // 좌측검정
        _ => 0.0,
    };
    
    // 결과 출력
    println!("\n=== 독립표본 t 검정 결과 ===");
    println!("그룹 1 ({}): n₁ = {}, x̄₁ = {:.4}, s₁ = {:.4}", group1_name, n1 as usize, mean1, std1);
    println!("그룹 2 ({}): n₂ = {}, x̄₂ = {:.4}, s₂ = {:.4}", group2_name, n2 as usize, mean2, std2);
    println!("평균 차이 (x̄₁ - x̄₂): {:.4}", mean1 - mean2);
    println!("등분산 가정: {}", if equal_variance { "예" } else { "아니오" });
    println!();
    
    println!("가설:");
    match test_type {
        "1" => {
            println!("H₀: μ₁ = μ₂");
            println!("H₁: μ₁ ≠ μ₂");
        }
        "2" => {
            println!("H₀: μ₁ ≤ μ₂");
            println!("H₁: μ₁ > μ₂");
        }
        "3" => {
            println!("H₀: μ₁ ≥ μ₂");
            println!("H₁: μ₁ < μ₂");
        }
        _ => {}
    }
    
    println!();
    println!("검정 통계량:");
    println!("t = {:.4}", t_statistic);
    println!("자유도 (df) = {:.2}", df);
    println!("p-value = {:.6}", p_value);
    println!("유의수준 (α) = {:.3}", alpha);
    
    match test_type {
        "1" => println!("임계값 = ±{:.4}", critical_value),
        "2" => println!("임계값 = {:.4}", critical_value),
        "3" => println!("임계값 = {:.4}", critical_value),
        _ => {}
    }
    
    println!();
    println!("결론:");
    if p_value < alpha {
        println!("p-value ({:.6}) < α ({:.3})", p_value, alpha);
        println!("귀무가설을 기각합니다.");
        match test_type {
            "1" => println!("두 그룹의 평균이 유의하게 다릅니다."),
            "2" => println!("그룹 1의 평균이 그룹 2보다 유의하게 큽니다."),
            "3" => println!("그룹 1의 평균이 그룹 2보다 유의하게 작습니다."),
            _ => {}
        }
    } else {
        println!("p-value ({:.6}) ≥ α ({:.3})", p_value, alpha);
        println!("귀무가설을 기각하지 않습니다.");
        match test_type {
            "1" => println!("두 그룹의 평균이 유의한 차이가 없습니다."),
            "2" => println!("그룹 1의 평균이 그룹 2보다 유의하게 크다고 할 수 없습니다."),
            "3" => println!("그룹 1의 평균이 그룹 2보다 유의하게 작다고 할 수 없습니다."),
            _ => {}
        }
    }
    
    Ok(())
}