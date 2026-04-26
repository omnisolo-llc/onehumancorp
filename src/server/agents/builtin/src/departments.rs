use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum Department {
    Operations,
    Marketing,
    Sales,
    CustomerSuccess,
    Finance,
    Legal,
    BusinessAdvisory,
}

impl FromStr for Department {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "operations" => Ok(Department::Operations),
            "marketing" => Ok(Department::Marketing),
            "sales" => Ok(Department::Sales),
            "customersuccess" | "customer_success" => Ok(Department::CustomerSuccess),
            "finance" => Ok(Department::Finance),
            "legal" => Ok(Department::Legal),
            "businessadvisory" | "business_advisory" => Ok(Department::BusinessAdvisory),
            _ => Err(format!("Unknown department: {}", s)),
        }
    }
}

pub struct DepartmentConfig {
    pub system_prompt: &'static str,
    pub allowed_tools: Vec<&'static str>,
}

pub fn get_department_config(dep: Department) -> DepartmentConfig {
    match dep {
        Department::Operations => DepartmentConfig {
            system_prompt: "You are the Operations Manager agent. Your primary job is to handle order/booking processing, inventory alerts, and fulfillment coordination. Ensure smooth business operations.",
            allowed_tools: vec!["read", "write", "glob", "task_create", "task_update"],
        },
        Department::Marketing => DepartmentConfig {
            system_prompt: "You are the Promoter agent handling Marketing & Advertising. You design websites, optimize SEO, create social posts, promotional content, and generate QR codes.",
            allowed_tools: vec!["write", "websearch", "webfetch"],
        },
        Department::Sales => DepartmentConfig {
            system_prompt: "You are the Salesperson agent handling Sales & Acquisition. You generate quotes, follow up on leads, and track referrals.",
            allowed_tools: vec!["read", "write", "sendmessage"],
        },
        Department::CustomerSuccess => DepartmentConfig {
            system_prompt: "You are the Ambassador agent handling Customer Success. You reply to messages across all channels, provide order updates, and request reviews.",
            allowed_tools: vec!["read", "sendmessage", "task_list"],
        },
        Department::Finance => DepartmentConfig {
            system_prompt: "You are the Accountant agent handling Finance & Payments. You generate financial reports, handle subscription billing, and summarize taxes.",
            allowed_tools: vec!["read", "write", "bash"], // Bash might be needed for running financial scripts
        },
        Department::Legal => DepartmentConfig {
            system_prompt: "You are the Protector agent handling Legal & Compliance. You draft terms/policies, contracts, ensure GDPR compliance, and track licenses.",
            allowed_tools: vec!["read", "write", "grep"],
        },
        Department::BusinessAdvisory => DepartmentConfig {
            system_prompt: "You are the Advisor agent handling Business Advisory. You generate weekly health reports, suggest next actions, and analyze seasonal trends.",
            allowed_tools: vec!["read", "write", "websearch"],
        },
    }
}
