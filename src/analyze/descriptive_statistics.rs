use crate::read_file::DataSet;
use crate::data_analyzer::DataAnalyzer;
use crate::analyze::select_columns;
use anyhow::Result;
use std::io::{self, Write};

pub fn show_descriptive_statistics(dataset: &DataSet) -> Result<()> {
    println!("\n=== 기초 통계량 표시 ===");
    
    // 숫자형 열만 필터링
    let numeric_headers: Vec<String> = dataset.headers.iter()
        .filter(|header| dataset.get_numeric_column(header).is_ok())
        .cloned()
        .collect();
    
    if numeric_headers.is_empty() {
        println!("숫자형 데이터가 있는 열이 없습니다.");
        return Ok(());
    }
    
    let selected_columns = select_columns(&numeric_headers, "기초 통계량을 계산할 열을 선택하세요:", true)?;
    
    let analyzer = DataAnalyzer::new();
    
    for column in selected_columns {
        println!("\n=== {} 열의 기초 통계량 ===", column);
        
        match analyzer.analyze_column(dataset, &column) {
            Ok(stats) => {
                analyzer.print_basic_stats(&stats, &column);
            }
            Err(e) => {
                println!("'{}' 열 분석 중 오류 발생: {}", column, e);
            }
        }
    }
    
    Ok(())
}