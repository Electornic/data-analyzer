use crate::read_file::DataSet;
use crate::data_analyzer::DataAnalyzer;
use crate::analyze::select_columns;
use anyhow::Result;
use plotters::prelude::*;
use std::collections::HashMap;

pub fn create_bar_chart(dataset: &DataSet) -> Result<()> {
    println!("\n=== 막대 그래프 생성 ===");

    let selected_columns = select_columns(&dataset.headers, "막대 그래프를 생성할 열을 선택하세요:", false)?;
    let column = &selected_columns[0];

    let analyzer = DataAnalyzer::new();

    // 빈도 데이터 계산
    let freq_data = analyzer.analyze_column_frequency(dataset, column)?;
    let frequencies = freq_data.get_relative_frequencies();

    if frequencies.is_empty() {
        println!("데이터가 없습니다.");
        return Ok(());
    }

    // 파일명 생성
    let safe_column = column.replace(" ", "_").replace("/", "_");
    let output_path = format!("bar_chart_{}.png", safe_column);

    // 그래프 생성
    let root = BitMapBackend::new(&output_path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(&format!("막대 그래프 - {}", column), ("sans-serif", 40))
        .margin(20)
        .x_label_area_size(60)
        .y_label_area_size(60)
        .build_cartesian_2d(
            0f32..frequencies.len() as f32,
            0f64..frequencies.values().fold(0.0f64, |a, &b| a.max(b)) * 1.1
        )?;

    chart
        .configure_mesh()
        .x_desc("카테고리")
        .y_desc("상대 빈도")
        .draw()?;

    // 데이터를 벡터로 변환하고 정렬
    let mut freq_vec: Vec<(String, f64)> = frequencies.iter()
        .map(|(k, v)| (k.clone(), *v))
        .collect();
    freq_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // 막대 그래프 그리기
    chart.draw_series(
        freq_vec.iter().enumerate().map(|(i, (_, freq))| {
            Rectangle::new([(i as f32, 0.0), (i as f32 + 0.8, *freq)], BLUE.filled())
        })
    )?
    .label("빈도")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &BLUE));

    // X축 레이블은 plotters의 lifetime 제약으로 인해 생략
    // 대신 콘솔에 카테고리 정보 출력
    println!("\n카테고리별 빈도:");
    for (i, (category, freq)) in freq_vec.iter().enumerate() {
        println!("{}: {} ({:.1}%)", i + 1, category, freq * 100.0);
    }

    chart.configure_series_labels().draw()?;
    root.present()?;

    println!("막대 그래프가 '{}'에 저장되었습니다.", output_path);

    Ok(())
}
