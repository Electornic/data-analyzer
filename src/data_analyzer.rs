use crate::read_file::DataSet;
use anyhow::{Result, anyhow};
use std::collections::HashMap;
use statrs::statistics::{Statistics, OrderStatistics};
use plotters::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Clone)]
pub struct BasicStats {
    pub mean: f64,
    pub median: f64,
    pub std_dev: f64,
    pub variance: f64,
    pub min: f64,
    pub max: f64,
    pub count: usize,
    pub q1: f64,
    pub q3: f64,
}

#[derive(Debug, Clone)]
pub struct FrequencyData {
    pub frequencies: HashMap<String, usize>,
    pub total_count: usize,
}

impl FrequencyData {
    pub fn get_relative_frequencies(&self) -> HashMap<String, f64> {
        self.frequencies
            .iter()
            .map(|(k, &v)| (k.clone(), v as f64 / self.total_count as f64))
            .collect()
    }
}

// Helper functions for statistical calculations
fn calculate_median(sorted_data: &[f64]) -> f64 {
    let len = sorted_data.len();
    if len % 2 == 0 {
        (sorted_data[len / 2 - 1] + sorted_data[len / 2]) / 2.0
    } else {
        sorted_data[len / 2]
    }
}

fn calculate_quartile(sorted_data: &[f64], percentile: f64) -> f64 {
    let len = sorted_data.len();
    let index = percentile * (len - 1) as f64;
    let lower = index.floor() as usize;
    let upper = index.ceil() as usize;

    if lower == upper {
        sorted_data[lower]
    } else {
        let weight = index - lower as f64;
        sorted_data[lower] * (1.0 - weight) + sorted_data[upper] * weight
    }
}

pub struct DataAnalyzer;

impl DataAnalyzer {
    pub fn new() -> Self {
        Self
    }

    /// 기초통계량 계산 (평균, 중앙값, 표준편차 등)
    pub fn calculate_basic_stats(&self, data: &[f64]) -> Result<BasicStats> {
        if data.is_empty() {
            return Err(anyhow!("Cannot calculate statistics for empty data"));
        }

        let mut sorted_data = data.to_vec();
        sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mean = data.mean();
        let median = calculate_median(&sorted_data);
        let std_dev = data.std_dev();
        let variance = data.variance();
        let min = *sorted_data.first().unwrap();
        let max = *sorted_data.last().unwrap();
        let count = data.len();

        // Calculate quartiles
        let q1 = calculate_quartile(&sorted_data, 0.25);
        let q3 = calculate_quartile(&sorted_data, 0.75);

        Ok(BasicStats {
            mean,
            median,
            std_dev,
            variance,
            min,
            max,
            count,
            q1,
            q3,
        })
    }

    /// 빈도 분석
    pub fn calculate_frequency(&self, data: &[String]) -> FrequencyData {
        let mut frequencies = HashMap::new();

        for item in data {
            *frequencies.entry(item.clone()).or_insert(0) += 1;
        }

        FrequencyData {
            frequencies,
            total_count: data.len(),
        }
    }

    /// 특정 열의 기초통계량 계산
    pub fn analyze_column(&self, dataset: &DataSet, column_name: &str) -> Result<BasicStats> {
        let numeric_data = dataset.get_numeric_column(column_name)?;
        self.calculate_basic_stats(&numeric_data)
    }

    /// 특정 열의 빈도 분석
    pub fn analyze_column_frequency(&self, dataset: &DataSet, column_name: &str) -> Result<FrequencyData> {
        let column_data = dataset.get_column(column_name)?;
        Ok(self.calculate_frequency(&column_data))
    }

    /// Box Plot 그리기
    pub fn create_box_plot(&self, data: &[f64], title: &str, output_path: &str) -> Result<()> {
        if data.is_empty() {
            return Err(anyhow!("Cannot create box plot for empty data"));
        }

        let mut sorted_data = data.to_vec();
        sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let stats = self.calculate_basic_stats(data)?;

        let root = BitMapBackend::new(output_path, (800, 600)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption(title, ("sans-serif", 40))
            .margin(20)
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d(0f64..2f64, stats.min..stats.max)?;

        chart.configure_mesh().draw()?;

        // Draw box plot components
        let box_x = 1.0;
        let box_width = 0.3;

        // Draw box (Q1 to Q3)
        chart.draw_series(std::iter::once(Rectangle::new(
            [(box_x - box_width/2.0, stats.q1), (box_x + box_width/2.0, stats.q3)],
            BLUE.filled(),
        )))?;

        // Draw median line
        chart.draw_series(std::iter::once(PathElement::new(
            vec![(box_x - box_width/2.0, stats.median), (box_x + box_width/2.0, stats.median)],
            &RED,
        )))?;

        // Draw whiskers
        chart.draw_series(std::iter::once(PathElement::new(
            vec![(box_x, stats.min), (box_x, stats.q1)],
            &BLACK,
        )))?;
        chart.draw_series(std::iter::once(PathElement::new(
            vec![(box_x, stats.q3), (box_x, stats.max)],
            &BLACK,
        )))?;

        // Draw whisker caps
        chart.draw_series(std::iter::once(PathElement::new(
            vec![(box_x - 0.1, stats.min), (box_x + 0.1, stats.min)],
            &BLACK,
        )))?;
        chart.draw_series(std::iter::once(PathElement::new(
            vec![(box_x - 0.1, stats.max), (box_x + 0.1, stats.max)],
            &BLACK,
        )))?;

        root.present()?;
        println!("Box plot saved to: {}", output_path);
        Ok(())
    }

    /// QQ Plot 그리기 (Normal Q-Q Plot)
    pub fn create_qq_plot(&self, data: &[f64], title: &str, output_path: &str) -> Result<()> {
        if data.is_empty() {
            return Err(anyhow!("Cannot create QQ plot for empty data"));
        }

        let mut sorted_data = data.to_vec();
        sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let n = sorted_data.len();
        let mut theoretical_quantiles = Vec::new();

        // Calculate theoretical quantiles for normal distribution
        for i in 1..=n {
            let p = (i as f64 - 0.5) / n as f64;
            // Approximate inverse normal CDF using Box-Muller transformation
            let z = self.inverse_normal_cdf(p);
            theoretical_quantiles.push(z);
        }

        let root = BitMapBackend::new(output_path, (800, 600)).into_drawing_area();
        root.fill(&WHITE)?;

        let x_min = theoretical_quantiles.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let x_max = theoretical_quantiles.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let y_min = sorted_data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let y_max = sorted_data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

        let x_range = x_min..x_max;
        let y_range = y_min..y_max;

        let mut chart = ChartBuilder::on(&root)
            .caption(title, ("sans-serif", 40))
            .margin(20)
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d(x_range, y_range)?;

        chart.configure_mesh()
            .x_desc("Theoretical Quantiles")
            .y_desc("Sample Quantiles")
            .draw()?;

        // Plot points
        let points: Vec<(f64, f64)> = theoretical_quantiles
            .iter()
            .zip(sorted_data.iter())
            .map(|(&x, &y)| (x, y))
            .collect();

        chart.draw_series(
            points.iter().map(|&point| Circle::new(point, 3, BLUE.filled()))
        )?;

        // Draw reference line (y = x)
        let min_val = x_min.min(y_min);
        let max_val = x_max.max(y_max);
        chart.draw_series(std::iter::once(PathElement::new(
            vec![(min_val, min_val), (max_val, max_val)],
            &RED,
        )))?;

        root.present()?;
        println!("QQ plot saved to: {}", output_path);
        Ok(())
    }

    /// 히스토그램 그리기
    pub fn create_histogram(&self, data: &[f64], title: &str, output_path: &str, bins: usize) -> Result<()> {
        if data.is_empty() {
            return Err(anyhow!("Cannot create histogram for empty data"));
        }

        let min_val = data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_val = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let bin_width = (max_val - min_val) / bins as f64;

        let mut histogram = vec![0; bins];
        for &value in data {
            let bin_index = ((value - min_val) / bin_width).floor() as usize;
            let bin_index = bin_index.min(bins - 1);
            histogram[bin_index] += 1;
        }

        let max_count = *histogram.iter().max().unwrap();

        let root = BitMapBackend::new(output_path, (800, 600)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption(title, ("sans-serif", 40))
            .margin(20)
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d(min_val..max_val, 0..max_count)?;

        chart.configure_mesh().draw()?;

        chart.draw_series(
            histogram
                .iter()
                .enumerate()
                .map(|(i, &count)| {
                    let x0 = min_val + i as f64 * bin_width;
                    let x1 = x0 + bin_width;
                    Rectangle::new([(x0, 0), (x1, count)], BLUE.filled())
                })
        )?;

        root.present()?;
        println!("Histogram saved to: {}", output_path);
        Ok(())
    }

    /// 표본 추출
    pub fn random_sample(&self, dataset: &DataSet, sample_size: usize) -> Result<DataSet> {
        if sample_size > dataset.row_count() {
            return Err(anyhow!("Sample size cannot be larger than dataset size"));
        }

        let mut rng = thread_rng();
        let mut indices: Vec<usize> = (0..dataset.row_count()).collect();
        indices.shuffle(&mut rng);
        indices.truncate(sample_size);

        let mut sampled_data = Vec::new();
        for &index in &indices {
            if let Some(row) = dataset.get_row(index) {
                sampled_data.push(row.clone());
            }
        }

        Ok(DataSet::new(
            dataset.headers.clone(),
            sampled_data,
            format!("{}_sample_{}", dataset.file_path, sample_size),
        ))
    }

    /// 계층 표본 추출
    pub fn stratified_sample(&self, dataset: &DataSet, strata_column: &str, sample_size: usize) -> Result<DataSet> {
        let strata_data = dataset.get_column(strata_column)?;
        let mut strata_groups: HashMap<String, Vec<usize>> = HashMap::new();

        // Group rows by strata
        for (index, stratum) in strata_data.iter().enumerate() {
            strata_groups.entry(stratum.clone()).or_insert_with(Vec::new).push(index);
        }

        let total_strata = strata_groups.len();
        let samples_per_stratum = sample_size / total_strata;
        let mut rng = thread_rng();
        let mut sampled_indices = Vec::new();

        for (_, mut indices) in strata_groups {
            indices.shuffle(&mut rng);
            let take_count = samples_per_stratum.min(indices.len());
            sampled_indices.extend(indices.into_iter().take(take_count));
        }

        let mut sampled_data = Vec::new();
        for &index in &sampled_indices {
            if let Some(row) = dataset.get_row(index) {
                sampled_data.push(row.clone());
            }
        }

        Ok(DataSet::new(
            dataset.headers.clone(),
            sampled_data,
            format!("{}_stratified_sample_{}", dataset.file_path, sample_size),
        ))
    }

    // Helper function for inverse normal CDF approximation
    fn inverse_normal_cdf(&self, p: f64) -> f64 {
        // Beasley-Springer-Moro algorithm approximation
        let a0 = -3.969683028665376e+01;
        let a1 = 2.209460984245205e+02;
        let a2 = -2.759285104469687e+02;
        let a3 = 1.383577518672690e+02;
        let a4 = -3.066479806614716e+01;
        let a5 = 2.506628277459239e+00;

        let b1 = -5.447609879822406e+01;
        let b2 = 1.615858368580409e+02;
        let b3 = -1.556989798598866e+02;
        let b4 = 6.680131188771972e+01;
        let b5 = -1.328068155288572e+01;

        if p <= 0.5 {
            let t = (-2.0 * p.ln()).sqrt();
            (a0 + a1*t + a2*t*t + a3*t*t*t + a4*t*t*t*t + a5*t*t*t*t*t) /
            (1.0 + b1*t + b2*t*t + b3*t*t*t + b4*t*t*t*t + b5*t*t*t*t*t)
        } else {
            let t = (-2.0 * (1.0 - p).ln()).sqrt();
            -((a0 + a1*t + a2*t*t + a3*t*t*t + a4*t*t*t*t + a5*t*t*t*t*t) /
              (1.0 + b1*t + b2*t*t + b3*t*t*t + b4*t*t*t*t + b5*t*t*t*t*t))
        }
    }

    /// 데이터셋 요약 정보 출력
    pub fn print_dataset_summary(&self, dataset: &DataSet) {
        println!("=== Dataset Summary ===");
        println!("File: {}", dataset.file_path);
        println!("Rows: {}", dataset.row_count());
        println!("Columns: {}", dataset.column_count());
        println!("Headers: {:?}", dataset.headers);
        println!("========================");
    }

    /// 통계 결과 출력
    pub fn print_basic_stats(&self, stats: &BasicStats, column_name: &str) {
        println!("=== Basic Statistics for '{}' ===", column_name);
        println!("Count: {}", stats.count);
        println!("Mean: {:.4}", stats.mean);
        println!("Median: {:.4}", stats.median);
        println!("Standard Deviation: {:.4}", stats.std_dev);
        println!("Variance: {:.4}", stats.variance);
        println!("Minimum: {:.4}", stats.min);
        println!("Maximum: {:.4}", stats.max);
        println!("Q1 (25th percentile): {:.4}", stats.q1);
        println!("Q3 (75th percentile): {:.4}", stats.q3);
        println!("=====================================");
    }

    /// 빈도 결과 출력
    pub fn print_frequency_data(&self, freq_data: &FrequencyData, column_name: &str) {
        println!("=== Frequency Analysis for '{}' ===", column_name);
        println!("Total Count: {}", freq_data.total_count);

        let mut sorted_freq: Vec<_> = freq_data.frequencies.iter().collect();
        sorted_freq.sort_by(|a, b| b.1.cmp(a.1)); // Sort by frequency descending

        for (value, count) in sorted_freq.iter().take(10) { // Show top 10
            let percentage = (**count as f64 / freq_data.total_count as f64) * 100.0;
            println!("{}: {} ({:.2}%)", value, count, percentage);
        }

        if sorted_freq.len() > 10 {
            println!("... and {} more unique values", sorted_freq.len() - 10);
        }
        println!("=====================================");
    }
}
