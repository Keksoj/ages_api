use actix_web::{http, http::StatusCode, HttpResponse, ResponseError};
use bcrypt;
use diesel::result::Error as DieselError;
use serde::Deserialize;
use serde_json::json;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct CustomError {
    pub error_status_code: u16,
    pub error_message: String,
}

impl CustomError {
    pub fn new(error_status_code: u16, error_message: String) -> Self {
        Self {
            error_status_code,
            error_message,
        }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.error_message.as_str())
    }
}

// diesel::result::Error
impl From<DieselError> for CustomError {
    fn from(error: DieselError) -> CustomError {
        match error {
            DieselError::DatabaseError(_, err) => {
                CustomError::new(409, err.message().to_string())
            }
            DieselError::NotFound => {
                CustomError::new(404, "Person not found".to_string())
            }
            err => CustomError::new(500, format!("Unknown Diesel error: {}", err)),
        }
    }
}
impl From<http::header::ToStrError> for CustomError {
    fn from(error: http::header::ToStrError) -> CustomError {
        CustomError::new(500, error.to_string())
    }
}
impl From<jsonwebtoken::errors::Error> for CustomError {
    fn from(error: jsonwebtoken::errors::Error) -> CustomError {
        CustomError::new(500, error.to_string())
    }
}
impl From<String> for CustomError {
    fn from(error: String) -> CustomError {
        CustomError::new(500, error)
    }
}
impl From<serde_json::error::Error> for CustomError {
    fn from(error: serde_json::error::Error) -> CustomError {
        CustomError::new(500, error.to_string())
    }
}
impl From<bcrypt::BcryptError> for CustomError {
    fn from(error: bcrypt::BcryptError) -> CustomError {
        CustomError::new(500, error.to_string())
    }
}
impl From <r2d2::Error> for CustomError {
    fn from(error: r2d2::Error) -> CustomError {
        CustomError::new(500, error.to_string())
    }
}
// impl From<NoneError> for CustomError {
//     fn from(error: NoneError) -> CustomError {
//         CustomError::new(500, error)
//     }
// }

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        let status_code = match StatusCode::from_u16(self.error_status_code) {
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        // this filters error messages under 500
        // good for privacy, bad for debugging
        // let error_message = match status_code.as_u16() < 500 {
        //     true => self.error_message.clone(),
        //     false => "Internal server error".to_string(),
        // };
        let error_message = &self.error_message;

        HttpResponse::build(status_code).json(json!({ "message": error_message }))
    }
}

