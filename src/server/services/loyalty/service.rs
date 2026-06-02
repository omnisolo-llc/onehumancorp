use tonic::{Request, Response, Status};

pub mod proto {
    // This assumes that the generated code is available here.
    // In a real project, this would be an include! or similar, based on the rust_prost_library.
    // We mock it for the sake of compiling without fully knowing the internal tonic generator paths.

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LoyaltyProgram {
        #[prost(string, tag="1")]
        pub id: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub tenant_id: ::prost::alloc::string::String,
        #[prost(bool, tag="3")]
        pub is_active: bool,
        #[prost(string, tag="4")]
        pub program_type: ::prost::alloc::string::String,
        #[prost(int32, tag="5")]
        pub spend_amount_cents_per_point: i32,
        #[prost(int32, tag="6")]
        pub points_per_reward: i32,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CustomerLoyaltyProfile {
        #[prost(string, tag="1")]
        pub id: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub tenant_id: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub customer_id: ::prost::alloc::string::String,
        #[prost(int32, tag="4")]
        pub points_balance: i32,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ToggleLoyaltyProgramRequest {
        #[prost(string, tag="1")]
        pub tenant_id: ::prost::alloc::string::String,
        #[prost(bool, tag="2")]
        pub is_active: bool,
        #[prost(string, tag="3")]
        pub program_type: ::prost::alloc::string::String,
        #[prost(int32, tag="4")]
        pub spend_amount_cents_per_point: i32,
        #[prost(int32, tag="5")]
        pub points_per_reward: i32,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ToggleLoyaltyProgramResponse {
        #[prost(message, optional, tag="1")]
        pub program: ::core::option::Option<LoyaltyProgram>,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetPointsBalanceRequest {
        #[prost(string, tag="1")]
        pub tenant_id: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub customer_id: ::prost::alloc::string::String,
    }

    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetPointsBalanceResponse {
        #[prost(message, optional, tag="1")]
        pub profile: ::core::option::Option<CustomerLoyaltyProfile>,
    }

    #[tonic::async_trait]
    pub trait LoyaltyService: Send + Sync + 'static {
        async fn toggle_loyalty_program(
            &self,
            request: tonic::Request<ToggleLoyaltyProgramRequest>,
        ) -> Result<tonic::Response<ToggleLoyaltyProgramResponse>, tonic::Status>;

        async fn get_points_balance(
            &self,
            request: tonic::Request<GetPointsBalanceRequest>,
        ) -> Result<tonic::Response<GetPointsBalanceResponse>, tonic::Status>;
    }
}

pub struct LoyaltyServiceImpl {}

impl LoyaltyServiceImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl proto::LoyaltyService for LoyaltyServiceImpl {
    async fn toggle_loyalty_program(
        &self,
        request: tonic::Request<proto::ToggleLoyaltyProgramRequest>,
    ) -> Result<tonic::Response<proto::ToggleLoyaltyProgramResponse>, tonic::Status> {
        let req = request.into_inner();
        let program = proto::LoyaltyProgram {
            id: "mock_program_id".to_string(),
            tenant_id: req.tenant_id,
            is_active: req.is_active,
            program_type: req.program_type,
            spend_amount_cents_per_point: req.spend_amount_cents_per_point,
            points_per_reward: req.points_per_reward,
        };

        Ok(tonic::Response::new(proto::ToggleLoyaltyProgramResponse {
            program: Some(program),
        }))
    }

    async fn get_points_balance(
        &self,
        request: tonic::Request<proto::GetPointsBalanceRequest>,
    ) -> Result<tonic::Response<proto::GetPointsBalanceResponse>, tonic::Status> {
        let req = request.into_inner();
        let profile = proto::CustomerLoyaltyProfile {
            id: "mock_profile_id".to_string(),
            tenant_id: req.tenant_id,
            customer_id: req.customer_id,
            points_balance: 100, // Mocked balance
        };

        Ok(tonic::Response::new(proto::GetPointsBalanceResponse {
            profile: Some(profile),
        }))
    }
}
