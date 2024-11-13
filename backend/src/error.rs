use axum::http;
use axum::response;

#[derive(Debug)]
pub struct Error(worker::Error);

impl Error {
    pub fn infallible<T>(_error: T) -> Self {
        Error(worker::Error::Infallible)
    }

    pub fn not_found() -> Self {
        Error(worker::Error::RustError(String::from("not found")))
    }

    pub fn from_string(value: impl ToString) -> Self {
        Error(worker::Error::RustError(value.to_string()))
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}

impl From<worker::Error> for Error {
    fn from(value: worker::Error) -> Self {
        Error(value)
    }
}

impl response::IntoResponse for Error {
    fn into_response(self) -> response::Response {
        (http::StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
    }
}
