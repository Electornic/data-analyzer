use crate::read_file::DataSet;
use crate::analyze::select_columns;
use anyhow::Result;
use statrs::statistics::{Statistics, OrderStatistics};
use statrs::distribution::{StudentsT, ContinuousCDF};
use std::io::{self, Write};

pub fn perform_one_sample_t_test(dataset: &DataSet) -> Result<()> {
    println!("\n=== 일표본 t 검정 ===");
    
    // 숫자형 열만 필터링
    let numeric_headers: Vec<String> = dataset.headers.iter()
        .filter(|header| dataset.get_numeric_column(header).is_ok())
        .cloned()
        .collect();
    
    if numeric_headers.is_empty() {
        println!("숫자형 데이터가 있는 열이 없습니다.");
        return Ok(());
    }
    
    let selected_columns = select_columns(&numeric_headers, "t 검정을 수행할 열을 선택하세요:", false)?;
    let column = &selected_columns[0];
    
    // 귀무가설의 모평균 입력받기
    println!("귀무가설의 모평균을 입력하세요:");
    print!("모평균 (μ₀): ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mu_0: f64 = input.trim().parse()
        .map_err(|_| anyhow::anyhow!("잘못된 숫자 형식입니다"))?;
    
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
    println!("1. μ ≠ {} (양측검정)", mu_0);
    println!("2. μ > {} (우측검정)", mu_0);
    println!("3. μ < {} (좌측검정)", mu_0);
    print!("선택 (1-3): ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let test_type = input.trim();
    
    // 데이터 추출 및 검정 수행
    match dataset.get_numeric_column(column) {
        Ok(data) => {
            perform_t_test(&data, mu_0, alpha, test_type, column)?;
        }
        Err(e) => {
            println!("'{}' 열의 숫자 데이터 추출 실패: {}", column, e);
        }
    }
    
    Ok(())
}

fn perform_t_test(data: &[f64], mu_0: f64, alpha: f64, test_type: &str, column_name: &str) -> Result<()> {
    if data.len() < 2 {
        return Err(anyhow::anyhow!("t 검정을 위해서는 최소 2개의 데이터가 필요합니다."));
    }
    
    // 기본 통계량 계산
    let n = data.len() as f64;
    let sample_mean = data.mean();
    let sample_std = data.std_dev();
    let standard_error = sample_std / n.sqrt();
    
    // t 통계량 계산
    let t_statistic = (sample_mean - mu_0) / standard_error;
    let df = n - 1.0;
    
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
    println!("\n=== {} 열의 일표본 t 검정 결과 ===", column_name);
    println!("표본 크기 (n): {}", n as usize);
    println!("표본 평균 (x̄): {:.4}", sample_mean);
    println!("표본 표준편차 (s): {:.4}", sample_std);
    println!("표준오차 (SE): {:.4}", standard_error);
    println!("귀무가설 모평균 (μ₀): {:.4}", mu_0);
    println!();
    
    println!("가설:");
    match test_type {
        "1" => {
            println!("H₀: μ = {}", mu_0);
            println!("H₁: μ ≠ {}", mu_0);
        }
        "2" => {
            println!("H₀: μ ≤ {}", mu_0);
            println!("H₁: μ > {}", mu_0);
        }
        "3" => {
            println!("H₀: μ ≥ {}", mu_0);
            println!("H₁: μ < {}", mu_0);
        }
        _ => {}
    }
    
    println!();
    println!("검정 통계량:");
    println!("t = {:.4}", t_statistic);
    println!("자유도 (df) = {:.0}", df);
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
            "1" => println!("표본 평균이 {}와 유의하게 다릅니다.", mu_0),
            "2" => println!("표본 평균이 {}보다 유의하게 큽니다.", mu_0),
            "3" => println!("표본 평균이 {}보다 유의하게 작습니다.", mu_0),
            _ => {}
        }
    } else {
        println!("p-value ({:.6}) ≥ α ({:.3})", p_value, alpha);
        println!("귀무가설을 기각하지 않습니다.");
        match test_type {
            "1" => println!("표본 평균이 {}와 유의한 차이가 없습니다.", mu_0),
            "2" => println!("표본 평균이 {}보다 유의하게 크다고 할 수 없습니다.", mu_0),
            "3" => println!("표본 평균이 {}보다 유의하게 작다고 할 수 없습니다.", mu_0),
            _ => {}
        }
    }
    
    Ok(())
}