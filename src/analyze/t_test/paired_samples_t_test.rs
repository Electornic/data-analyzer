use crate::read_file::DataSet;
use crate::analyze::select_columns;
use anyhow::Result;
use statrs::statistics::{Statistics, OrderStatistics};
use statrs::distribution::{StudentsT, ContinuousCDF};
use std::io::{self, Write};

pub fn perform_paired_samples_t_test(dataset: &DataSet) -> Result<()> {
    println!("\n=== 대응표본 t 검정 ===");

    // 숫자형 열만 필터링
    let numeric_headers: Vec<String> = dataset.headers.iter()
        .filter(|header| dataset.get_numeric_column(header).is_ok())
        .cloned()
        .collect();

    if numeric_headers.len() < 2 {
        println!("대응표본 t 검정을 위해서는 최소 2개의 숫자형 열이 필요합니다.");
        return Ok(());
    }

    println!("두 개의 대응된 측정값을 비교합니다 (예: 사전-사후, 처리 전-후).");

    // 첫 번째 측정값 선택
    let group1_columns = select_columns(&numeric_headers, "첫 번째 측정값의 열을 선택하세요:", false)?;
    let group1_column = &group1_columns[0];

    // 두 번째 측정값 선택 (첫 번째와 다른 열)
    let remaining_headers: Vec<String> = numeric_headers.iter()
        .filter(|header| *header != group1_column)
        .cloned()
        .collect();

    if remaining_headers.is_empty() {
        println!("두 번째 측정값으로 선택할 수 있는 열이 없습니다.");
        return Ok(());
    }

    let group2_columns = select_columns(&remaining_headers, "두 번째 측정값의 열을 선택하세요:", false)?;
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
    println!("1. μd ≠ 0 (양측검정) - 차이가 있다");
    println!("2. μd > 0 (우측검정) - 첫 번째가 두 번째보다 크다");
    println!("3. μd < 0 (좌측검정) - 첫 번째가 두 번째보다 작다");
    print!("선택 (1-3): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let test_type = input.trim();

    // 데이터 추출 및 검정 수행
    match (dataset.get_numeric_column(group1_column), dataset.get_numeric_column(group2_column)) {
        (Ok(data1), Ok(data2)) => {
            perform_paired_t_test(&data1, &data2, alpha, test_type, group1_column, group2_column)?;
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

fn perform_paired_t_test(
    data1: &[f64], 
    data2: &[f64], 
    alpha: f64, 
    test_type: &str,
    group1_name: &str,
    group2_name: &str
) -> Result<()> {
    if data1.len() != data2.len() {
        return Err(anyhow::anyhow!("대응표본 t 검정을 위해서는 두 그룹의 데이터 개수가 같아야 합니다."));
    }

    if data1.len() < 2 {
        return Err(anyhow::anyhow!("대응표본 t 검정을 위해서는 최소 2쌍의 데이터가 필요합니다."));
    }

    // 차이값 계산
    let differences: Vec<f64> = data1.iter().zip(data2.iter())
        .map(|(&x1, &x2)| x1 - x2)
        .collect();

    // 기본 통계량 계산
    let n = differences.len() as f64;
    let mean1 = data1.mean();
    let mean2 = data2.mean();
    let std1 = data1.std_dev();
    let std2 = data2.std_dev();

    // 차이값의 통계량
    let mean_diff = differences.as_slice().mean();
    let std_diff = differences.as_slice().std_dev();
    let standard_error = std_diff / n.sqrt();

    // t 통계량 계산 (귀무가설: μd = 0)
    let t_statistic = mean_diff / standard_error;
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

    // 상관계수 계산 (참고용)
    let correlation = calculate_correlation(data1, data2);

    // 결과 출력
    println!("\n=== 대응표본 t 검정 결과 ===");
    println!("측정 1 ({}): n = {}, x̄₁ = {:.4}, s₁ = {:.4}", group1_name, n as usize, mean1, std1);
    println!("측정 2 ({}): n = {}, x̄₂ = {:.4}, s₂ = {:.4}", group2_name, n as usize, mean2, std2);
    println!("상관계수 (r): {:.4}", correlation);
    println!();

    println!("차이값 통계량:");
    println!("평균 차이 (d̄): {:.4}", mean_diff);
    println!("차이의 표준편차 (sd): {:.4}", std_diff);
    println!("표준오차 (SE): {:.4}", standard_error);
    println!();

    println!("가설:");
    match test_type {
        "1" => {
            println!("H₀: μd = 0 (차이가 없다)");
            println!("H₁: μd ≠ 0 (차이가 있다)");
        }
        "2" => {
            println!("H₀: μd ≤ 0");
            println!("H₁: μd > 0 (첫 번째가 두 번째보다 크다)");
        }
        "3" => {
            println!("H₀: μd ≥ 0");
            println!("H₁: μd < 0 (첫 번째가 두 번째보다 작다)");
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
            "1" => println!("두 측정값 사이에 유의한 차이가 있습니다."),
            "2" => println!("첫 번째 측정값이 두 번째보다 유의하게 큽니다."),
            "3" => println!("첫 번째 측정값이 두 번째보다 유의하게 작습니다."),
            _ => {}
        }
    } else {
        println!("p-value ({:.6}) ≥ α ({:.3})", p_value, alpha);
        println!("귀무가설을 기각하지 않습니다.");
        match test_type {
            "1" => println!("두 측정값 사이에 유의한 차이가 없습니다."),
            "2" => println!("첫 번째 측정값이 두 번째보다 유의하게 크다고 할 수 없습니다."),
            "3" => println!("첫 번째 측정값이 두 번째보다 유의하게 작다고 할 수 없습니다."),
            _ => {}
        }
    }

    // 차이값의 기본 정보 출력
    println!("\n차이값 분포 정보:");
    let mut sorted_diffs = differences.clone();
    sorted_diffs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let min_diff = sorted_diffs[0];
    let max_diff = sorted_diffs[sorted_diffs.len() - 1];
    let median_diff = if sorted_diffs.len() % 2 == 0 {
        (sorted_diffs[sorted_diffs.len() / 2 - 1] + sorted_diffs[sorted_diffs.len() / 2]) / 2.0
    } else {
        sorted_diffs[sorted_diffs.len() / 2]
    };

    println!("최솟값: {:.4}", min_diff);
    println!("최댓값: {:.4}", max_diff);
    println!("중앙값: {:.4}", median_diff);

    Ok(())
}

fn calculate_correlation(data1: &[f64], data2: &[f64]) -> f64 {
    if data1.len() != data2.len() || data1.is_empty() {
        return 0.0;
    }

    let mean1 = data1.mean();
    let mean2 = data2.mean();

    let numerator: f64 = data1.iter().zip(data2.iter())
        .map(|(&x1, &x2)| (x1 - mean1) * (x2 - mean2))
        .sum();

    let sum_sq1: f64 = data1.iter().map(|&x| (x - mean1).powi(2)).sum();
    let sum_sq2: f64 = data2.iter().map(|&x| (x - mean2).powi(2)).sum();

    let denominator = (sum_sq1 * sum_sq2).sqrt();

    if denominator == 0.0 {
        0.0
    } else {
        numerator / denominator
    }
}
