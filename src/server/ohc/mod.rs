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
    pub mod inbox {
        tonic::include_proto!("ohc.inbox");
    }
    pub mod inventory {
        tonic::include_proto!("ohc.inventory");
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
    pub mod inbox {
        pub use inbox_proto::ohc::inbox::*;
    }
    pub mod inventory {
        pub use inventory_proto::ohc::inventory::*;
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

pub mod inbox {
    pub use crate::ohc::inbox::*;
}
pub mod inventory {
    pub use crate::ohc::inventory::*;
}
