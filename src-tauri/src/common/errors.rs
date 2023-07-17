use mongodb::error::Error;


#[derive(Debug, thiserror::Error)]
pub enum DBErrors {
    #[error("Data was not found")]
    NotFound,
    #[error("The value inputed is incorrect")]
    IncorrectInput(String),
    #[error("No results")]
    NoResults,
    #[error("Something went wrong: {0}")]
    InternalError(String),
}

impl DBErrors {
    pub fn into_string(&self) -> String {
        match &self {
            DBErrors::NotFound => String::from("Value was not found"),
            DBErrors::IncorrectInput(e) => format!("The input is incorrect: {e}"),
            DBErrors::NoResults => String::from("No results"),
            DBErrors::InternalError(e) => format!("Internal error: {e}"),
        }
    }

    pub fn from_unknown_result<T>(result: Result<T, Error>, message: &str) -> Result<T, Self> {
        match result {
            Ok(v) => Ok(v),
            Err(e) => Err(Self::InternalError(format!("Something went wrong: {message}")))
        }
    }
}

pub type DBResult<T> = Result<T, DBErrors>; 