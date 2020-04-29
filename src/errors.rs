#[derive(Debug)]
pub enum AppError {
    IoError(std::io::Error),
    ParseError(serde_xml_rs::Error),
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::IoError(error)
    }
}

impl From<serde_xml_rs::Error> for AppError {
    fn from(error: serde_xml_rs::Error) -> Self {
        AppError::ParseError(error)
    }
}
