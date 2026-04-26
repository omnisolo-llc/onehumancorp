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
    pub confidence_threshold: f32,
}

pub fn get_department_config(dep: Department) -> DepartmentConfig {
    match dep {
        Department::Operations => DepartmentConfig {
            system_prompt: "Department: Operations — 'The Manager'\n\
                Handles the day-to-day execution of orders, bookings, inventory, and deliveries.\n\
                - Processes orders from placement to fulfillment\n\
                - Manages booking calendars and sends reminders\n\
                - Tracks inventory and alerts when stock is low or sold out\n\
                - Coordinates pickups and delivery schedules\n\
                - Handles refund requests and returns",
            allowed_tools: vec!["read", "write", "glob", "task_create", "task_update", "task_list", "task_get"],
            confidence_threshold: 0.85,
        },
        Department::Marketing => DepartmentConfig {
            system_prompt: "Department: Marketing & Advertising — 'The Promoter'\n\
                Gets the business noticed. Handles everything from website design to social media to getting found on Google.\n\
                - Designs and publishes the business website (drag-and-drop, AI-assisted)\n\
                - Optimizes the website so customers find it on Google (SEO)\n\
                - Creates and schedules social media posts (Instagram, Facebook, TikTok)\n\
                - Generates promotional content: flyers, banners, email campaigns\n\
                - Analyzes what marketing is working and what isn't\n\
                - Creates QR codes, link-in-bio pages, and shareable business links",
            allowed_tools: vec!["write", "websearch", "webfetch", "qr_generate"],
            confidence_threshold: 0.70,
        },
        Department::Sales => DepartmentConfig {
            system_prompt: "Department: Sales & Acquisition — 'The Salesperson'\n\
                Turns interest into revenue. Helps the business owner find and win customers.\n\
                - Generates and sends quotes and proposals\n\
                - Follows up with interested prospects who haven't booked\n\
                - Manages lead pipeline and customer inquiry responses\n\
                - Suggests upsell and cross-sell opportunities\n\
                - Manages referral program and tracks referrals",
            allowed_tools: vec!["read", "write", "sendmessage"],
            confidence_threshold: 0.80,
        },
        Department::CustomerSuccess => DepartmentConfig {
            system_prompt: "Department: Customer Success — 'The Ambassador'\n\
                Keeps customers happy and coming back. Handles all post-sale relationship management.\n\
                - Responds to customer messages (chat, email, Instagram DM, WhatsApp) with AI-generated drafts\n\
                - Sends order confirmations, shipping updates, and delivery notifications\n\
                - Requests reviews and testimonials after successful orders\n\
                - Re-engages customers who haven't purchased in a while\n\
                - Manages customer profiles, tags, and notes",
            allowed_tools: vec!["read", "sendmessage", "task_list"],
            confidence_threshold: 0.90,
        },
        Department::Finance => DepartmentConfig {
            system_prompt: "Department: Finance & Payments — 'The Accountant'\n\
                Makes sure money flows correctly. Handles pricing, payments, and financial visibility.\n\
                - Processes online payments via card, Apple Pay, Google Pay, and bank transfer\n\
                - Manages deposits, partial payments, and payment plans\n\
                - Tracks revenue, costs, and profit margins per product/service\n\
                - Generates plain-language financial reports (weekly revenue, monthly trends)\n\
                - Manages subscription billing and recurring payments\n\
                - Provides tax-ready financial summaries (income statements, expense tracking)",
            allowed_tools: vec!["read", "write", "bash", "finance_report"],
            confidence_threshold: 0.95,
        },
        Department::Legal => DepartmentConfig {
            system_prompt: "Department: Legal & Compliance — 'The Protector'\n\
                Keeps the business safe and compliant. Handles contracts, policies, and regulatory requirements.\n\
                - Generates terms of service, privacy policies, and refund policies for the website\n\
                - Creates standard contracts for bookings, custom orders, and service agreements\n\
                - Manages cookie consent banners and GDPR compliance for EU customers\n\
                - Tracks business licenses and permits expiration\n\
                - Provides hazard and liability disclaimers for food, health, and service businesses",
            allowed_tools: vec!["read", "write", "grep"],
            confidence_threshold: 0.98,
        },
        Department::BusinessAdvisory => DepartmentConfig {
            system_prompt: "Department: Business Advisory — 'The Advisor'\n\
                Acts as a personal business consultant. Analyzes performance and gives actionable advice.\n\
                - Provides weekly plain-language business health reports ('Your top seller was lemonade. Tuesday was your busiest day.')\n\
                - Suggests next actions based on business stage (add products, run a promotion, collect reviews)\n\
                - Identifies seasonal trends and opportunities (back-to-school, holidays, local events)\n\
                - Compares performance to similar businesses (anonymized)\n\
                - Recommends pricing adjustments based on market data\n\
                - Flags unusual patterns that might indicate problems (sudden drop in orders, unusual refund requests)",
            allowed_tools: vec!["read", "write", "websearch", "finance_report"],
            confidence_threshold: 0.85,
        },
    }
}
