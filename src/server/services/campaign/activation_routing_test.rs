mod activation_routing;

use activation_routing::CampaignChannel;

#[test]
fn routes_campaign_asset_types_to_third_party_channels() {
    assert_eq!(CampaignChannel::from_asset_type("Email"), Some(CampaignChannel::SendGrid));
    assert_eq!(CampaignChannel::from_asset_type("sendgrid"), Some(CampaignChannel::SendGrid));
    assert_eq!(CampaignChannel::from_asset_type("SMS"), Some(CampaignChannel::Twilio));
    assert_eq!(CampaignChannel::from_asset_type("twilio"), Some(CampaignChannel::Twilio));
    assert_eq!(CampaignChannel::from_asset_type("Social"), Some(CampaignChannel::Meta));
    assert_eq!(CampaignChannel::from_asset_type("instagram"), Some(CampaignChannel::Meta));
    assert_eq!(CampaignChannel::from_asset_type("Image"), None);
}
