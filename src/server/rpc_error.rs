//! Compact internal RPC errors. Keep the complete tonic status (including
//! details and metadata), allocating it only on error paths. Convert back at
//! protocol boundaries instead of changing public RPC response semantics.
use std::{fmt, ops::Deref};
use tonic::Status;

#[derive(Debug)]
pub struct RpcError(Box<Status>);

impl From<Status> for RpcError {
    fn from(status: Status) -> Self {
        Self(Box::new(status))
    }
}

impl From<RpcError> for Status {
    fn from(error: RpcError) -> Self {
        *error.0
    }
}

impl Deref for RpcError {
    type Target = Status;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for RpcError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, formatter)
    }
}

impl std::error::Error for RpcError {}

impl RpcError {
    pub fn invalid_argument(message: impl Into<String>) -> Self {
        Status::invalid_argument(message).into()
    }

    pub fn failed_precondition(message: impl Into<String>) -> Self {
        Status::failed_precondition(message).into()
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Status::internal(message).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_preserves_protocol_status_details_and_metadata() {
        let mut status = Status::with_details(
            tonic::Code::PermissionDenied,
            "owner approval required",
            b"evidence".as_slice().into(),
        );
        status
            .metadata_mut()
            .insert("x-request-id", "request-1".parse().unwrap());
        let error = RpcError::from(status);
        assert_eq!(error.code(), tonic::Code::PermissionDenied);
        let restored = Status::from(error);
        assert_eq!(restored.code(), tonic::Code::PermissionDenied);
        assert_eq!(restored.message(), "owner approval required");
        assert_eq!(restored.details(), b"evidence");
        assert_eq!(
            restored.metadata().get("x-request-id").unwrap(),
            "request-1"
        );
    }

    #[test]
    fn validation_codes_remain_distinct_and_internal_errors_stay_compact() {
        assert_eq!(
            RpcError::invalid_argument("invalid input").code(),
            tonic::Code::InvalidArgument
        );
        assert_eq!(
            RpcError::failed_precondition("needs evidence").code(),
            tonic::Code::FailedPrecondition
        );
        assert_eq!(
            RpcError::internal("storage unavailable").code(),
            tonic::Code::Internal
        );
        assert_eq!(
            std::mem::size_of::<RpcError>(),
            std::mem::size_of::<usize>()
        );
        assert!(
            std::mem::size_of::<Result<i64, RpcError>>()
                < std::mem::size_of::<Result<i64, Status>>()
        );
    }
}
