use crate::read_file::DataSet;
use crate::data_analyzer::DataAnalyzer;
use crate::analyze::select_columns;
use anyhow::Result;
use std::io::{self, Write};

pub fn create_histogram(dataset: &DataSet) -> Result<()> {
    println!("\n=== 히스토그램 생성 ===");

    // 숫자형 열만 필터링
    let numeric_headers: Vec<String> = dataset.headers.iter()
        .filter(|header| dataset.get_numeric_column(header).is_ok())
        .cloned()
        .collect();

    if numeric_headers.is_empty() {
        println!("숫자형 데이터가 있는 열이 없습니다.");
        return Ok(());
    }

    let selected_columns = select_columns(&numeric_headers, "히스토그램을 생성할 열을 선택하세요:", true)?;

    // 구간 수 입력받기
    println!("히스토그램의 구간 수를 입력하세요 (기본값: 20):");
    print!("구간 수: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let bins = if input.trim().is_empty() {
        20
    } else {
        input.trim().parse().unwrap_or(20)
    };

    let analyzer = DataAnalyzer::new();

    for column in selected_columns {
        println!("\n'{}' 열의 히스토그램을 생성 중...", column);

        match dataset.get_numeric_column(&column) {
            Ok(numeric_data) => {
                let safe_column = column.replace(" ", "_").replace("/", "_");
                let output_path = format!("result/histogram_{}.png", safe_column);

                match analyzer.create_histogram(&numeric_data, &format!("히스토그램 - {}", column), &output_path, bins) {
                    Ok(_) => {
                        println!("히스토그램이 '{}'에 저장되었습니다.", output_path);
                    }
                    Err(e) => {
                        println!("'{}' 열의 히스토그램 생성 실패: {}", column, e);
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
