use crate::read_file::DataSet;
use crate::data_analyzer::DataAnalyzer;
use crate::analyze::select_columns;
use anyhow::Result;
use plotters::prelude::*;
use std::f64::consts::PI;

pub fn create_pie_chart(dataset: &DataSet) -> Result<()> {
    println!("\n=== 원 그래프 생성 ===");

    let selected_columns = select_columns(&dataset.headers, "원 그래프를 생성할 열을 선택하세요:", false)?;
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
    let output_path = format!("result/pie_chart_{}.png", safe_column);

    // 그래프 생성
    let root = BitMapBackend::new(&output_path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(&format!("원 그래프 - {}", column), ("sans-serif", 40))
        .margin(20)
        .build_cartesian_2d(-1.2f64..1.2f64, -1.2f64..1.2f64)?;

    // 데이터를 벡터로 변환하고 정렬
    let mut freq_vec: Vec<(String, f64)> = frequencies.iter()
        .map(|(k, v)| (k.clone(), *v))
        .collect();
    freq_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // 색상 팔레트
    let colors = [
        &BLUE, &RED, &GREEN, &MAGENTA, &CYAN, 
        &RGBColor(255, 165, 0), // Orange
        &RGBColor(128, 0, 128), // Purple
        &RGBColor(255, 192, 203), // Pink
        &RGBColor(165, 42, 42), // Brown
        &RGBColor(128, 128, 128), // Gray
    ];

    let mut start_angle = 0.0;
    let center = (0.0, 0.0);
    let radius = 0.8;

    // 원 그래프 그리기
    for (i, (category, freq)) in freq_vec.iter().enumerate() {
        let angle = freq * 2.0 * PI;
        let end_angle = start_angle + angle;

        let color = colors[i % colors.len()];

        // 부채꼴 그리기
        let mut points = vec![center];
        let steps = 50;
        for j in 0..=steps {
            let current_angle = start_angle + (angle * j as f64 / steps as f64);
            let x = center.0 + radius * current_angle.cos();
            let y = center.1 + radius * current_angle.sin();
            points.push((x, y));
        }

        chart.draw_series(std::iter::once(Polygon::new(points, color.filled())))?;

        // 레이블 위치 계산
        let label_angle = start_angle + angle / 2.0;
        let label_x = center.0 + (radius + 0.2) * label_angle.cos();
        let label_y = center.1 + (radius + 0.2) * label_angle.sin();

        // 레이블 그리기
        chart.draw_series(std::iter::once(Text::new(
            format!("{}\n({:.1}%)", category, freq * 100.0),
            (label_x, label_y),
            ("sans-serif", 12).into_font().color(color)
        )))?;

        start_angle = end_angle;
    }

    root.present()?;

    println!("원 그래프가 '{}'에 저장되었습니다.", output_path);

    Ok(())
}
