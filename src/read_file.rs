use std::path::Path;
use anyhow::{Result, anyhow};
use csv::Reader;
use calamine::{Reader as ExcelReader, open_workbook, Xlsx, DataType};

#[derive(Debug, Clone)]
pub struct DataSet {
    pub headers: Vec<String>,
    pub data: Vec<Vec<String>>,
    pub file_path: String,
}

impl DataSet {
    pub fn new(headers: Vec<String>, data: Vec<Vec<String>>, file_path: String) -> Self {
        Self {
            headers,
            data,
            file_path,
        }
    }

    pub fn get_column(&self, column_name: &str) -> Result<Vec<String>> {
        let column_index = self.headers
            .iter()
            .position(|h| h == column_name)
            .ok_or_else(|| anyhow!("Column '{}' not found", column_name))?;

        Ok(self.data
            .iter()
            .map(|row| row.get(column_index).unwrap_or(&String::new()).clone())
            .collect())
    }

    pub fn get_numeric_column(&self, column_name: &str) -> Result<Vec<f64>> {
        let column_data = self.get_column(column_name)?;
        let mut numeric_data = Vec::new();

        for value in column_data {
            if let Ok(num) = value.parse::<f64>() {
                numeric_data.push(num);
            }
        }

        if numeric_data.is_empty() {
            return Err(anyhow!("No numeric data found in column '{}'", column_name));
        }

        Ok(numeric_data)
    }

    pub fn get_row(&self, index: usize) -> Option<&Vec<String>> {
        self.data.get(index)
    }

    pub fn row_count(&self) -> usize {
        self.data.len()
    }

    pub fn column_count(&self) -> usize {
        self.headers.len()
    }
}

pub fn read_csv_file<P: AsRef<Path>>(file_path: P) -> Result<DataSet> {
    let path = file_path.as_ref();
    let mut reader = Reader::from_path(path)?;

    // Read headers
    let headers = reader.headers()?
        .iter()
        .map(|h| h.to_string())
        .collect::<Vec<String>>();

    // Read data
    let mut data = Vec::new();
    for result in reader.records() {
        let record = result?;
        let row: Vec<String> = record.iter().map(|field| field.to_string()).collect();
        data.push(row);
    }

    Ok(DataSet::new(
        headers,
        data,
        path.to_string_lossy().to_string(),
    ))
}

pub fn read_excel_file<P: AsRef<Path>>(file_path: P) -> Result<DataSet> {
    let path = file_path.as_ref();
    let mut workbook: Xlsx<_> = open_workbook(path)?;

    // Get the first worksheet
    let worksheet_names = workbook.sheet_names();
    if worksheet_names.is_empty() {
        return Err(anyhow!("No worksheets found in Excel file"));
    }

    let worksheet_name = &worksheet_names[0];
    let range = workbook
        .worksheet_range(worksheet_name)
        .ok_or_else(|| anyhow!("Worksheet '{}' not found", worksheet_name))?
        .map_err(|e| anyhow!("Error reading worksheet '{}': {}", worksheet_name, e))?;

    let mut headers = Vec::new();
    let mut data = Vec::new();

    // Read data from the range
    for (row_idx, row) in range.rows().enumerate() {
        let row_data: Vec<String> = row
            .iter()
            .map(|cell| match cell {
                DataType::Empty => String::new(),
                DataType::String(s) => s.clone(),
                DataType::Float(f) => f.to_string(),
                DataType::Int(i) => i.to_string(),
                DataType::Bool(b) => b.to_string(),
                DataType::Error(e) => format!("Error: {:?}", e),
                DataType::DateTime(dt) => dt.to_string(),
                DataType::DateTimeIso(dt) => dt.clone(),
                DataType::DurationIso(d) => d.clone(),
                DataType::Duration(d) => d.to_string(),
            })
            .collect();

        if row_idx == 0 {
            headers = row_data;
        } else {
            data.push(row_data);
        }
    }

    Ok(DataSet::new(
        headers,
        data,
        path.to_string_lossy().to_string(),
    ))
}

pub fn read_file<P: AsRef<Path>>(file_path: P) -> Result<DataSet> {
    let path = file_path.as_ref();
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .ok_or_else(|| anyhow!("Unable to determine file extension"))?
        .to_lowercase();

    match extension.as_str() {
        "csv" => read_csv_file(path),
        "xlsx" | "xls" => read_excel_file(path),
        _ => Err(anyhow!("Unsupported file format: {}", extension)),
    }
}

pub fn save_dataset_to_csv<P: AsRef<Path>>(dataset: &DataSet, output_path: P) -> Result<()> {
    let path = output_path.as_ref();
    let mut writer = csv::Writer::from_path(path)?;

    // Write headers
    writer.write_record(&dataset.headers)?;

    // Write data
    for row in &dataset.data {
        writer.write_record(row)?;
    }

    writer.flush()?;
    Ok(())
}

pub fn extract_subset(dataset: &DataSet, row_indices: Option<Vec<usize>>, column_names: Option<Vec<String>>) -> Result<DataSet> {
    let selected_columns = if let Some(cols) = column_names {
        let mut indices = Vec::new();
        for col_name in &cols {
            let index = dataset.headers
                .iter()
                .position(|h| h == col_name)
                .ok_or_else(|| anyhow!("Column '{}' not found", col_name))?;
            indices.push(index);
        }
        Some((cols, indices))
    } else {
        None
    };

    let (new_headers, column_indices) = if let Some((cols, indices)) = selected_columns {
        (cols, indices)
    } else {
        (dataset.headers.clone(), (0..dataset.headers.len()).collect())
    };

    let selected_rows = if let Some(rows) = row_indices {
        rows
    } else {
        (0..dataset.data.len()).collect()
    };

    let mut new_data = Vec::new();
    for &row_idx in &selected_rows {
        if let Some(row) = dataset.data.get(row_idx) {
            let new_row: Vec<String> = column_indices
                .iter()
                .map(|&col_idx| row.get(col_idx).unwrap_or(&String::new()).clone())
                .collect();
            new_data.push(new_row);
        }
    }

    Ok(DataSet::new(
        new_headers,
        new_data,
        format!("{}_subset", dataset.file_path),
    ))
}
