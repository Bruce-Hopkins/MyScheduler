use mongodb::error::Error;


#[derive(Debug, thiserror::Error)]
pub enum AppErrors {
    #[error("Data was not found")]
    NotFound,
    #[error("The value inputed is incorrect")]
    IncorrectInput(String),
    #[error("No results")]
    NoResults,
    #[error("Something went wrong: {0}")]
    InternalError(String),
}

impl AppErrors {
    pub fn into_string(&self) -> String {
        match &self {
            AppErrors::NotFound => String::from("Value was not found"),
            AppErrors::IncorrectInput(e) => format!("The input is incorrect: {e}"),
            AppErrors::NoResults => String::from("No results"),
            AppErrors::InternalError(e) => format!("Internal error: {e}"),
        }
    }

    pub fn from_unknown_result<T>(result: Result<T, Error>, message: &str) -> Result<T, Self> {
        match result {
            Ok(v) => Ok(v),
            Err(e) => Err(Self::InternalError(format!("Something went wrong: {message}")))
        }
    }
}

pub type AppResult<T> = Result<T, AppErrors>; 