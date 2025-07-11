use crate::read_file::DataSet;
use crate::analyze::select_columns;
use anyhow::Result;
use std::collections::BTreeMap;

pub fn create_stem_and_leaf(dataset: &DataSet) -> Result<()> {
    println!("\n=== 줄기와 잎 그림 생성 ===");
    
    // 숫자형 열만 필터링
    let numeric_headers: Vec<String> = dataset.headers.iter()
        .filter(|header| dataset.get_numeric_column(header).is_ok())
        .cloned()
        .collect();
    
    if numeric_headers.is_empty() {
        println!("숫자형 데이터가 있는 열이 없습니다.");
        return Ok(());
    }
    
    let selected_columns = select_columns(&numeric_headers, "줄기와 잎 그림을 생성할 열을 선택하세요:", false)?;
    let column = &selected_columns[0];
    
    match dataset.get_numeric_column(column) {
        Ok(numeric_data) => {
            println!("\n=== {} 열의 줄기와 잎 그림 ===", column);
            display_stem_and_leaf(&numeric_data)?;
        }
        Err(e) => {
            println!("'{}' 열의 숫자 데이터 추출 실패: {}", column, e);
        }
    }
    
    Ok(())
}

fn display_stem_and_leaf(data: &[f64]) -> Result<()> {
    if data.is_empty() {
        println!("데이터가 없습니다.");
        return Ok(());
    }
    
    // 데이터를 정수로 변환 (소수점 첫째 자리까지 고려)
    let mut int_data: Vec<i32> = data.iter()
        .map(|&x| (x * 10.0).round() as i32)
        .collect();
    
    int_data.sort();
    
    // 줄기와 잎으로 분리
    let mut stem_leaf_map: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
    
    for &value in &int_data {
        let stem = value / 10;
        let leaf = value % 10;
        stem_leaf_map.entry(stem).or_insert_with(Vec::new).push(leaf.abs());
    }
    
    // 결과 출력
    println!("줄기 | 잎");
    println!("-----|----");
    
    for (stem, mut leaves) in stem_leaf_map {
        leaves.sort();
        let leaves_str: String = leaves.iter()
            .map(|&leaf| leaf.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        
        println!("{:4} | {}", stem, leaves_str);
    }
    
    println!("\n범례: 줄기|잎 = 실제값");
    println!("예: 2|3 = 2.3");
    
    // 기본 통계 정보
    let min_val = data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max_val = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let count = data.len();
    
    println!("\n기본 정보:");
    println!("데이터 개수: {}", count);
    println!("최솟값: {:.1}", min_val);
    println!("최댓값: {:.1}", max_val);
    
    Ok(())
}