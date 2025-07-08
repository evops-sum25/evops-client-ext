use thiserror::Error;

pub type ApiResult<T> = std::result::Result<T, crate::ApiError>;

#[derive(Debug, Error)]
#[error("{0}")]
#[cfg_attr(
    feature = "aide",
    derive(aide::OperationIo),
    aide(output_with = "String")
)]
pub enum ApiError {
    InvalidArgument(String),
    NotFound(String),
    AlreadyExists(String),
    #[error(transparent)]
    Db(#[from] diesel::result::Error),
    Storage(String),
    Other(String),
    #[error("gRPC connection error: {0}")]
    GrpcConnection(String),
    #[error("gRPC request failed: {0}")]
    GrpcRequest(String),
    #[error("gRPC timeout after retries")]
    GrpcTimeout,
    #[error("Invalid gRPC response: {0}")]
    GrpcResponse(String),
}

#[cfg(feature = "axum")]
impl axum::response::IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        use axum::http::StatusCode;

        match self {
            ApiError::InvalidArgument(e) => (StatusCode::UNPROCESSABLE_ENTITY, e),
            ApiError::NotFound(e) => (StatusCode::NOT_FOUND, e),
            ApiError::AlreadyExists(e) => (StatusCode::CONFLICT, e),
            ApiError::Db(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            ApiError::Storage(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            ApiError::Other(e) => (StatusCode::INTERNAL_SERVER_ERROR, e),
            ApiError::GrpcConnection(e) => (StatusCode::BAD_GATEWAY, e),
            ApiError::GrpcRequest(e) => (StatusCode::BAD_GATEWAY, e),
            ApiError::GrpcTimeout => (
                StatusCode::GATEWAY_TIMEOUT,
                "gRPC timeout after retries".to_string(),
            ),
            ApiError::GrpcResponse(e) => (StatusCode::BAD_GATEWAY, e),
        }
        .into_response()
    }
}

#[cfg(feature = "tonic")]
impl From<ApiError> for tonic::Status {
    fn from(value: ApiError) -> Self {
        match value {
            ApiError::InvalidArgument(e) => Self::invalid_argument(e),
            ApiError::NotFound(e) => Self::not_found(e),
            ApiError::AlreadyExists(e) => Self::already_exists(e),
            ApiError::Db(e) => Self::internal(e.to_string()),
            ApiError::Storage(e) => Self::internal(e.to_string()),
            ApiError::Other(e) => Self::unknown(e),
            ApiError::GrpcConnection(e) => Self::unavailable(e),
            ApiError::GrpcRequest(e) => Self::internal(e),
            ApiError::GrpcTimeout => Self::deadline_exceeded("gRPC timeout after retries"),
            ApiError::GrpcResponse(e) => Self::invalid_argument(e),
        }
    }
}

#[cfg(feature = "tonic")]
impl From<tonic::transport::Error> for ApiError {
    fn from(err: tonic::transport::Error) -> Self {
        ApiError::GrpcConnection(format!("Transport error: {}", err))
    }
}

#[cfg(feature = "tonic")]
impl From<tonic::Status> for ApiError {
    fn from(status: tonic::Status) -> Self {
        ApiError::GrpcRequest(format!(
            "gRPC status: {} - {}",
            status.code(),
            status.message()
        ))
    }
}
