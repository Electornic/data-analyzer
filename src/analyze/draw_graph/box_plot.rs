use crate::read_file::DataSet;
use crate::data_analyzer::DataAnalyzer;
use crate::analyze::select_columns;
use anyhow::Result;

pub fn create_box_plot(dataset: &DataSet) -> Result<()> {
    println!("\n=== 상자 그림 생성 ===");
    
    // 숫자형 열만 필터링
    let numeric_headers: Vec<String> = dataset.headers.iter()
        .filter(|header| dataset.get_numeric_column(header).is_ok())
        .cloned()
        .collect();
    
    if numeric_headers.is_empty() {
        println!("숫자형 데이터가 있는 열이 없습니다.");
        return Ok(());
    }
    
    let selected_columns = select_columns(&numeric_headers, "상자 그림을 생성할 열을 선택하세요:", true)?;
    
    let analyzer = DataAnalyzer::new();
    
    for column in selected_columns {
        println!("\n'{}' 열의 상자 그림을 생성 중...", column);
        
        match dataset.get_numeric_column(&column) {
            Ok(numeric_data) => {
                let safe_column = column.replace(" ", "_").replace("/", "_");
                let output_path = format!("boxplot_{}.png", safe_column);
                
                match analyzer.create_box_plot(&numeric_data, &format!("상자 그림 - {}", column), &output_path) {
                    Ok(_) => {
                        println!("상자 그림이 '{}'에 저장되었습니다.", output_path);
                    }
                    Err(e) => {
                        println!("'{}' 열의 상자 그림 생성 실패: {}", column, e);
                    }
                }
            }
            Err(e) => {
                println!("'{}' 열의 숫자 데이터 추출 실패: {}", column, e);
            }
        }
    }
    
    Ok(())
}