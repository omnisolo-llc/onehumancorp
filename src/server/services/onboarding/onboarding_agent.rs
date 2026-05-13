use serde_json::json;
use ::server_ohc::orchestration::{StartOnboardingRequest, StartOnboardingResponse};

#[derive(Clone)]
pub struct OnboardingAgent {
    pub db: std::sync::Arc<crate::db::DB>,
    hub: std::sync::Arc<crate::hub::Hub>,
}

impl OnboardingAgent {
    pub fn new(db: std::sync::Arc<crate::db::DB>, hub: std::sync::Arc<crate::hub::Hub>) -> Self {
        OnboardingAgent { db, hub }
    }

    pub async fn start_onboarding(&self, req: StartOnboardingRequest) -> Result<StartOnboardingResponse, String> {
        let org_id = format!("org-{}", uuid::Uuid::new_v4());

        let business_type = req.business_type.clone();
        let company_name = req.company_name.clone();


        let user_id = format!("usr-{}", uuid::Uuid::new_v4());
        let email = req.admin_email.clone();
        let username = if req.admin_name.is_empty() { email.clone() } else { req.admin_name.clone() };
        let password = req.admin_password.clone();

        let req_first_product_name = req.first_product_name.clone();
        let req_first_product_price = req.first_product_price.clone();
        let req_price_type = req.price_type.clone();

        let swarm_directives = match business_type.as_str() {
            "Legal" => vec![
                "Agent 1 (The Manager): Focus strictly on Legal operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Legal audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Legal client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Legal services.",
                "Agent 5 (The Accountant): Automate Legal-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Legal.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Legal sector."
            ],
            "Medical" => vec![
                "Agent 1 (The Manager): Focus strictly on Medical operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Medical audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Medical client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Medical services.",
                "Agent 5 (The Accountant): Automate Medical-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Medical.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Medical sector."
            ],
            "Accounting" => vec![
                "Agent 1 (The Manager): Focus strictly on Accounting operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Accounting audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Accounting client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Accounting services.",
                "Agent 5 (The Accountant): Automate Accounting-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Accounting.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Accounting sector."
            ],
            "Dental" => vec![
                "Agent 1 (The Manager): Focus strictly on Dental operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Dental audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Dental client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Dental services.",
                "Agent 5 (The Accountant): Automate Dental-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Dental.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Dental sector."
            ],
            "Chiropractic" => vec![
                "Agent 1 (The Manager): Focus strictly on Chiropractic operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Chiropractic audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Chiropractic client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Chiropractic services.",
                "Agent 5 (The Accountant): Automate Chiropractic-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Chiropractic.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Chiropractic sector."
            ],
            "Construction" => vec![
                "Agent 1 (The Manager): Focus strictly on Construction operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Construction audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Construction client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Construction services.",
                "Agent 5 (The Accountant): Automate Construction-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Construction.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Construction sector."
            ],
            "Plumbing" => vec![
                "Agent 1 (The Manager): Focus strictly on Plumbing operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Plumbing audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Plumbing client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Plumbing services.",
                "Agent 5 (The Accountant): Automate Plumbing-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Plumbing.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Plumbing sector."
            ],
            "Electrical" => vec![
                "Agent 1 (The Manager): Focus strictly on Electrical operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Electrical audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Electrical client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Electrical services.",
                "Agent 5 (The Accountant): Automate Electrical-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Electrical.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Electrical sector."
            ],
            "HVAC" => vec![
                "Agent 1 (The Manager): Focus strictly on HVAC operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the HVAC audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to HVAC client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for HVAC services.",
                "Agent 5 (The Accountant): Automate HVAC-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to HVAC.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the HVAC sector."
            ],
            "Roofing" => vec![
                "Agent 1 (The Manager): Focus strictly on Roofing operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Roofing audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Roofing client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Roofing services.",
                "Agent 5 (The Accountant): Automate Roofing-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Roofing.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Roofing sector."
            ],
            "Landscaping" => vec![
                "Agent 1 (The Manager): Focus strictly on Landscaping operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Landscaping audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Landscaping client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Landscaping services.",
                "Agent 5 (The Accountant): Automate Landscaping-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Landscaping.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Landscaping sector."
            ],
            "Cleaning" => vec![
                "Agent 1 (The Manager): Focus strictly on Cleaning operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Cleaning audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Cleaning client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Cleaning services.",
                "Agent 5 (The Accountant): Automate Cleaning-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Cleaning.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Cleaning sector."
            ],
            "Restoration" => vec![
                "Agent 1 (The Manager): Focus strictly on Restoration operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Restoration audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Restoration client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Restoration services.",
                "Agent 5 (The Accountant): Automate Restoration-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Restoration.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Restoration sector."
            ],
            "Security" => vec![
                "Agent 1 (The Manager): Focus strictly on Security operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Security audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Security client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Security services.",
                "Agent 5 (The Accountant): Automate Security-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Security.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Security sector."
            ],
            "IT Services" => vec![
                "Agent 1 (The Manager): Focus strictly on IT Services operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the IT Services audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to IT Services client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for IT Services services.",
                "Agent 5 (The Accountant): Automate IT Services-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to IT Services.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the IT Services sector."
            ],
            "Software" => vec![
                "Agent 1 (The Manager): Focus strictly on Software operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Software audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Software client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Software services.",
                "Agent 5 (The Accountant): Automate Software-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Software.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Software sector."
            ],
            "Marketing" => vec![
                "Agent 1 (The Manager): Focus strictly on Marketing operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Marketing audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Marketing client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Marketing services.",
                "Agent 5 (The Accountant): Automate Marketing-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Marketing.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Marketing sector."
            ],
            "Consulting" => vec![
                "Agent 1 (The Manager): Focus strictly on Consulting operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Consulting audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Consulting client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Consulting services.",
                "Agent 5 (The Accountant): Automate Consulting-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Consulting.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Consulting sector."
            ],
            "Financial" => vec![
                "Agent 1 (The Manager): Focus strictly on Financial operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Financial audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Financial client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Financial services.",
                "Agent 5 (The Accountant): Automate Financial-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Financial.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Financial sector."
            ],
            "Insurance" => vec![
                "Agent 1 (The Manager): Focus strictly on Insurance operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Insurance audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Insurance client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Insurance services.",
                "Agent 5 (The Accountant): Automate Insurance-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Insurance.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Insurance sector."
            ],
            "Real Estate" => vec![
                "Agent 1 (The Manager): Focus strictly on Real Estate operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Real Estate audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Real Estate client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Real Estate services.",
                "Agent 5 (The Accountant): Automate Real Estate-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Real Estate.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Real Estate sector."
            ],
            "Architecture" => vec![
                "Agent 1 (The Manager): Focus strictly on Architecture operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Architecture audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Architecture client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Architecture services.",
                "Agent 5 (The Accountant): Automate Architecture-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Architecture.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Architecture sector."
            ],
            "Engineering" => vec![
                "Agent 1 (The Manager): Focus strictly on Engineering operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Engineering audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Engineering client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Engineering services.",
                "Agent 5 (The Accountant): Automate Engineering-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Engineering.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Engineering sector."
            ],
            "Design" => vec![
                "Agent 1 (The Manager): Focus strictly on Design operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Design audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Design client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Design services.",
                "Agent 5 (The Accountant): Automate Design-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Design.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Design sector."
            ],
            "Photography" => vec![
                "Agent 1 (The Manager): Focus strictly on Photography operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Photography audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Photography client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Photography services.",
                "Agent 5 (The Accountant): Automate Photography-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Photography.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Photography sector."
            ],
            "Videography" => vec![
                "Agent 1 (The Manager): Focus strictly on Videography operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Videography audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Videography client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Videography services.",
                "Agent 5 (The Accountant): Automate Videography-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Videography.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Videography sector."
            ],
            "Music" => vec![
                "Agent 1 (The Manager): Focus strictly on Music operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Music audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Music client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Music services.",
                "Agent 5 (The Accountant): Automate Music-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Music.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Music sector."
            ],
            "Entertainment" => vec![
                "Agent 1 (The Manager): Focus strictly on Entertainment operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Entertainment audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Entertainment client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Entertainment services.",
                "Agent 5 (The Accountant): Automate Entertainment-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Entertainment.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Entertainment sector."
            ],
            "Event Planning" => vec![
                "Agent 1 (The Manager): Focus strictly on Event Planning operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Event Planning audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Event Planning client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Event Planning services.",
                "Agent 5 (The Accountant): Automate Event Planning-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Event Planning.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Event Planning sector."
            ],
            "Catering" => vec![
                "Agent 1 (The Manager): Focus strictly on Catering operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Catering audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Catering client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Catering services.",
                "Agent 5 (The Accountant): Automate Catering-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Catering.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Catering sector."
            ],
            "Bakery" => vec![
                "Agent 1 (The Manager): Focus strictly on Bakery operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Bakery audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Bakery client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Bakery services.",
                "Agent 5 (The Accountant): Automate Bakery-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Bakery.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Bakery sector."
            ],
            "Restaurant" => vec![
                "Agent 1 (The Manager): Focus strictly on Restaurant operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Restaurant audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Restaurant client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Restaurant services.",
                "Agent 5 (The Accountant): Automate Restaurant-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Restaurant.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Restaurant sector."
            ],
            "Cafe" => vec![
                "Agent 1 (The Manager): Focus strictly on Cafe operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Cafe audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Cafe client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Cafe services.",
                "Agent 5 (The Accountant): Automate Cafe-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Cafe.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Cafe sector."
            ],
            "Bar" => vec![
                "Agent 1 (The Manager): Focus strictly on Bar operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Bar audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Bar client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Bar services.",
                "Agent 5 (The Accountant): Automate Bar-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Bar.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Bar sector."
            ],
            "Brewery" => vec![
                "Agent 1 (The Manager): Focus strictly on Brewery operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Brewery audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Brewery client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Brewery services.",
                "Agent 5 (The Accountant): Automate Brewery-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Brewery.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Brewery sector."
            ],
            "Winery" => vec![
                "Agent 1 (The Manager): Focus strictly on Winery operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Winery audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Winery client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Winery services.",
                "Agent 5 (The Accountant): Automate Winery-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Winery.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Winery sector."
            ],
            "Food Truck" => vec![
                "Agent 1 (The Manager): Focus strictly on Food Truck operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Food Truck audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Food Truck client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Food Truck services.",
                "Agent 5 (The Accountant): Automate Food Truck-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Food Truck.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Food Truck sector."
            ],
            "Delivery" => vec![
                "Agent 1 (The Manager): Focus strictly on Delivery operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Delivery audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Delivery client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Delivery services.",
                "Agent 5 (The Accountant): Automate Delivery-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Delivery.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Delivery sector."
            ],
            "Logistics" => vec![
                "Agent 1 (The Manager): Focus strictly on Logistics operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Logistics audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Logistics client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Logistics services.",
                "Agent 5 (The Accountant): Automate Logistics-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Logistics.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Logistics sector."
            ],
            "Transportation" => vec![
                "Agent 1 (The Manager): Focus strictly on Transportation operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Transportation audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Transportation client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Transportation services.",
                "Agent 5 (The Accountant): Automate Transportation-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Transportation.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Transportation sector."
            ],
            "Moving" => vec![
                "Agent 1 (The Manager): Focus strictly on Moving operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Moving audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Moving client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Moving services.",
                "Agent 5 (The Accountant): Automate Moving-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Moving.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Moving sector."
            ],
            "Storage" => vec![
                "Agent 1 (The Manager): Focus strictly on Storage operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Storage audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Storage client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Storage services.",
                "Agent 5 (The Accountant): Automate Storage-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Storage.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Storage sector."
            ],
            "Warehousing" => vec![
                "Agent 1 (The Manager): Focus strictly on Warehousing operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Warehousing audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Warehousing client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Warehousing services.",
                "Agent 5 (The Accountant): Automate Warehousing-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Warehousing.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Warehousing sector."
            ],
            "Retail" => vec![
                "Agent 1 (The Manager): Focus strictly on Retail operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Retail audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Retail client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Retail services.",
                "Agent 5 (The Accountant): Automate Retail-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Retail.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Retail sector."
            ],
            "Ecommerce" => vec![
                "Agent 1 (The Manager): Focus strictly on Ecommerce operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Ecommerce audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Ecommerce client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Ecommerce services.",
                "Agent 5 (The Accountant): Automate Ecommerce-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Ecommerce.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Ecommerce sector."
            ],
            "Wholesale" => vec![
                "Agent 1 (The Manager): Focus strictly on Wholesale operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Wholesale audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Wholesale client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Wholesale services.",
                "Agent 5 (The Accountant): Automate Wholesale-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Wholesale.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Wholesale sector."
            ],
            "Manufacturing" => vec![
                "Agent 1 (The Manager): Focus strictly on Manufacturing operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Manufacturing audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Manufacturing client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Manufacturing services.",
                "Agent 5 (The Accountant): Automate Manufacturing-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Manufacturing.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Manufacturing sector."
            ],
            "Farming" => vec![
                "Agent 1 (The Manager): Focus strictly on Farming operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Farming audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Farming client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Farming services.",
                "Agent 5 (The Accountant): Automate Farming-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Farming.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Farming sector."
            ],
            "Agriculture" => vec![
                "Agent 1 (The Manager): Focus strictly on Agriculture operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Agriculture audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Agriculture client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Agriculture services.",
                "Agent 5 (The Accountant): Automate Agriculture-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Agriculture.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Agriculture sector."
            ],
            "Mining" => vec![
                "Agent 1 (The Manager): Focus strictly on Mining operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Mining audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Mining client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Mining services.",
                "Agent 5 (The Accountant): Automate Mining-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Mining.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Mining sector."
            ],
            "Forestry" => vec![
                "Agent 1 (The Manager): Focus strictly on Forestry operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Forestry audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Forestry client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Forestry services.",
                "Agent 5 (The Accountant): Automate Forestry-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Forestry.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Forestry sector."
            ],
            "Fishing" => vec![
                "Agent 1 (The Manager): Focus strictly on Fishing operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Fishing audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Fishing client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Fishing services.",
                "Agent 5 (The Accountant): Automate Fishing-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Fishing.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Fishing sector."
            ],
            "Fitness" => vec![
                "Agent 1 (The Manager): Focus strictly on Fitness operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Fitness audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Fitness client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Fitness services.",
                "Agent 5 (The Accountant): Automate Fitness-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Fitness.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Fitness sector."
            ],
            "Gym" => vec![
                "Agent 1 (The Manager): Focus strictly on Gym operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Gym audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Gym client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Gym services.",
                "Agent 5 (The Accountant): Automate Gym-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Gym.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Gym sector."
            ],
            "Yoga" => vec![
                "Agent 1 (The Manager): Focus strictly on Yoga operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Yoga audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Yoga client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Yoga services.",
                "Agent 5 (The Accountant): Automate Yoga-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Yoga.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Yoga sector."
            ],
            "Pilates" => vec![
                "Agent 1 (The Manager): Focus strictly on Pilates operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Pilates audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Pilates client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Pilates services.",
                "Agent 5 (The Accountant): Automate Pilates-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Pilates.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Pilates sector."
            ],
            "Martial Arts" => vec![
                "Agent 1 (The Manager): Focus strictly on Martial Arts operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Martial Arts audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Martial Arts client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Martial Arts services.",
                "Agent 5 (The Accountant): Automate Martial Arts-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Martial Arts.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Martial Arts sector."
            ],
            "Sports" => vec![
                "Agent 1 (The Manager): Focus strictly on Sports operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Sports audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Sports client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Sports services.",
                "Agent 5 (The Accountant): Automate Sports-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Sports.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Sports sector."
            ],
            "Recreation" => vec![
                "Agent 1 (The Manager): Focus strictly on Recreation operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Recreation audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Recreation client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Recreation services.",
                "Agent 5 (The Accountant): Automate Recreation-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Recreation.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Recreation sector."
            ],
            "Education" => vec![
                "Agent 1 (The Manager): Focus strictly on Education operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Education audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Education client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Education services.",
                "Agent 5 (The Accountant): Automate Education-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Education.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Education sector."
            ],
            "Tutoring" => vec![
                "Agent 1 (The Manager): Focus strictly on Tutoring operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Tutoring audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Tutoring client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Tutoring services.",
                "Agent 5 (The Accountant): Automate Tutoring-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Tutoring.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Tutoring sector."
            ],
            "School" => vec![
                "Agent 1 (The Manager): Focus strictly on School operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the School audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to School client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for School services.",
                "Agent 5 (The Accountant): Automate School-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to School.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the School sector."
            ],
            "Daycare" => vec![
                "Agent 1 (The Manager): Focus strictly on Daycare operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Daycare audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Daycare client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Daycare services.",
                "Agent 5 (The Accountant): Automate Daycare-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Daycare.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Daycare sector."
            ],
            "Childcare" => vec![
                "Agent 1 (The Manager): Focus strictly on Childcare operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Childcare audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Childcare client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Childcare services.",
                "Agent 5 (The Accountant): Automate Childcare-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Childcare.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Childcare sector."
            ],
            "Pet Care" => vec![
                "Agent 1 (The Manager): Focus strictly on Pet Care operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Pet Care audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Pet Care client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Pet Care services.",
                "Agent 5 (The Accountant): Automate Pet Care-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Pet Care.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Pet Care sector."
            ],
            "Veterinary" => vec![
                "Agent 1 (The Manager): Focus strictly on Veterinary operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Veterinary audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Veterinary client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Veterinary services.",
                "Agent 5 (The Accountant): Automate Veterinary-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Veterinary.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Veterinary sector."
            ],
            "Grooming" => vec![
                "Agent 1 (The Manager): Focus strictly on Grooming operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Grooming audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Grooming client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Grooming services.",
                "Agent 5 (The Accountant): Automate Grooming-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Grooming.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Grooming sector."
            ],
            "Training" => vec![
                "Agent 1 (The Manager): Focus strictly on Training operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Training audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Training client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Training services.",
                "Agent 5 (The Accountant): Automate Training-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Training.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Training sector."
            ],
            "Boarding" => vec![
                "Agent 1 (The Manager): Focus strictly on Boarding operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Boarding audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Boarding client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Boarding services.",
                "Agent 5 (The Accountant): Automate Boarding-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Boarding.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Boarding sector."
            ],
            "Beauty" => vec![
                "Agent 1 (The Manager): Focus strictly on Beauty operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Beauty audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Beauty client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Beauty services.",
                "Agent 5 (The Accountant): Automate Beauty-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Beauty.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Beauty sector."
            ],
            "Salon" => vec![
                "Agent 1 (The Manager): Focus strictly on Salon operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Salon audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Salon client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Salon services.",
                "Agent 5 (The Accountant): Automate Salon-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Salon.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Salon sector."
            ],
            "Spa" => vec![
                "Agent 1 (The Manager): Focus strictly on Spa operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Spa audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Spa client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Spa services.",
                "Agent 5 (The Accountant): Automate Spa-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Spa.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Spa sector."
            ],
            "Massage" => vec![
                "Agent 1 (The Manager): Focus strictly on Massage operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Massage audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Massage client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Massage services.",
                "Agent 5 (The Accountant): Automate Massage-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Massage.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Massage sector."
            ],
            "Therapy" => vec![
                "Agent 1 (The Manager): Focus strictly on Therapy operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Therapy audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Therapy client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Therapy services.",
                "Agent 5 (The Accountant): Automate Therapy-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Therapy.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Therapy sector."
            ],
            "Wellness" => vec![
                "Agent 1 (The Manager): Focus strictly on Wellness operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Wellness audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Wellness client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Wellness services.",
                "Agent 5 (The Accountant): Automate Wellness-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Wellness.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Wellness sector."
            ],
            "Healthcare" => vec![
                "Agent 1 (The Manager): Focus strictly on Healthcare operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Healthcare audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Healthcare client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Healthcare services.",
                "Agent 5 (The Accountant): Automate Healthcare-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Healthcare.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Healthcare sector."
            ],
            "Pharmacy" => vec![
                "Agent 1 (The Manager): Focus strictly on Pharmacy operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Pharmacy audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Pharmacy client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Pharmacy services.",
                "Agent 5 (The Accountant): Automate Pharmacy-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Pharmacy.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Pharmacy sector."
            ],
            "Optometry" => vec![
                "Agent 1 (The Manager): Focus strictly on Optometry operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Optometry audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Optometry client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Optometry services.",
                "Agent 5 (The Accountant): Automate Optometry-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Optometry.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Optometry sector."
            ],
            "Vision" => vec![
                "Agent 1 (The Manager): Focus strictly on Vision operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Vision audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Vision client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Vision services.",
                "Agent 5 (The Accountant): Automate Vision-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Vision.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Vision sector."
            ],
            "Audiology" => vec![
                "Agent 1 (The Manager): Focus strictly on Audiology operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Audiology audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Audiology client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Audiology services.",
                "Agent 5 (The Accountant): Automate Audiology-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Audiology.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Audiology sector."
            ],
            "Hearing" => vec![
                "Agent 1 (The Manager): Focus strictly on Hearing operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Hearing audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Hearing client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Hearing services.",
                "Agent 5 (The Accountant): Automate Hearing-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Hearing.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Hearing sector."
            ],
            "Mental Health" => vec![
                "Agent 1 (The Manager): Focus strictly on Mental Health operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Mental Health audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Mental Health client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Mental Health services.",
                "Agent 5 (The Accountant): Automate Mental Health-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Mental Health.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Mental Health sector."
            ],
            "Counseling" => vec![
                "Agent 1 (The Manager): Focus strictly on Counseling operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Counseling audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Counseling client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Counseling services.",
                "Agent 5 (The Accountant): Automate Counseling-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Counseling.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Counseling sector."
            ],
            "Psychology" => vec![
                "Agent 1 (The Manager): Focus strictly on Psychology operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Psychology audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Psychology client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Psychology services.",
                "Agent 5 (The Accountant): Automate Psychology-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Psychology.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Psychology sector."
            ],
            "Psychiatry" => vec![
                "Agent 1 (The Manager): Focus strictly on Psychiatry operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Psychiatry audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Psychiatry client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Psychiatry services.",
                "Agent 5 (The Accountant): Automate Psychiatry-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Psychiatry.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Psychiatry sector."
            ],
            "Coaching" => vec![
                "Agent 1 (The Manager): Focus strictly on Coaching operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Coaching audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Coaching client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Coaching services.",
                "Agent 5 (The Accountant): Automate Coaching-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Coaching.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Coaching sector."
            ],
            "Social Work" => vec![
                "Agent 1 (The Manager): Focus strictly on Social Work operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Social Work audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Social Work client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Social Work services.",
                "Agent 5 (The Accountant): Automate Social Work-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Social Work.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Social Work sector."
            ],
            "Nonprofit" => vec![
                "Agent 1 (The Manager): Focus strictly on Nonprofit operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Nonprofit audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Nonprofit client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Nonprofit services.",
                "Agent 5 (The Accountant): Automate Nonprofit-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Nonprofit.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Nonprofit sector."
            ],
            "Charity" => vec![
                "Agent 1 (The Manager): Focus strictly on Charity operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Charity audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Charity client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Charity services.",
                "Agent 5 (The Accountant): Automate Charity-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Charity.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Charity sector."
            ],
            "Foundation" => vec![
                "Agent 1 (The Manager): Focus strictly on Foundation operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Foundation audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Foundation client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Foundation services.",
                "Agent 5 (The Accountant): Automate Foundation-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Foundation.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Foundation sector."
            ],
            "Association" => vec![
                "Agent 1 (The Manager): Focus strictly on Association operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Association audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Association client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Association services.",
                "Agent 5 (The Accountant): Automate Association-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Association.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Association sector."
            ],
            "Club" => vec![
                "Agent 1 (The Manager): Focus strictly on Club operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Club audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Club client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Club services.",
                "Agent 5 (The Accountant): Automate Club-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Club.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Club sector."
            ],
            "Organization" => vec![
                "Agent 1 (The Manager): Focus strictly on Organization operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Organization audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Organization client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Organization services.",
                "Agent 5 (The Accountant): Automate Organization-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Organization.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Organization sector."
            ],
            "Government" => vec![
                "Agent 1 (The Manager): Focus strictly on Government operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Government audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Government client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Government services.",
                "Agent 5 (The Accountant): Automate Government-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Government.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Government sector."
            ],
            "Public Sector" => vec![
                "Agent 1 (The Manager): Focus strictly on Public Sector operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Public Sector audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Public Sector client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Public Sector services.",
                "Agent 5 (The Accountant): Automate Public Sector-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Public Sector.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Public Sector sector."
            ],
            "Politics" => vec![
                "Agent 1 (The Manager): Focus strictly on Politics operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Politics audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Politics client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Politics services.",
                "Agent 5 (The Accountant): Automate Politics-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Politics.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Politics sector."
            ],
            "Campaign" => vec![
                "Agent 1 (The Manager): Focus strictly on Campaign operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Campaign audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Campaign client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Campaign services.",
                "Agent 5 (The Accountant): Automate Campaign-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Campaign.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Campaign sector."
            ],
            "Advocacy" => vec![
                "Agent 1 (The Manager): Focus strictly on Advocacy operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Advocacy audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Advocacy client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Advocacy services.",
                "Agent 5 (The Accountant): Automate Advocacy-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Advocacy.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Advocacy sector."
            ],
            "Media" => vec![
                "Agent 1 (The Manager): Focus strictly on Media operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Media audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Media client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Media services.",
                "Agent 5 (The Accountant): Automate Media-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Media.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Media sector."
            ],
            "Publishing" => vec![
                "Agent 1 (The Manager): Focus strictly on Publishing operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Publishing audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Publishing client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Publishing services.",
                "Agent 5 (The Accountant): Automate Publishing-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Publishing.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Publishing sector."
            ],
            "Broadcasting" => vec![
                "Agent 1 (The Manager): Focus strictly on Broadcasting operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Broadcasting audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Broadcasting client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Broadcasting services.",
                "Agent 5 (The Accountant): Automate Broadcasting-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Broadcasting.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Broadcasting sector."
            ],
            "Journalism" => vec![
                "Agent 1 (The Manager): Focus strictly on Journalism operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Journalism audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Journalism client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Journalism services.",
                "Agent 5 (The Accountant): Automate Journalism-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Journalism.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Journalism sector."
            ],
            "News" => vec![
                "Agent 1 (The Manager): Focus strictly on News operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the News audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to News client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for News services.",
                "Agent 5 (The Accountant): Automate News-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to News.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the News sector."
            ],
            "Advertising" => vec![
                "Agent 1 (The Manager): Focus strictly on Advertising operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Advertising audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Advertising client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Advertising services.",
                "Agent 5 (The Accountant): Automate Advertising-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Advertising.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Advertising sector."
            ],
            "PR" => vec![
                "Agent 1 (The Manager): Focus strictly on PR operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the PR audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to PR client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for PR services.",
                "Agent 5 (The Accountant): Automate PR-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to PR.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the PR sector."
            ],
            "Communications" => vec![
                "Agent 1 (The Manager): Focus strictly on Communications operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Communications audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Communications client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Communications services.",
                "Agent 5 (The Accountant): Automate Communications-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Communications.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Communications sector."
            ],
            "Telecommunications" => vec![
                "Agent 1 (The Manager): Focus strictly on Telecommunications operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Telecommunications audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Telecommunications client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Telecommunications services.",
                "Agent 5 (The Accountant): Automate Telecommunications-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Telecommunications.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Telecommunications sector."
            ],
            "Internet" => vec![
                "Agent 1 (The Manager): Focus strictly on Internet operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Internet audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Internet client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Internet services.",
                "Agent 5 (The Accountant): Automate Internet-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Internet.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Internet sector."
            ],
            "ISP" => vec![
                "Agent 1 (The Manager): Focus strictly on ISP operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the ISP audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to ISP client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for ISP services.",
                "Agent 5 (The Accountant): Automate ISP-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to ISP.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the ISP sector."
            ],
            "Hosting" => vec![
                "Agent 1 (The Manager): Focus strictly on Hosting operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Hosting audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Hosting client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Hosting services.",
                "Agent 5 (The Accountant): Automate Hosting-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Hosting.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Hosting sector."
            ],
            "Cloud" => vec![
                "Agent 1 (The Manager): Focus strictly on Cloud operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Cloud audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Cloud client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Cloud services.",
                "Agent 5 (The Accountant): Automate Cloud-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Cloud.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Cloud sector."
            ],
            "SaaS" => vec![
                "Agent 1 (The Manager): Focus strictly on SaaS operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the SaaS audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to SaaS client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for SaaS services.",
                "Agent 5 (The Accountant): Automate SaaS-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to SaaS.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the SaaS sector."
            ],
            "PaaS" => vec![
                "Agent 1 (The Manager): Focus strictly on PaaS operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the PaaS audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to PaaS client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for PaaS services.",
                "Agent 5 (The Accountant): Automate PaaS-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to PaaS.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the PaaS sector."
            ],
            "IaaS" => vec![
                "Agent 1 (The Manager): Focus strictly on IaaS operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the IaaS audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to IaaS client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for IaaS services.",
                "Agent 5 (The Accountant): Automate IaaS-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to IaaS.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the IaaS sector."
            ],
            "Hardware" => vec![
                "Agent 1 (The Manager): Focus strictly on Hardware operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Hardware audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Hardware client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Hardware services.",
                "Agent 5 (The Accountant): Automate Hardware-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Hardware.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Hardware sector."
            ],
            "Electronics" => vec![
                "Agent 1 (The Manager): Focus strictly on Electronics operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Electronics audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Electronics client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Electronics services.",
                "Agent 5 (The Accountant): Automate Electronics-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Electronics.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Electronics sector."
            ],
            "Apparel" => vec![
                "Agent 1 (The Manager): Focus strictly on Apparel operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Apparel audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Apparel client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Apparel services.",
                "Agent 5 (The Accountant): Automate Apparel-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Apparel.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Apparel sector."
            ],
            "Fashion" => vec![
                "Agent 1 (The Manager): Focus strictly on Fashion operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Fashion audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Fashion client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Fashion services.",
                "Agent 5 (The Accountant): Automate Fashion-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Fashion.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Fashion sector."
            ],
            "Clothing" => vec![
                "Agent 1 (The Manager): Focus strictly on Clothing operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Clothing audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Clothing client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Clothing services.",
                "Agent 5 (The Accountant): Automate Clothing-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Clothing.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Clothing sector."
            ],
            "Shoes" => vec![
                "Agent 1 (The Manager): Focus strictly on Shoes operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Shoes audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Shoes client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Shoes services.",
                "Agent 5 (The Accountant): Automate Shoes-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Shoes.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Shoes sector."
            ],
            "Accessories" => vec![
                "Agent 1 (The Manager): Focus strictly on Accessories operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Accessories audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Accessories client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Accessories services.",
                "Agent 5 (The Accountant): Automate Accessories-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Accessories.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Accessories sector."
            ],
            "Jewelry" => vec![
                "Agent 1 (The Manager): Focus strictly on Jewelry operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Jewelry audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Jewelry client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Jewelry services.",
                "Agent 5 (The Accountant): Automate Jewelry-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Jewelry.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Jewelry sector."
            ],
            "Cosmetics" => vec![
                "Agent 1 (The Manager): Focus strictly on Cosmetics operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Cosmetics audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Cosmetics client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Cosmetics services.",
                "Agent 5 (The Accountant): Automate Cosmetics-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Cosmetics.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Cosmetics sector."
            ],
            "Beauty Products" => vec![
                "Agent 1 (The Manager): Focus strictly on Beauty Products operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Beauty Products audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Beauty Products client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Beauty Products services.",
                "Agent 5 (The Accountant): Automate Beauty Products-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Beauty Products.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Beauty Products sector."
            ],
            "Skincare" => vec![
                "Agent 1 (The Manager): Focus strictly on Skincare operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Skincare audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Skincare client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Skincare services.",
                "Agent 5 (The Accountant): Automate Skincare-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Skincare.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Skincare sector."
            ],
            "Haircare" => vec![
                "Agent 1 (The Manager): Focus strictly on Haircare operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Haircare audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Haircare client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Haircare services.",
                "Agent 5 (The Accountant): Automate Haircare-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Haircare.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Haircare sector."
            ],
            "Fragrance" => vec![
                "Agent 1 (The Manager): Focus strictly on Fragrance operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Fragrance audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Fragrance client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Fragrance services.",
                "Agent 5 (The Accountant): Automate Fragrance-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Fragrance.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Fragrance sector."
            ],
            "Home Goods" => vec![
                "Agent 1 (The Manager): Focus strictly on Home Goods operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Home Goods audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Home Goods client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Home Goods services.",
                "Agent 5 (The Accountant): Automate Home Goods-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Home Goods.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Home Goods sector."
            ],
            "Furniture" => vec![
                "Agent 1 (The Manager): Focus strictly on Furniture operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Furniture audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Furniture client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Furniture services.",
                "Agent 5 (The Accountant): Automate Furniture-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Furniture.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Furniture sector."
            ],
            "Decor" => vec![
                "Agent 1 (The Manager): Focus strictly on Decor operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Decor audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Decor client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Decor services.",
                "Agent 5 (The Accountant): Automate Decor-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Decor.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Decor sector."
            ],
            "Appliances" => vec![
                "Agent 1 (The Manager): Focus strictly on Appliances operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Appliances audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Appliances client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Appliances services.",
                "Agent 5 (The Accountant): Automate Appliances-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Appliances.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Appliances sector."
            ],
            "Kitchen" => vec![
                "Agent 1 (The Manager): Focus strictly on Kitchen operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Kitchen audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Kitchen client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Kitchen services.",
                "Agent 5 (The Accountant): Automate Kitchen-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Kitchen.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Kitchen sector."
            ],
            "Bath" => vec![
                "Agent 1 (The Manager): Focus strictly on Bath operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Bath audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Bath client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Bath services.",
                "Agent 5 (The Accountant): Automate Bath-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Bath.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Bath sector."
            ],
            "Bedding" => vec![
                "Agent 1 (The Manager): Focus strictly on Bedding operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Bedding audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Bedding client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Bedding services.",
                "Agent 5 (The Accountant): Automate Bedding-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Bedding.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Bedding sector."
            ],
            "Linens" => vec![
                "Agent 1 (The Manager): Focus strictly on Linens operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Linens audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Linens client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Linens services.",
                "Agent 5 (The Accountant): Automate Linens-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Linens.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Linens sector."
            ],
            "Textiles" => vec![
                "Agent 1 (The Manager): Focus strictly on Textiles operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Textiles audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Textiles client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Textiles services.",
                "Agent 5 (The Accountant): Automate Textiles-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Textiles.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Textiles sector."
            ],
            "Rugs" => vec![
                "Agent 1 (The Manager): Focus strictly on Rugs operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Rugs audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Rugs client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Rugs services.",
                "Agent 5 (The Accountant): Automate Rugs-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Rugs.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Rugs sector."
            ],
            "Carpets" => vec![
                "Agent 1 (The Manager): Focus strictly on Carpets operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Carpets audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Carpets client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Carpets services.",
                "Agent 5 (The Accountant): Automate Carpets-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Carpets.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Carpets sector."
            ],
            "Flooring" => vec![
                "Agent 1 (The Manager): Focus strictly on Flooring operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Flooring audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Flooring client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Flooring services.",
                "Agent 5 (The Accountant): Automate Flooring-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Flooring.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Flooring sector."
            ],
            "Tile" => vec![
                "Agent 1 (The Manager): Focus strictly on Tile operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Tile audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Tile client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Tile services.",
                "Agent 5 (The Accountant): Automate Tile-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Tile.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Tile sector."
            ],
            "Lighting" => vec![
                "Agent 1 (The Manager): Focus strictly on Lighting operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Lighting audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Lighting client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Lighting services.",
                "Agent 5 (The Accountant): Automate Lighting-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Lighting.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Lighting sector."
            ],
            "Lamps" => vec![
                "Agent 1 (The Manager): Focus strictly on Lamps operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Lamps audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Lamps client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Lamps services.",
                "Agent 5 (The Accountant): Automate Lamps-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Lamps.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Lamps sector."
            ],
            "Fixtures" => vec![
                "Agent 1 (The Manager): Focus strictly on Fixtures operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Fixtures audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Fixtures client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Fixtures services.",
                "Agent 5 (The Accountant): Automate Fixtures-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Fixtures.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Fixtures sector."
            ],
            "Hardware Store" => vec![
                "Agent 1 (The Manager): Focus strictly on Hardware Store operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Hardware Store audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Hardware Store client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Hardware Store services.",
                "Agent 5 (The Accountant): Automate Hardware Store-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Hardware Store.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Hardware Store sector."
            ],
            "Tools" => vec![
                "Agent 1 (The Manager): Focus strictly on Tools operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Tools audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Tools client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Tools services.",
                "Agent 5 (The Accountant): Automate Tools-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Tools.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Tools sector."
            ],
            "Lumber" => vec![
                "Agent 1 (The Manager): Focus strictly on Lumber operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Lumber audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Lumber client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Lumber services.",
                "Agent 5 (The Accountant): Automate Lumber-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Lumber.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Lumber sector."
            ],
            "Building Materials" => vec![
                "Agent 1 (The Manager): Focus strictly on Building Materials operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Building Materials audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Building Materials client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Building Materials services.",
                "Agent 5 (The Accountant): Automate Building Materials-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Building Materials.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Building Materials sector."
            ],
            "Paint" => vec![
                "Agent 1 (The Manager): Focus strictly on Paint operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Paint audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Paint client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Paint services.",
                "Agent 5 (The Accountant): Automate Paint-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Paint.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Paint sector."
            ],
            "Wallpaper" => vec![
                "Agent 1 (The Manager): Focus strictly on Wallpaper operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Wallpaper audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Wallpaper client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Wallpaper services.",
                "Agent 5 (The Accountant): Automate Wallpaper-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Wallpaper.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Wallpaper sector."
            ],
            "Supplies" => vec![
                "Agent 1 (The Manager): Focus strictly on Supplies operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Supplies audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Supplies client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Supplies services.",
                "Agent 5 (The Accountant): Automate Supplies-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Supplies.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Supplies sector."
            ],
            "Garden" => vec![
                "Agent 1 (The Manager): Focus strictly on Garden operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Garden audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Garden client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Garden services.",
                "Agent 5 (The Accountant): Automate Garden-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Garden.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Garden sector."
            ],
            "Plants" => vec![
                "Agent 1 (The Manager): Focus strictly on Plants operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Plants audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Plants client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Plants services.",
                "Agent 5 (The Accountant): Automate Plants-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Plants.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Plants sector."
            ],
            "Nursery" => vec![
                "Agent 1 (The Manager): Focus strictly on Nursery operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Nursery audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Nursery client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Nursery services.",
                "Agent 5 (The Accountant): Automate Nursery-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Nursery.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Nursery sector."
            ],
            "Landscaping Supplies" => vec![
                "Agent 1 (The Manager): Focus strictly on Landscaping Supplies operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Landscaping Supplies audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Landscaping Supplies client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Landscaping Supplies services.",
                "Agent 5 (The Accountant): Automate Landscaping Supplies-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Landscaping Supplies.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Landscaping Supplies sector."
            ],
            "Outdoor" => vec![
                "Agent 1 (The Manager): Focus strictly on Outdoor operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Outdoor audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Outdoor client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Outdoor services.",
                "Agent 5 (The Accountant): Automate Outdoor-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Outdoor.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Outdoor sector."
            ],
            "Patio" => vec![
                "Agent 1 (The Manager): Focus strictly on Patio operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Patio audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Patio client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Patio services.",
                "Agent 5 (The Accountant): Automate Patio-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Patio.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Patio sector."
            ],
            "Deck" => vec![
                "Agent 1 (The Manager): Focus strictly on Deck operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Deck audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Deck client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Deck services.",
                "Agent 5 (The Accountant): Automate Deck-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Deck.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Deck sector."
            ],
            "Pool" => vec![
                "Agent 1 (The Manager): Focus strictly on Pool operational workflows, resource allocation, and daily schedule management.",
                "Agent 2 (The Promoter): Optimize all marketing copy for the Pool audience. Avoid jargon and focus on the Grandmother Test.",
                "Agent 3 (The Salesperson): Handle inbound leads with empathy tailored to Pool client needs.",
                "Agent 4 (The Ambassador): Provide customer success tracking for Pool services.",
                "Agent 5 (The Accountant): Automate Pool-specific invoicing and reconciliation.",
                "Agent 6 (The Protector): Monitor compliance and legal disclaimers specific to Pool.",
                "Agent 7 (The Advisor): Provide strategic growth forecasting for the Pool sector."
            ],
            _ => vec![
                "Standard operational workflow.",
                "Standard marketing.",
                "Standard sales.",
                "Standard customer success.",
                "Standard finance.",
                "Standard legal.",
                "Standard advisory."
            ]
        };

        let org_id_clone1 = org_id.clone();
        let org_id_clone2 = org_id.clone();
        let business_type_clone = business_type.clone();

        let agent_clone_product = self.clone();
        let product_future = tokio::task::spawn(async move {
            if !req_first_product_name.is_empty() {
                agent_clone_product.create_product(&org_id_clone1, &req_first_product_name, &req_first_product_price, &req_price_type, &business_type_clone).await
            } else {
                agent_clone_product.generate_initial_products(&org_id_clone1, &business_type_clone).await
            }
        });

        let agent_clone_seed = self.clone();
        let seed_future = tokio::task::spawn(async move {
            agent_clone_seed.seed_default_agents(&org_id_clone2).await
        });

        let org_id_clone3 = org_id.clone();
        let pool = self.db.pool.clone();
        let publish_events_future = tokio::task::spawn(async move {
            // Subscribe default AI Agents to specific tenant events dynamically
            let event_topics = vec![
                ("The Manager", "tenant.booking.created"),
                ("The Manager", "tenant.order.placed"),
                ("The Promoter", "tenant.product.created"),
                ("The Salesperson", "tenant.lead.created"),
                ("The Ambassador", "tenant.message.received"),
                ("The Accountant", "tenant.payment.success"),
                ("The Protector", "tenant.contract.signed"),
                ("The Advisor", "tenant.report.generated"),
            ];

            for (agent_role, topic) in event_topics {
                let _ = sqlx::query("INSERT INTO agent_event_subscriptions (tenant_id, agent_role, topic) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING")
                    .bind(&org_id_clone3)
                    .bind(agent_role)
                    .bind(topic)
                    .execute(&pool)
                    .await;
            }
            Ok::<(), String>(())
        });

        let hash_future = tokio::task::spawn(async move {
            if !password.is_empty() {
                tokio::task::spawn_blocking(move || {
                    bcrypt::hash(&password, if cfg!(test) { 4 } else { bcrypt::DEFAULT_COST }).map_err(|e| format!("Failed to hash password: {}", e))
                }).await.map_err(|e| e.to_string())?
            } else {
                Ok("".to_string())
            }
        });

        let (product_res_res, seed_res_res, _events_res_res, hash_res_res) = tokio::join!(product_future, seed_future, publish_events_future, hash_future);

        let product_res = product_res_res.unwrap_or_else(|e| Err(e.to_string()));
        let seed_res = seed_res_res.unwrap_or_else(|e| Err(e.to_string()));
        let hash_res = hash_res_res.unwrap_or_else(|e| Err(e.to_string()));

        product_res?;
        seed_res?;
        let password_hash = hash_res?;

        let roles_json = serde_json::to_string(&vec!["admin"]).unwrap_or_default();
        let now = chrono::Utc::now();
        let oidc_subject = "";

        sqlx::query(
            r#"
            INSERT INTO users (id, username, email, password_hash, roles, active, organization_id, oidc_subject, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#
        )
        .bind(&user_id)
        .bind(&username)
        .bind(&email)
        .bind(&password_hash)
        .bind(&roles_json)
        .bind(true)
        .bind(&org_id)
        .bind(&oidc_subject)
        .bind(now)
        .bind(now)
        .execute(&self.db.pool)
        .await
        .map_err(|e| e.to_string())?;

        // Extract feature flags logic
        let mut flags = serde_json::Map::new();
        if business_type == "Service Business" || business_type == "Service" || req.selling_categories.contains(&"services".to_string()) {
            flags.insert("enable_booking".to_string(), serde_json::json!(true));
        }
        if business_type == "Restaurant / Food" || business_type == "Food Cart" || req.selling_categories.contains(&"food".to_string()) {
            flags.insert("enable_menu".to_string(), serde_json::json!(true));
            flags.insert("enable_pre_order".to_string(), serde_json::json!(true));
        }
        if req.selling_categories.contains(&"physical".to_string()) || req.selling_categories.contains(&"digital".to_string()) {
            flags.insert("enable_ecommerce".to_string(), serde_json::json!(true));
        }
        if req.selling_categories.contains(&"subscriptions".to_string()) {
            flags.insert("enable_subscriptions".to_string(), serde_json::json!(true));
        }

        let flags_json = serde_json::Value::Object(flags);

        sqlx::query(
            "INSERT INTO onboarding_state (tenant_id, organization_id, user_id, current_step, state_json) VALUES ($1, $2, $3, $4, $5)"
        )
        .bind(&org_id)
        .bind(&org_id)
        .bind(&user_id)
        .bind(1)
        .bind(flags_json)
        .execute(&self.db.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(StartOnboardingResponse {
            success: true,
            message: format!("Successfully onboarded {} as a {}!", company_name, business_type),
            organization_id: org_id,
        })
    }

    async fn create_product(&self, org_id: &str, name: &str, price_str: &str, price_type: &str, business_type: &str) -> Result<(), String> {
        let price_cents = (price_str.parse::<f64>().unwrap_or(0.0) * 100.0) as i64;
        let strategy = match business_type {
            "Service Business" => "booking",
            _ => "physical",
        };

        let id = format!("prod-{}", uuid::Uuid::new_v4());
        sqlx::query("INSERT INTO products (id, organization_id, name, description, price_cents, fulfillment_strategy, metadata) VALUES ($1, $2, $3, $4, $5, $6, $7)")
            .bind(&id)
            .bind(org_id)
            .bind(name)
            .bind("Added during onboarding")
            .bind(price_cents)
            .bind(strategy)
            .bind(json!({"price_type": price_type}))
            .execute(&self.db.pool)
            .await
            .map_err(|e| e.to_string())?;

        let event_payload = json!({
            "product_id": id,
            "name": name,
            "organization_id": org_id,
        });

        let event = ::server_ohc::orchestration::TeammateMeshEvent {
            agent_id: "system".to_string(),
            action: "ProductCreated".to_string(),
            status: "success".to_string(),
            payload: serde_json::to_vec(&event_payload).unwrap_or_default(),
            msg_id: uuid::Uuid::new_v4().to_string(),
        };

        let _ = self.hub.publish_teammate_event("products_inbox".to_string(), event);

        Ok(())
    }

    async fn generate_initial_products(&self, org_id: &str, business_type: &str) -> Result<(), String> {
        let products = match business_type {
            "Online Store" => vec![
                ("Standard Product", "A great product for your store", 1999, "physical"),
                ("Premium Product", "A premium offering", 4999, "physical"),
            ],
            "Service Business" => vec![
                ("Consultation", "1-hour professional consultation", 10000, "booking"),
                ("Service Call", "On-site service visit", 7500, "booking"),
            ],
            "Restaurant / Food" => vec![
                ("House Special", "Our most popular dish", 1599, "physical"),
                ("Drink of the Day", "Refreshing beverage", 450, "physical"),
            ],
            _ => vec![
                ("Default Item", "Welcome to your new business", 1000, "physical"),
            ],
        };

        let mut futures = vec![];
        for (name, desc, price, strategy) in products {
            let id = format!("prod-{}", uuid::Uuid::new_v4());
            let org_id = org_id.to_string();
            let name = name.to_string();
            let desc = desc.to_string();
            let strategy = strategy.to_string();
            let pool = self.db.pool.clone();

            let hub = self.hub.clone();
            futures.push(tokio::spawn(async move {
                sqlx::query("INSERT INTO products (id, organization_id, name, description, price_cents, fulfillment_strategy, metadata) VALUES ($1, $2, $3, $4, $5, $6, $7)")
                    .bind(&id)
                    .bind(&org_id)
                    .bind(&name)
                    .bind(&desc)
                    .bind(price)
                    .bind(&strategy)
                    .bind(json!({}))
                    .execute(&pool)
                    .await?;

                let event_payload = json!({
                    "product_id": id,
                    "name": name,
                    "organization_id": org_id,
                });

                let event = ::server_ohc::orchestration::TeammateMeshEvent {
                    agent_id: "system".to_string(),
                    action: "ProductCreated".to_string(),
                    status: "success".to_string(),
                    payload: serde_json::to_vec(&event_payload).unwrap_or_default(),
                    msg_id: uuid::Uuid::new_v4().to_string(),
                };

                let _ = hub.publish_teammate_event("products_inbox".to_string(), event);
                Ok::<_, sqlx::Error>(())
            }));
        }

        for f in futures {
            f.await.map_err(|e| e.to_string())?.map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    async fn seed_default_agents(&self, org_id: &str) -> Result<(), String> {
        let default_agents = vec![
            ("Operations", "The Manager", "Operations"),
            ("Marketing & Advertising", "The Promoter", "Marketing"),
            ("Sales & Acquisition", "The Salesperson", "Sales"),
            ("Customer Success", "The Ambassador", "CustomerSuccess"),
            ("Finance & Payments", "The Accountant", "Finance"),
            ("Legal & Compliance", "The Protector", "Legal"),
            ("Business Advisory", "The Advisor", "Advisory"),
        ];

        for (name, role, role_id) in default_agents {
            let id = format!("{}-{}", org_id, role_id.to_lowercase());
            sqlx::query("INSERT INTO agents (id, name, role, organization_id, status, provider_type, is_default) VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT (id) DO UPDATE SET name = EXCLUDED.name, role = EXCLUDED.role, status = EXCLUDED.status")
                .bind(id)
                .bind(name)
                .bind(role)
                .bind(org_id)
                .bind("IDLE")
                .bind("builtin")
                .bind(true)
                .execute(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use crate::db::DB;
    use ::server_ohc::orchestration::StartOnboardingRequest;

    async fn setup_test_db() -> Option<Arc<DB>> {
        let _ = std::env::var("DATABASE_URL").ok()?;
        unsafe {
            std::env::set_var("OHC_SQLITE_KEY", "test-fallback-key");
        }
        let db = Arc::new(DB::new().await.ok()?);
        Some(db)
    }

    #[tokio::test]
    async fn test_start_onboarding_online_store() {
        let db = match setup_test_db().await {
            Some(db) => db,
            None => return,
        };
        let (tx, _) = tokio::sync::mpsc::channel(10);
        let hub = std::sync::Arc::new(crate::hub::Hub::new(tx, db.pool.clone()));
        let agent = OnboardingAgent::new(db, hub);

        let req = StartOnboardingRequest {
            business_type: "Online Store".to_string(),
            company_name: "Test Store".to_string(),
            company_description: "A test store".to_string(),
            selling_categories: vec!["physical".to_string(), "digital".to_string()],
            payment_pref: "online".to_string(),
            admin_email: "admin@test.com".to_string(),
            admin_name: "Admin User".to_string(),
            admin_password: "password123".to_string(),
            website_template: "Modern".to_string(),
            first_product_name: "Cake".to_string(),
            first_product_price: "25.00".to_string(),
            domain_choice: "subdomain".to_string(),
            price_type: "fixed".to_string(),
        };

        let req_categories = req.selling_categories.clone();
        assert_eq!(req_categories.len(), 2);
        assert_eq!(req_categories[0], "physical");

        let res = agent.start_onboarding(req).await;
        assert!(res.is_ok());
        let resp = res.unwrap();
        assert!(resp.success);
        assert!(!resp.organization_id.is_empty());

        let org_id = resp.organization_id;
        use sqlx::Row;
        let agents = sqlx::query("SELECT id, name, role FROM agents WHERE organization_id = $1 AND is_default = TRUE")
            .bind(&org_id)
            .fetch_all(&agent.db.pool)
            .await
            .unwrap();

        assert_eq!(agents.len(), 7);

        let expected_roles = vec!["The Manager", "The Promoter", "The Salesperson", "The Ambassador", "The Accountant", "The Protector", "The Advisor"];
        for role in expected_roles {
            assert!(agents.iter().any(|a| a.get::<String, _>("role") == role));
        }

        let users = sqlx::query("SELECT username, email, roles FROM users WHERE organization_id = $1")
            .bind(&org_id)
            .fetch_all(&agent.db.pool)
            .await
            .unwrap();

        assert_eq!(users.len(), 1);
        assert_eq!(users[0].get::<String, _>("email"), "admin@test.com");
        assert_eq!(users[0].get::<String, _>("username"), "Admin User");
        assert!(users[0].get::<String, _>("roles").contains("admin"));
    }

    #[tokio::test]
    async fn test_start_onboarding_service_and_food_cart() {
        let db = match setup_test_db().await {
            Some(db) => db,
            None => return,
        };
        let (tx, _) = tokio::sync::mpsc::channel(10);
        let hub = std::sync::Arc::new(crate::hub::Hub::new(tx, db.pool.clone()));
        let agent = OnboardingAgent::new(db.clone(), hub);

        // Test Service Business
        let req_service = StartOnboardingRequest {
            business_type: "Service Business".to_string(),
            company_name: "Test Service".to_string(),
            company_description: "A test service".to_string(),
            selling_categories: vec![],
            payment_pref: "online".to_string(),
            admin_email: "service@test.com".to_string(),
            admin_name: "Service Admin".to_string(),
            admin_password: "password123".to_string(),
            website_template: "Modern".to_string(),
            first_product_name: "Consultation".to_string(),
            first_product_price: "100.00".to_string(),
            domain_choice: "subdomain".to_string(),
            price_type: "fixed".to_string(),
        };

        let res_service = agent.start_onboarding(req_service).await.unwrap();
        let org_id_service = res_service.organization_id;

        use sqlx::Row;
        let row_service = sqlx::query("SELECT state_json FROM onboarding_state WHERE organization_id = $1")
            .bind(&org_id_service)
            .fetch_one(&db.pool)
            .await
            .unwrap();

        let state_json_service: serde_json::Value = row_service.try_get("state_json").unwrap_or_else(|_| serde_json::json!({}));
        assert_eq!(state_json_service.get("enable_booking").and_then(|v| v.as_bool()), Some(true));

        let agents_service = sqlx::query("SELECT role FROM agents WHERE organization_id = $1 AND role = 'The Salesperson'")
            .bind(&org_id_service)
            .fetch_all(&agent.db.pool)
            .await
            .unwrap();
        assert_eq!(agents_service.len(), 1);

        // Test Food Cart
        let req_food = StartOnboardingRequest {
            business_type: "Food Cart".to_string(),
            company_name: "Test Food".to_string(),
            company_description: "A test food cart".to_string(),
            selling_categories: vec![],
            payment_pref: "online".to_string(),
            admin_email: "food@test.com".to_string(),
            admin_name: "Food Admin".to_string(),
            admin_password: "password123".to_string(),
            website_template: "Modern".to_string(),
            first_product_name: "Taco".to_string(),
            first_product_price: "5.00".to_string(),
            domain_choice: "subdomain".to_string(),
            price_type: "fixed".to_string(),
        };

        let res_food = agent.start_onboarding(req_food).await.unwrap();
        let org_id_food = res_food.organization_id;

        let row_food = sqlx::query("SELECT state_json FROM onboarding_state WHERE organization_id = $1")
            .bind(&org_id_food)
            .fetch_one(&db.pool)
            .await
            .unwrap();

        let state_json_food: serde_json::Value = row_food.try_get("state_json").unwrap_or_else(|_| serde_json::json!({}));
        assert_eq!(state_json_food.get("enable_menu").and_then(|v| v.as_bool()), Some(true));
    }
}
