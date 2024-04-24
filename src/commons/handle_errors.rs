use serde_derive::{Deserialize, Serialize};
//use reqwest::Error as ReqwestError;
//use reqwest_middleware::Error as MiddlewareReqwestError;
use warp::{
    filters::{body::BodyDeserializeError, cors::CorsForbidden},
    http::StatusCode,
    reject::Reject,
    reply::{json, Json, WithStatus},
    Rejection, Reply,
};

#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    Unauthorized,
    //DatabaseQueryError(sqlx::Error),
    //MigrationError(sqlx::migrate::MigrateError),
    //ReqwestAPIError(ReqwestError),
    //MiddlewareReqwestAPIError(MiddlewareReqwestError),
    ClientError(APILayerError),
    ServerError(APILayerError),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APILayerError {
    pub status: u16,
    pub message: String,
}

impl std::fmt::Display for APILayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Status: {}, Message: {}", self.status, self.message)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &*self {
            Error::ParseError(ref err) => write!(f, "Cannot parse parameter: {}", err),
            Error::MissingParameters => write!(f, "Missing parameter"),
            Error::Unauthorized => write!(f, "No permission to change the underlying resource"),
            //Error::DatabaseQueryError(_) => write!(f, "Cannot update, invalid data"),
            //Error::MigrationError(_) => write!(f, "Cannot migrate data"),
            //Error::ReqwestAPIError(err) => write!(f, "External API error: {}", err),
            //Error::MiddlewareReqwestAPIError(err) => write!(f, "External API error: {}", err),
            Error::ClientError(err) => write!(f, "External Client error: {}", err),
            Error::ServerError(err) => write!(f, "External Server error: {}", err),
        }
    }
}

impl Reject for Error {}
impl Reject for APILayerError {}

pub async fn return_error(r: Rejection) -> Result<WithStatus<Json>, Rejection> {
    if let Some(Error::Unauthorized) = r.find() {
        let err = APILayerError {
            status: 403,
            message: "Unauthorized".to_string(),
        };
        Ok(warp::reply::with_status(
            json(&err),
            StatusCode::UNAUTHORIZED,
        ))
    //} else if let Some(crate::Error::MiddlewareReqwestAPIError(e)) = r.find() {
    //    event!(Level::ERROR, "{}", e);
    //    Ok(warp::reply::with_status(
    //        "Internal Server Error".to_string(),
    //        StatusCode::INTERNAL_SERVER_ERROR,
    //    ))
    //
    //} else if let Some(Error::ClientError(e)) = r.find() {
    //    Ok(warp::reply::with_status(
    //        "Internal Server Error".to_string(),
    //        StatusCode::INTERNAL_SERVER_ERROR,
    //    ))
    //} else if let Some(Error::ServerError(e)) = r.find() {
    //    Ok(warp::reply::with_status(
    //        "Internal Server Error".to_string(),
    //        StatusCode::INTERNAL_SERVER_ERROR,
    //    ))
    //} else if let Some(error) = r.find::<CorsForbidden>() {
    //    Ok(warp::reply::with_status(
    //        error.to_string(),
    //        StatusCode::FORBIDDEN,
    //    ))
    //} else if let Some(error) = r.find::<BodyDeserializeError>() {
    //    Ok(warp::reply::with_status(
    //        error.to_string(),
    //        StatusCode::UNPROCESSABLE_ENTITY,
    //    ))
    //} else if let Some(error) = r.find::<Error>() {
    //    Ok(warp::reply::with_status(
    //        "".to_string(),
    //        StatusCode::UNPROCESSABLE_ENTITY,
    //    ))
    } else {
        let err = APILayerError {
            status: 500,
            message: "Internal Server Error".to_string(),
        };
        Ok(warp::reply::with_status(json(&err), StatusCode::NOT_FOUND))
    }
}
