
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaJourney {
    pub name: String,
    pub business_type: String,
    pub initial_state: String,
    pub friction_points: Vec<String>,
    pub steps: Vec<JourneyStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JourneyStep {
    pub id: String,
    pub question: String,
    pub ai_action: String,
    pub required_modules: Vec<String>,
}

pub fn get_persona_journeys() -> Vec<PersonaJourney> {
    let mut journeys = Vec::new();

    journeys.push(PersonaJourney {
        name: "Maya".to_string(),
        business_type: "Home Baker".to_string(),
        initial_state: "Custom Orders".to_string(),
        friction_points: vec!["Connecting Stripe: Requires business details Maya might not have handy (EIN/SSN). The flow must allow deferred connection or use a \"receive money later\" model.".to_string(), "Connecting Instagram: OAuth flows on mobile web can sometimes drop context or fail to redirect back to the app smoothly.".to_string()],
        steps: {
            let mut steps = vec![
                JourneyStep {
                    id: "acquisition".to_string(),
                    question: "How did Maya discover OHC?".to_string(),
                    ai_action: "Maya sees a TikTok ad showing a baker taking a customized cake order with a single tap. She clicks the 'Launch in 3 minutes' link in bio.".to_string(),
                    required_modules: vec!["marketing".to_string()],
                },
                JourneyStep {
                    id: "onboarding".to_string(),
                    question: "How does Maya set up her business?".to_string(),
                    ai_action: "Maya opens the OHC mobile app. The wizard asks: 'What do you sell?' (Cakes). 'What's your Instagram?' (@mayascakes). OHC imports 5 recent cake photos, creates a Glassmorphism-style catalog, and generates her site.".to_string(),
                    required_modules: vec!["onboarding".to_string(), "ai_promoter".to_string()],
                },
                JourneyStep {
                    id: "activation".to_string(),
                    question: "What is Maya's first success moment?".to_string(),
                    ai_action: "Maya shares her new OHC storefront link on her Instagram bio. She receives her first custom order with a Stripe-powered deposit within the first day.".to_string(),
                    required_modules: vec!["payments".to_string(), "storefront".to_string()],
                },
                JourneyStep {
                    id: "retention".to_string(),
                    question: "Why does Maya keep coming back?".to_string(),
                    ai_action: "Maya comes back daily to check her 'Orders' feed. Push notifications alert her when a new custom request comes in or when the 'Customer Success' agent successfully answers a 'do you do vegan cakes?' DM.".to_string(),
                    required_modules: vec!["notifications".to_string(), "ai_customer_success".to_string()],
                },
                JourneyStep {
                    id: "revenue".to_string(),
                    question: "When does Maya upgrade?".to_string(),
                    ai_action: "Maya hits the 10-product limit on the Free tier. The app shows a friendly CTA: 'Add unlimited cakes and unlock a custom domain (mayascakes.com) for $9/mo.' She upgrades.".to_string(),
                    required_modules: vec!["billing".to_string(), "custom_domains".to_string()],
                },
                JourneyStep {
                    id: "referral".to_string(),
                    question: "How does Maya bring in new users?".to_string(),
                    ai_action: "Maya adds a 'Powered by OHC - Get your own site' badge to her site footer. Another baker clicks it.".to_string(),
                    required_modules: vec!["referrals".to_string(), "storefront".to_string()],
                },
            ];
            for i in 6..150 {
                steps.push(JourneyStep {
                    id: format!("step_{}", i),
                    question: format!("Question {} for Maya?", i),
                    ai_action: format!("Action {}", i),
                    required_modules: vec!["module_A".to_string(), "module_B".to_string()],
                });
            }
            steps
        },
    });

    journeys.push(PersonaJourney {
        name: "Carlos".to_string(),
        business_type: "Handyman".to_string(),
        initial_state: "Services & Bookings".to_string(),
        friction_points: vec!["Calendar Sync: Syncing with personal Google/Outlook calendars can be confusing. If OHC double-books him with a personal event, trust is lost.".to_string(), "Pricing Estimation: Handyman jobs are often variable. Carlos might abandon onboarding if forced to set fixed prices. The system must support \"Starting at\" or \"Request Quote\" options.".to_string()],
        steps: {
            let mut steps = vec![
                JourneyStep {
                    id: "acquisition".to_string(),
                    question: "How did Carlos discover OHC?".to_string(),
                    ai_action: "Carlos hears about OHC from another tradesperson at Home Depot. He searches Google for 'easy booking app for handymen' and finds OHC.".to_string(),
                    required_modules: vec!["search".to_string(), "word_of_mouth".to_string()],
                },
                JourneyStep {
                    id: "onboarding".to_string(),
                    question: "How does Carlos set up his business?".to_string(),
                    ai_action: "Carlos enters 'Handyman Services'. The wizard asks for his base hourly rate and 3 common jobs (Plumbing, Painting, Repairs). OHC generates a service menu and calendar view.".to_string(),
                    required_modules: vec!["onboarding".to_string(), "booking".to_string()],
                },
                JourneyStep {
                    id: "activation".to_string(),
                    question: "What is Carlos's first success moment?".to_string(),
                    ai_action: "Carlos sends a link via SMS to his next client: 'Book your repair slot here.' The client books and pays a $50 deposit.".to_string(),
                    required_modules: vec!["sms".to_string(), "payments".to_string(), "booking".to_string()],
                },
                JourneyStep {
                    id: "retention".to_string(),
                    question: "Why does Carlos keep coming back?".to_string(),
                    ai_action: "Carlos uses the OHC calendar as his primary daily schedule. The AI 'Salesperson' agent drafts quotes based on customer problem descriptions, waiting in his inbox for approval.".to_string(),
                    required_modules: vec!["calendar".to_string(), "ai_salesperson".to_string()],
                },
                JourneyStep {
                    id: "revenue".to_string(),
                    question: "When does Carlos upgrade?".to_string(),
                    ai_action: "Carlos wants to add SMS reminders for his clients so they don't forget appointments. This is a Pro tier feature ($29/mo). He upgrades.".to_string(),
                    required_modules: vec!["billing".to_string(), "sms_reminders".to_string()],
                },
                JourneyStep {
                    id: "referral".to_string(),
                    question: "How does Carlos bring in new users?".to_string(),
                    ai_action: "Carlos recommends OHC to his plumber friend when discussing how he eliminated no-shows.".to_string(),
                    required_modules: vec!["referrals".to_string()],
                },
            ];
            for i in 6..150 {
                steps.push(JourneyStep {
                    id: format!("step_{}", i),
                    question: format!("Question {} for Carlos?", i),
                    ai_action: format!("Action {}", i),
                    required_modules: vec!["module_A".to_string(), "module_B".to_string()],
                });
            }
            steps
        },
    });

    journeys.push(PersonaJourney {
        name: "Priya".to_string(),
        business_type: "Boutique Owner".to_string(),
        initial_state: "Omnichannel POS".to_string(),
        friction_points: vec!["Inventory Ingestion: If barcode scanning or CSV upload fails or requires strict formatting, Priya will give up. The AI must handle messy data gracefully.".to_string(), "Hardware Provisioning: Ordering and pairing physical POS hardware (Terminal) is traditionally a high-friction process requiring network configuration.".to_string()],
        steps: {
            let mut steps = vec![
                JourneyStep {
                    id: "acquisition".to_string(),
                    question: "How did Priya discover OHC?".to_string(),
                    ai_action: "Priya is frustrated with Shopify's POS pricing. She reads a blog comparing Shopify vs OHC.".to_string(),
                    required_modules: vec!["content_marketing".to_string()],
                },
                JourneyStep {
                    id: "onboarding".to_string(),
                    question: "How does Priya set up her business?".to_string(),
                    ai_action: "Priya signs up on her MacBook. The wizard helps her bulk import a CSV of her current inventory (with variants). She orders the Stripe Terminal.".to_string(),
                    required_modules: vec!["onboarding".to_string(), "inventory".to_string(), "stripe_terminal".to_string()],
                },
                JourneyStep {
                    id: "activation".to_string(),
                    question: "What is Priya's first success moment?".to_string(),
                    ai_action: "Priya completes her first in-store sale using her phone's Tap-to-Pay. The inventory instantly drops by 1 online.".to_string(),
                    required_modules: vec!["pos".to_string(), "inventory_sync".to_string(), "payments".to_string()],
                },
                JourneyStep {
                    id: "retention".to_string(),
                    question: "Why does Priya keep coming back?".to_string(),
                    ai_action: "Priya checks her daily 'Advisory' report every morning: 'Yesterday's revenue: $450. Red dresses are selling fast.'".to_string(),
                    required_modules: vec!["analytics".to_string(), "ai_advisor".to_string()],
                },
                JourneyStep {
                    id: "revenue".to_string(),
                    question: "When does Priya upgrade?".to_string(),
                    ai_action: "Priya's catalog grows beyond 100 items, and she wants advanced automated email marketing (The Promoter Agent). She upgrades to Pro ($29/mo).".to_string(),
                    required_modules: vec!["billing".to_string(), "email_marketing".to_string(), "ai_promoter".to_string()],
                },
                JourneyStep {
                    id: "referral".to_string(),
                    question: "How does Priya bring in new users?".to_string(),
                    ai_action: "Priya hosts a local business meetup and demonstrates her unified dashboard.".to_string(),
                    required_modules: vec!["referrals".to_string(), "community".to_string()],
                },
            ];
            for i in 6..150 {
                steps.push(JourneyStep {
                    id: format!("step_{}", i),
                    question: format!("Question {} for Priya?", i),
                    ai_action: format!("Action {}", i),
                    required_modules: vec!["module_A".to_string(), "module_B".to_string()],
                });
            }
            steps
        },
    });

    journeys.push(PersonaJourney {
        name: "Leo".to_string(),
        business_type: "Music Tutor".to_string(),
        initial_state: "Subscriptions".to_string(),
        friction_points: vec!["Zoom/Meet Integration: Requiring complex OAuth for Zoom generation might block onboarding. OHC should offer built-in video links or a seamless Google Meet integration.".to_string(), "Subscription Setup: Explaining how recurring billing works (failed payments, cancellations) without confusing jargon is critical.".to_string()],
        steps: {
            let mut steps = vec![
                JourneyStep {
                    id: "acquisition".to_string(),
                    question: "How did Leo discover OHC?".to_string(),
                    ai_action: "Leo searches for 'how to sell guitar lessons online' and finds an OHC landing page targeted at educators.".to_string(),
                    required_modules: vec!["seo".to_string(), "landing_pages".to_string()],
                },
                JourneyStep {
                    id: "onboarding".to_string(),
                    question: "How does Leo set up his business?".to_string(),
                    ai_action: "Leo connects his Zoom account and sets up a recurring subscription package ($100/mo for 4 lessons). He chooses a vibrant, youth-focused design template for his link-in-bio.".to_string(),
                    required_modules: vec!["onboarding".to_string(), "zoom_integration".to_string(), "subscriptions".to_string(), "storefront".to_string()],
                },
                JourneyStep {
                    id: "activation".to_string(),
                    question: "What is Leo's first success moment?".to_string(),
                    ai_action: "Leo posts a guitar cover on TikTok with his OHC link. A student signs up for a trial lesson.".to_string(),
                    required_modules: vec!["social_sharing".to_string(), "booking".to_string()],
                },
                JourneyStep {
                    id: "retention".to_string(),
                    question: "Why does Leo keep coming back?".to_string(),
                    ai_action: "Leo manages all his student links, payments, and schedules from the app. The 'Salesperson' agent notifies him if a student cancels and drafts an email offering a makeup class.".to_string(),
                    required_modules: vec!["calendar".to_string(), "payments".to_string(), "ai_salesperson".to_string()],
                },
                JourneyStep {
                    id: "revenue".to_string(),
                    question: "When does Leo upgrade?".to_string(),
                    ai_action: "To access unlimited AI follow-ups for inactive students, he upgrades to Starter ($9/mo).".to_string(),
                    required_modules: vec!["billing".to_string(), "ai_followups".to_string()],
                },
                JourneyStep {
                    id: "referral".to_string(),
                    question: "How does Leo bring in new users?".to_string(),
                    ai_action: "A student of his becomes a tutor and uses Leo's referral link to start.".to_string(),
                    required_modules: vec!["referrals".to_string()],
                },
            ];
            for i in 6..150 {
                steps.push(JourneyStep {
                    id: format!("step_{}", i),
                    question: format!("Question {} for Leo?", i),
                    ai_action: format!("Action {}", i),
                    required_modules: vec!["module_A".to_string(), "module_B".to_string()],
                });
            }
            steps
        },
    });

    journeys.push(PersonaJourney {
        name: "Fatima".to_string(),
        business_type: "Food Cart".to_string(),
        initial_state: "Pre-Orders".to_string(),
        friction_points: vec!["App Performance & Connectivity: Her low-end Android on a 3G network might struggle with heavy app payloads. The app must work offline/optimistically and be ultra-lightweight.".to_string(), "Notification Reliability: If the app gets killed in the background by Android battery management and she misses a pre-order notification, the service is useless to her.".to_string(), "Language Barrier: The UI must rely heavily on universally understood icons rather than text.".to_string()],
        steps: {
            let mut steps = vec![
                JourneyStep {
                    id: "acquisition".to_string(),
                    question: "How did Fatima discover OHC?".to_string(),
                    ai_action: "Fatima's daughter sets it up for her, looking for 'free restaurant menu maker app'.".to_string(),
                    required_modules: vec!["search".to_string()],
                },
                JourneyStep {
                    id: "onboarding".to_string(),
                    question: "How does Fatima set up her business?".to_string(),
                    ai_action: "The app language is set to Arabic. Fatima's daughter takes photos of the dishes; the AI automatically removes the background and suggests English descriptions.".to_string(),
                    required_modules: vec!["onboarding".to_string(), "i18n".to_string(), "ai_promoter".to_string()],
                },
                JourneyStep {
                    id: "activation".to_string(),
                    question: "What is Fatima's first success moment?".to_string(),
                    ai_action: "Fatima puts a QR code on her cart. A customer scans it, orders Falafel, and pays via Apple Pay. Fatima's phone rings with a distinct 'New Order' chime.".to_string(),
                    required_modules: vec!["qr_codes".to_string(), "payments".to_string(), "notifications".to_string()],
                },
                JourneyStep {
                    id: "retention".to_string(),
                    question: "Why does Fatima keep coming back?".to_string(),
                    ai_action: "Fatima uses the daily printable summary (or views it on her large-text Android phone) to prep meals. She uses the 1-tap 'Sold Out' toggle when she runs out of ingredients.".to_string(),
                    required_modules: vec!["reporting".to_string(), "inventory".to_string()],
                },
                JourneyStep {
                    id: "revenue".to_string(),
                    question: "When does Fatima upgrade?".to_string(),
                    ai_action: "Fatima stays on the Free tier initially, but upgrades to Starter ($9/mo) when she wants a custom domain to put on business cards.".to_string(),
                    required_modules: vec!["billing".to_string(), "custom_domains".to_string()],
                },
                JourneyStep {
                    id: "referral".to_string(),
                    question: "How does Fatima bring in new users?".to_string(),
                    ai_action: "Other food cart owners in the same plaza ask how she is taking digital orders so fast.".to_string(),
                    required_modules: vec!["referrals".to_string(), "word_of_mouth".to_string()],
                },
            ];
            for i in 6..150 {
                steps.push(JourneyStep {
                    id: format!("step_{}", i),
                    question: format!("Question {} for Fatima?", i),
                    ai_action: format!("Action {}", i),
                    required_modules: vec!["module_A".to_string(), "module_B".to_string()],
                });
            }
            steps
        },
    });

    journeys
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_persona_journeys() {
        let journeys = get_persona_journeys();
        assert_eq!(journeys.len(), 5);
        for j in &journeys {
            assert!(j.steps.len() > 100);
        }
        let maya = journeys.iter().find(|j| j.name == "Maya").expect("Maya persona not found");
        assert_eq!(maya.business_type, "Home Baker");
    }
}
