use crate::read_file::DataSet;
use crate::data_analyzer::DataAnalyzer;
use crate::analyze::select_columns;
use anyhow::Result;
use std::io::{self, Write};

pub fn show_frequency_distribution(dataset: &DataSet) -> Result<()> {
    println!("\n=== 빈도 분석 ===");
    
    let selected_columns = select_columns(&dataset.headers, "빈도 분석을 수행할 열을 선택하세요:", true)?;
    
    let analyzer = DataAnalyzer::new();
    
    for column in selected_columns {
        println!("\n=== {} 열의 빈도 분석 ===", column);
        
        // 먼저 숫자형 데이터인지 확인
        if let Ok(numeric_data) = dataset.get_numeric_column(&column) {
            // 숫자형 데이터의 경우 구간별 빈도 분석
            show_numeric_frequency(&numeric_data, &column)?;
        } else {
            // 문자열 데이터의 경우 카테고리별 빈도 분석
            match analyzer.analyze_column_frequency(dataset, &column) {
                Ok(freq_data) => {
                    analyzer.print_frequency_data(&freq_data, &column);
                }
                Err(e) => {
                    println!("'{}' 열 빈도 분석 중 오류 발생: {}", column, e);
                }
            }
        }
    }
    
    Ok(())
}

fn show_numeric_frequency(data: &[f64], column_name: &str) -> Result<()> {
    if data.is_empty() {
        println!("데이터가 없습니다.");
        return Ok(());
    }
    
    let min_val = data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max_val = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    
    // 구간 수 설정 (기본 10개)
    let bins = 10;
    let bin_width = (max_val - min_val) / bins as f64;
    
    if bin_width == 0.0 {
        println!("모든 값이 동일합니다: {}", min_val);
        println!("빈도: {}", data.len());
        return Ok(());
    }
    
    let mut frequency = vec![0; bins];
    let mut bin_ranges = Vec::new();
    
    // 구간 범위 생성
    for i in 0..bins {
        let start = min_val + i as f64 * bin_width;
        let end = if i == bins - 1 { max_val } else { start + bin_width };
        bin_ranges.push((start, end));
    }
    
    // 각 데이터를 해당 구간에 배치
    for &value in data {
        let mut bin_index = ((value - min_val) / bin_width) as usize;
        if bin_index >= bins {
            bin_index = bins - 1;
        }
        frequency[bin_index] += 1;
    }
    
    // 결과 출력
    println!("구간별 빈도 분포:");
    println!("{:<20} {:<10} {:<10}", "구간", "빈도", "상대빈도");
    println!("{}", "-".repeat(40));
    
    let total = data.len() as f64;
    for (i, &freq) in frequency.iter().enumerate() {
        let (start, end) = bin_ranges[i];
        let relative_freq = freq as f64 / total;
        println!("{:<20} {:<10} {:<10.4}", 
                format!("[{:.2}, {:.2})", start, end), 
                freq, 
                relative_freq);
    }
    
    Ok(())
}