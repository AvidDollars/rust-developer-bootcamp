use thiserror::Error as ThisErr;

#[derive(ThisErr, Debug)]
pub enum DataProcessingError {

    #[error("Data not found: {0}")] // referring to 0th field of "NotFound" tuple
    NotFound(String),

    #[error("Invalid data format")]
    InvalidFormat,

    #[error("Lol: {name}")]
    StructLike { name: String },

    #[error("IO error")]
    Io(#[from] std::io::Error) // auto conversion 

}

pub fn process_data(file_path: &str) -> Result<(), DataProcessingError> {
    if file_path.is_empty() {
        return Err(DataProcessingError::NotFound(file_path.into()));
    }

    let data = std::fs::read_to_string(file_path)?;

    if data.is_empty() {
        return Err(DataProcessingError::InvalidFormat)
    }

    Ok(())
}