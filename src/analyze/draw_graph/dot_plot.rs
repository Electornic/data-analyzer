use crate::read_file::DataSet;
use crate::analyze::select_columns;
use anyhow::Result;
use plotters::prelude::*;
use std::collections::HashMap;

pub fn create_dot_plot(dataset: &DataSet) -> Result<()> {
    println!("\n=== 점도표 생성 ===");

    // 숫자형 열만 필터링
    let numeric_headers: Vec<String> = dataset.headers.iter()
        .filter(|header| dataset.get_numeric_column(header).is_ok())
        .cloned()
        .collect();

    if numeric_headers.is_empty() {
        println!("숫자형 데이터가 있는 열이 없습니다.");
        return Ok(());
    }

    let selected_columns = select_columns(&numeric_headers, "점도표를 생성할 열을 선택하세요:", false)?;
    let column = &selected_columns[0];

    match dataset.get_numeric_column(column) {
        Ok(numeric_data) => {
            let safe_column = column.replace(" ", "_").replace("/", "_");
            let output_path = format!("result/dot_plot_{}.png", safe_column);

            create_dot_plot_chart(&numeric_data, column, &output_path)?;
            println!("점도표가 '{}'에 저장되었습니다.", output_path);
        }
        Err(e) => {
            println!("'{}' 열의 숫자 데이터 추출 실패: {}", column, e);
        }
    }

    Ok(())
}

fn create_dot_plot_chart(data: &[f64], column_name: &str, output_path: &str) -> Result<()> {
    if data.is_empty() {
        return Err(anyhow::anyhow!("데이터가 없습니다."));
    }

    // 데이터를 반올림하여 빈도 계산
    let mut frequency_map: HashMap<i32, i32> = HashMap::new();
    for &value in data {
        let rounded_value = value.round() as i32;
        *frequency_map.entry(rounded_value).or_insert(0) += 1;
    }

    // 데이터 범위 계산
    let min_val = data.iter().fold(f64::INFINITY, |a, &b| a.min(b)) as i32;
    let max_val = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)) as i32;
    let max_freq = frequency_map.values().max().unwrap_or(&1);

    // 그래프 생성
    let root = BitMapBackend::new(output_path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(&format!("점도표 - {}", column_name), ("sans-serif", 40))
        .margin(20)
        .x_label_area_size(60)
        .y_label_area_size(60)
        .build_cartesian_2d(
            (min_val - 1)..(max_val + 1),
            0..(*max_freq + 1)
        )?;

    chart
        .configure_mesh()
        .x_desc("값")
        .y_desc("빈도")
        .draw()?;

    // 점들을 그리기
    for (value, &freq) in &frequency_map {
        for i in 1..=freq {
            chart.draw_series(std::iter::once(Circle::new((*value, i), 3, BLUE.filled())))?;
        }
    }

    root.present()?;

    // 콘솔에도 간단한 점도표 출력
    println!("\n=== {} 열의 점도표 (콘솔 버전) ===", column_name);

    let mut sorted_values: Vec<(&i32, &i32)> = frequency_map.iter().collect();
    sorted_values.sort_by_key(|&(value, _)| value);

    for (value, &freq) in sorted_values {
        let dots = "•".repeat(freq as usize);
        println!("{:4}: {}", value, dots);
    }

    Ok(())
}
