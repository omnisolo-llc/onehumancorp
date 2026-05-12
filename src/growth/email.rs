// Email Marketing simple
pub struct EmailCampaign {
    pub template: String,
    pub recipients: Vec<String>,
}
impl EmailCampaign {
    pub fn send(&self) {
        println!("Sending {} to {} recipients", self.template, self.recipients.len());
    }
}
