use axum::{
    async_trait,
    body::Bytes,
    extract::{FromRequest, Request},
    http::{header, HeaderMap},
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;
use serde_html_form::de::Error as DeserializeError;
use serde_valid::{validation::Errors as ValidationErrors, Validate};

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidForm<T>(pub T)
where
    T: DeserializeOwned + Validate;

impl<T> ValidForm<T>
where
    T: DeserializeOwned + Validate,
{
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, InvalidForm> {
        log::info!("got dem bytes, will try to deserialize");
        let value = serde_html_form::from_bytes::<T>(bytes)
            .inspect_err(|err| log::error!("LOL?! {:?}", err))?;
        log::info!("deserialized validating");
        value.validate()?;
        log::info!("validated, returning");

        Ok(ValidForm(value))
    }
}

#[async_trait]
impl<T, S> FromRequest<S> for ValidForm<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = InvalidForm;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        if form_content_type(req.headers()) {
            let bytes = Bytes::from_request(req, state)
                .await
                .map_err(|_err| InvalidForm::default())?;
            Self::from_bytes(&bytes)
        } else {
            log::info!("Invalid content-type");
            Err(InvalidForm::default())
        }
    }
}

fn form_content_type(headers: &HeaderMap) -> bool {
    let content_type = if let Some(content_type) = headers.get(header::CONTENT_TYPE) {
        content_type
    } else {
        return false;
    };
    return content_type.to_str().unwrap_or_default().to_lowercase()
        == "application/x-www-form-urlencoded";
}

#[derive(Debug, Clone, Default)]
pub struct InvalidForm {
    pub deserialize_error: Option<DeserializeError>,
    pub validation_error: Option<ValidationErrors>,
}

impl From<DeserializeError> for InvalidForm {
    fn from(value: DeserializeError) -> Self {
        InvalidForm {
            deserialize_error: Some(value),
            validation_error: None,
        }
    }
}

impl From<ValidationErrors> for InvalidForm {
    fn from(value: ValidationErrors) -> Self {
        InvalidForm {
            deserialize_error: None,
            validation_error: Some(value),
        }
    }
}

impl IntoResponse for InvalidForm {
    fn into_response(self) -> Response {
        Response::builder()
            .status(400)
            .body(format!("{:?}", self.deserialize_error).into())
            .unwrap()
    }
}
