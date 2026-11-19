use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone,Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    LoginFailure,
    // AuthError
    NoAuthCookie,
    WrongTokenFormat,
    CtxNotInRequestExtension,
    // model errors
    TicketNotFound { id: u64 },
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:<12} -- {self:?}", "into_response fn");

        // create placeholder for axum response
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // insert d error into response
        response.extensions_mut().insert(self);
        response
    }
}

//  start_region = Error Boiler plater
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl Error {
    pub fn client_error_and_status(&self) -> (StatusCode, ClientError) {
      #[allow(unreachable_patterns)]
        match self {
            Self::LoginFailure => (StatusCode::BAD_REQUEST, ClientError::LoginFailError), // auth
            Self::CtxNotInRequestExtension | Self::NoAuthCookie | Self::WrongTokenFormat => {
                (StatusCode::FORBIDDEN, ClientError::NoAuthError)
            }
            //  model
            Self::TicketNotFound { id } => (StatusCode::NOT_FOUND, ClientError::InvalidParamsError),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, ClientError::ServiceError),
        }
        // result
    }
}
//  end_region = Error Boiler plater

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LoginFailError,
    NoAuthError,
    InvalidParamsError,
    ServiceError,
}
