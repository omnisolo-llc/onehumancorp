#[cfg(not(ohc_bazel))]
pub mod ohc {
    pub mod interop {
        tonic::include_proto!("ohc.interop");
    }
    pub mod mcp_proxy {
        tonic::include_proto!("ohc.mcp_proxy");
    }
    pub mod orchestration {
        tonic::include_proto!("ohc.orchestration");
    }
    pub mod billing {
        tonic::include_proto!("ohc.billing");
    }
    pub mod agent {
        tonic::include_proto!("ohc.agent");
        pub mod service {
            tonic::include_proto!("ohc.agent.service");
        }
    }
    pub mod organization {
        tonic::include_proto!("ohc.organization");
    }
    pub mod common {
        tonic::include_proto!("ohc.common");
    }
    pub mod api {
        pub mod v1 {
            tonic::include_proto!("ohc.api.v1");
        }
    }
    pub mod campaign {
        tonic::include_proto!("ohc.campaign");
    }
    pub mod collective {
        tonic::include_proto!("ohc.collective");
    }
    pub mod invoice {
        tonic::include_proto!("ohc.invoice");
    }
    pub mod inventory {
        tonic::include_proto!("ohc.inventory");
    }
    pub mod harness_middleware {
        tonic::include_proto!("omnisolo.harness.middleware.v1");
    }
}

#[cfg(ohc_bazel)]
pub mod ohc {
    pub mod interop {
        pub use interop_proto::ohc::interop::*;
    }
    pub mod mcp_proxy {
        pub use mcp_proxy_proto::ohc::mcp_proxy::*;
    }
    pub mod orchestration {
        pub use hub_proto::ohc::orchestration::*;
    }
    pub mod billing {
        pub use billing_proto::ohc::billing::*;
    }
    pub mod agent {
        pub use agent_proto::ohc::agent::*;
        pub mod service {
            pub use agent_service_proto::ohc::agent::service::*;
        }
    }
    pub mod organization {
        pub use organization_proto::ohc::organization::*;
    }
    pub mod common {
        pub use common_proto::ohc::common::*;
    }
    pub mod api {
        pub mod v1 {
            pub use app_proto::ohc::api::v1::*;
        }
    }
    pub mod campaign {
        pub use campaign_proto::ohc::campaign::*;
    }
    pub mod collective {
        pub use collective_proto::ohc::collective::*;
    }
    pub mod invoice {
        pub use invoice_proto::ohc::invoice::*;
    }
    pub mod inventory {
        pub use inventory_proto::ohc::inventory::*;
    }
    pub mod harness_middleware {
        pub use harness_middleware_proto::omnisolo::harness::middleware::v1::*;
    }
}

pub mod interop {
    pub use crate::ohc::interop::*;
}
pub mod mcp_proxy {
    pub use crate::ohc::mcp_proxy::*;
}
pub mod orchestration {
    pub use crate::ohc::orchestration::*;
}
pub mod billing {
    pub use crate::ohc::billing::*;
}
pub mod agent {
    pub use crate::ohc::agent::*;
    pub mod service {
        pub use crate::ohc::agent::service::*;
    }
}
pub mod organization {
    pub use crate::ohc::organization::*;
}
pub mod common {
    pub use crate::ohc::common::*;
}
pub mod app {
    pub use crate::ohc::api::v1::*;
}
pub mod campaign {
    pub use crate::ohc::campaign::*;
}
pub mod collective {
    pub use crate::ohc::collective::*;
}
pub mod invoice {
    pub use crate::ohc::invoice::*;
}

pub mod inventory {
    pub use crate::ohc::inventory::*;
}

pub mod harness_middleware {
    pub use crate::ohc::harness_middleware::*;
}

#[cfg(test)]
mod tests {
    use prost::Message;

    use super::harness_middleware;

    #[test]
    fn harness_worker_proto_round_trip_preserves_version_and_extensions() {
        let message = harness_middleware::AttemptCommandEnvelope {
            protocol_version: 2,
            tenant_id: "tenant-1".to_owned(),
            session_id: "session-1".to_owned(),
            task_id: "task-1".to_owned(),
            attempt_id: "attempt-1".to_owned(),
            turn_id: String::new(),
            command_id: "command-1".to_owned(),
            lease_id: "lease-1".to_owned(),
            lease_generation: 9,
            fencing_token: "fence-9".to_owned(),
            kind: "execute".to_owned(),
            correlation_id: "corr-1".to_owned(),
            idempotency_key: "idem-1".to_owned(),
            payload_schema: "omnisolo.command.v1".to_owned(),
            payload_version: 7,
            payload: b"payload".to_vec(),
            extensions: [("future_field".to_owned(), "enabled".to_owned())]
                .into_iter()
                .collect(),
        };
        let decoded = harness_middleware::AttemptCommandEnvelope::decode(
            message.encode_to_vec().as_slice(),
        )
        .unwrap();
        assert_eq!(decoded, message);
        assert!(decoded.turn_id.is_empty());
        assert_eq!(decoded.payload_version, 7);
        assert_eq!(decoded.extensions["future_field"], "enabled");
    }
}
