use ports::errors::GeoClientError;

pub enum GeoClientGrpcError {
    GrpcError(String),
    ExecutionError(String),
}

impl From<tonic::transport::Error> for GeoClientGrpcError {
    fn from(v: tonic::transport::Error) -> Self {
        Self::GrpcError(v.to_string())
    }
}

impl From<tonic::Status> for GeoClientGrpcError {
    fn from(v: tonic::Status) -> Self {
        Self::GrpcError(v.to_string())
    }
}

impl From<GeoClientGrpcError> for GeoClientError {
    fn from(v: GeoClientGrpcError) -> Self {
        match v {
            GeoClientGrpcError::GrpcError(msg) => Self::ConnectionError(msg),
            GeoClientGrpcError::ExecutionError(msg) => Self::ExecutionError(msg),
        }
    }
}
