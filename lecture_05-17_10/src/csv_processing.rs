//! module containing [`CsvTable`] struct that provides functionality for processing CSV input

use std::io::{self, Error as IoError, ErrorKind};
use std::str;

#[derive(Debug, Default)]
pub struct CsvTable {
    header: Vec<String>,
    rows: Vec<Vec<String>>,
    max_len_per_column: Vec<usize>,
}

impl CsvTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn output_to(&mut self, mut output: impl io::Write) -> io::Result<()> {
        if self.header.is_empty() {
            return Ok(());
        }

        self.header
            .iter_mut()
            .enumerate()
            .for_each(|(index, field)| {
                let to_add = self.max_len_per_column[index] - field.chars().count();
                (0..to_add).for_each(|_| (field).push(' '));
            });

        let hr_line = " | ";
        let header = self.header.join(hr_line);

        for row in &mut self.rows {
            row.iter_mut().enumerate().for_each(|(index, field)| {
                let to_add = self.max_len_per_column[index] - field.chars().count();
                (0..to_add).for_each(|_| (field).push(' '));
            });
        }

        let table: Vec<String> = self.rows.iter().map(|row| row.join(hr_line)).collect();

        let hr_line = "-".repeat(
            self.max_len_per_column.iter().sum::<usize>()
                + (self.header.iter().count() - 1) * hr_line.chars().count(),
        );
        write!(output, "{}\n{}\n{}", header, hr_line, table.join("\n"))
    }
}

impl io::Write for CsvTable {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let input: String = str::from_utf8(buf)
            .map_err(|error| IoError::new(ErrorKind::InvalidData, error))?
            .trim()
            .into();

        if input.chars().count() > 0 {
            let fields: Vec<String> = input
                .split(',')
                .map(|field| field.trim())
                .map(|field| field.into())
                .collect();

            match self.header.is_empty() {
                true => {
                    self.header.extend(fields);

                    if self.max_len_per_column.is_empty() {
                        let headers_fields_len: Vec<_> = self
                            .header
                            .iter()
                            .map(|field| field.chars().count())
                            .collect();
                        self.max_len_per_column.extend(headers_fields_len);
                    }
                }
                false => {
                    if fields.len() != self.header.len() {
                        return Err(IoError::new(
                            ErrorKind::InvalidInput,
                            "invalid number of columns",
                        ));
                    } else {
                        fields.iter().enumerate().for_each(|(index, field)| {
                            let field_len = field.chars().count();
                            if field_len > self.max_len_per_column[index] {
                                self.max_len_per_column[index] = field_len;
                            }
                        });

                        self.rows.push(fields);
                    }
                }
            }
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        todo!()
    }
}
