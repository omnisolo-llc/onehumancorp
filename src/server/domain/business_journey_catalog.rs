pub struct BusinessJourney {
    pub id: String,
    pub persona: String,
    pub agents: Vec<String>,
    pub features: Vec<String>,
}

pub fn get_business_journeys() -> Vec<BusinessJourney> {
    vec![
        BusinessJourney {
            id: "baker_0".to_string(),
            persona: "Maya 0".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 0 A".to_string(),
                "Feature 0 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_0".to_string(),
            persona: "Carlos 0".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 0 A".to_string(),
                "Feature 0 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_0".to_string(),
            persona: "Priya 0".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 0 A".to_string(),
                "Feature 0 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_0".to_string(),
            persona: "Leo 0".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 0 A".to_string(),
                "Feature 0 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_0".to_string(),
            persona: "Fatima 0".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 0 A".to_string(),
                "Feature 0 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_1".to_string(),
            persona: "Maya 1".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 1 A".to_string(),
                "Feature 1 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_1".to_string(),
            persona: "Carlos 1".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 1 A".to_string(),
                "Feature 1 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_1".to_string(),
            persona: "Priya 1".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 1 A".to_string(),
                "Feature 1 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_1".to_string(),
            persona: "Leo 1".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 1 A".to_string(),
                "Feature 1 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_1".to_string(),
            persona: "Fatima 1".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 1 A".to_string(),
                "Feature 1 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_2".to_string(),
            persona: "Maya 2".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 2 A".to_string(),
                "Feature 2 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_2".to_string(),
            persona: "Carlos 2".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 2 A".to_string(),
                "Feature 2 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_2".to_string(),
            persona: "Priya 2".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 2 A".to_string(),
                "Feature 2 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_2".to_string(),
            persona: "Leo 2".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 2 A".to_string(),
                "Feature 2 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_2".to_string(),
            persona: "Fatima 2".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 2 A".to_string(),
                "Feature 2 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_3".to_string(),
            persona: "Maya 3".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 3 A".to_string(),
                "Feature 3 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_3".to_string(),
            persona: "Carlos 3".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 3 A".to_string(),
                "Feature 3 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_3".to_string(),
            persona: "Priya 3".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 3 A".to_string(),
                "Feature 3 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_3".to_string(),
            persona: "Leo 3".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 3 A".to_string(),
                "Feature 3 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_3".to_string(),
            persona: "Fatima 3".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 3 A".to_string(),
                "Feature 3 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_4".to_string(),
            persona: "Maya 4".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 4 A".to_string(),
                "Feature 4 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_4".to_string(),
            persona: "Carlos 4".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 4 A".to_string(),
                "Feature 4 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_4".to_string(),
            persona: "Priya 4".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 4 A".to_string(),
                "Feature 4 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_4".to_string(),
            persona: "Leo 4".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 4 A".to_string(),
                "Feature 4 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_4".to_string(),
            persona: "Fatima 4".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 4 A".to_string(),
                "Feature 4 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_5".to_string(),
            persona: "Maya 5".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 5 A".to_string(),
                "Feature 5 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_5".to_string(),
            persona: "Carlos 5".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 5 A".to_string(),
                "Feature 5 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_5".to_string(),
            persona: "Priya 5".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 5 A".to_string(),
                "Feature 5 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_5".to_string(),
            persona: "Leo 5".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 5 A".to_string(),
                "Feature 5 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_5".to_string(),
            persona: "Fatima 5".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 5 A".to_string(),
                "Feature 5 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_6".to_string(),
            persona: "Maya 6".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 6 A".to_string(),
                "Feature 6 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_6".to_string(),
            persona: "Carlos 6".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 6 A".to_string(),
                "Feature 6 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_6".to_string(),
            persona: "Priya 6".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 6 A".to_string(),
                "Feature 6 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_6".to_string(),
            persona: "Leo 6".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 6 A".to_string(),
                "Feature 6 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_6".to_string(),
            persona: "Fatima 6".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 6 A".to_string(),
                "Feature 6 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_7".to_string(),
            persona: "Maya 7".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 7 A".to_string(),
                "Feature 7 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_7".to_string(),
            persona: "Carlos 7".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 7 A".to_string(),
                "Feature 7 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_7".to_string(),
            persona: "Priya 7".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 7 A".to_string(),
                "Feature 7 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_7".to_string(),
            persona: "Leo 7".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 7 A".to_string(),
                "Feature 7 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_7".to_string(),
            persona: "Fatima 7".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 7 A".to_string(),
                "Feature 7 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_8".to_string(),
            persona: "Maya 8".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 8 A".to_string(),
                "Feature 8 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_8".to_string(),
            persona: "Carlos 8".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 8 A".to_string(),
                "Feature 8 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_8".to_string(),
            persona: "Priya 8".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 8 A".to_string(),
                "Feature 8 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_8".to_string(),
            persona: "Leo 8".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 8 A".to_string(),
                "Feature 8 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_8".to_string(),
            persona: "Fatima 8".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 8 A".to_string(),
                "Feature 8 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_9".to_string(),
            persona: "Maya 9".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 9 A".to_string(),
                "Feature 9 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_9".to_string(),
            persona: "Carlos 9".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 9 A".to_string(),
                "Feature 9 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_9".to_string(),
            persona: "Priya 9".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 9 A".to_string(),
                "Feature 9 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_9".to_string(),
            persona: "Leo 9".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 9 A".to_string(),
                "Feature 9 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_9".to_string(),
            persona: "Fatima 9".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 9 A".to_string(),
                "Feature 9 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_10".to_string(),
            persona: "Maya 10".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 10 A".to_string(),
                "Feature 10 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_10".to_string(),
            persona: "Carlos 10".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 10 A".to_string(),
                "Feature 10 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_10".to_string(),
            persona: "Priya 10".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 10 A".to_string(),
                "Feature 10 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_10".to_string(),
            persona: "Leo 10".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 10 A".to_string(),
                "Feature 10 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_10".to_string(),
            persona: "Fatima 10".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 10 A".to_string(),
                "Feature 10 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_11".to_string(),
            persona: "Maya 11".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 11 A".to_string(),
                "Feature 11 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_11".to_string(),
            persona: "Carlos 11".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 11 A".to_string(),
                "Feature 11 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_11".to_string(),
            persona: "Priya 11".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 11 A".to_string(),
                "Feature 11 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_11".to_string(),
            persona: "Leo 11".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 11 A".to_string(),
                "Feature 11 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_11".to_string(),
            persona: "Fatima 11".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 11 A".to_string(),
                "Feature 11 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_12".to_string(),
            persona: "Maya 12".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 12 A".to_string(),
                "Feature 12 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_12".to_string(),
            persona: "Carlos 12".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 12 A".to_string(),
                "Feature 12 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_12".to_string(),
            persona: "Priya 12".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 12 A".to_string(),
                "Feature 12 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_12".to_string(),
            persona: "Leo 12".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 12 A".to_string(),
                "Feature 12 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_12".to_string(),
            persona: "Fatima 12".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 12 A".to_string(),
                "Feature 12 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_13".to_string(),
            persona: "Maya 13".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 13 A".to_string(),
                "Feature 13 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_13".to_string(),
            persona: "Carlos 13".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 13 A".to_string(),
                "Feature 13 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_13".to_string(),
            persona: "Priya 13".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 13 A".to_string(),
                "Feature 13 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_13".to_string(),
            persona: "Leo 13".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 13 A".to_string(),
                "Feature 13 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_13".to_string(),
            persona: "Fatima 13".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 13 A".to_string(),
                "Feature 13 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_14".to_string(),
            persona: "Maya 14".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 14 A".to_string(),
                "Feature 14 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_14".to_string(),
            persona: "Carlos 14".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 14 A".to_string(),
                "Feature 14 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_14".to_string(),
            persona: "Priya 14".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 14 A".to_string(),
                "Feature 14 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_14".to_string(),
            persona: "Leo 14".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 14 A".to_string(),
                "Feature 14 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_14".to_string(),
            persona: "Fatima 14".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 14 A".to_string(),
                "Feature 14 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_15".to_string(),
            persona: "Maya 15".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 15 A".to_string(),
                "Feature 15 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_15".to_string(),
            persona: "Carlos 15".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 15 A".to_string(),
                "Feature 15 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_15".to_string(),
            persona: "Priya 15".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 15 A".to_string(),
                "Feature 15 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_15".to_string(),
            persona: "Leo 15".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 15 A".to_string(),
                "Feature 15 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_15".to_string(),
            persona: "Fatima 15".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 15 A".to_string(),
                "Feature 15 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_16".to_string(),
            persona: "Maya 16".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 16 A".to_string(),
                "Feature 16 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_16".to_string(),
            persona: "Carlos 16".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 16 A".to_string(),
                "Feature 16 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_16".to_string(),
            persona: "Priya 16".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 16 A".to_string(),
                "Feature 16 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_16".to_string(),
            persona: "Leo 16".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 16 A".to_string(),
                "Feature 16 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_16".to_string(),
            persona: "Fatima 16".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 16 A".to_string(),
                "Feature 16 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_17".to_string(),
            persona: "Maya 17".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 17 A".to_string(),
                "Feature 17 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_17".to_string(),
            persona: "Carlos 17".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 17 A".to_string(),
                "Feature 17 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_17".to_string(),
            persona: "Priya 17".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 17 A".to_string(),
                "Feature 17 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_17".to_string(),
            persona: "Leo 17".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 17 A".to_string(),
                "Feature 17 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_17".to_string(),
            persona: "Fatima 17".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 17 A".to_string(),
                "Feature 17 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_18".to_string(),
            persona: "Maya 18".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 18 A".to_string(),
                "Feature 18 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_18".to_string(),
            persona: "Carlos 18".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 18 A".to_string(),
                "Feature 18 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_18".to_string(),
            persona: "Priya 18".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 18 A".to_string(),
                "Feature 18 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_18".to_string(),
            persona: "Leo 18".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 18 A".to_string(),
                "Feature 18 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_18".to_string(),
            persona: "Fatima 18".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 18 A".to_string(),
                "Feature 18 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_19".to_string(),
            persona: "Maya 19".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 19 A".to_string(),
                "Feature 19 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_19".to_string(),
            persona: "Carlos 19".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 19 A".to_string(),
                "Feature 19 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_19".to_string(),
            persona: "Priya 19".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 19 A".to_string(),
                "Feature 19 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_19".to_string(),
            persona: "Leo 19".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 19 A".to_string(),
                "Feature 19 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_19".to_string(),
            persona: "Fatima 19".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 19 A".to_string(),
                "Feature 19 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_20".to_string(),
            persona: "Maya 20".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 20 A".to_string(),
                "Feature 20 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_20".to_string(),
            persona: "Carlos 20".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 20 A".to_string(),
                "Feature 20 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_20".to_string(),
            persona: "Priya 20".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 20 A".to_string(),
                "Feature 20 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_20".to_string(),
            persona: "Leo 20".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 20 A".to_string(),
                "Feature 20 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_20".to_string(),
            persona: "Fatima 20".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 20 A".to_string(),
                "Feature 20 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_21".to_string(),
            persona: "Maya 21".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 21 A".to_string(),
                "Feature 21 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_21".to_string(),
            persona: "Carlos 21".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 21 A".to_string(),
                "Feature 21 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_21".to_string(),
            persona: "Priya 21".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 21 A".to_string(),
                "Feature 21 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_21".to_string(),
            persona: "Leo 21".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 21 A".to_string(),
                "Feature 21 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_21".to_string(),
            persona: "Fatima 21".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 21 A".to_string(),
                "Feature 21 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_22".to_string(),
            persona: "Maya 22".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 22 A".to_string(),
                "Feature 22 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_22".to_string(),
            persona: "Carlos 22".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 22 A".to_string(),
                "Feature 22 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_22".to_string(),
            persona: "Priya 22".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 22 A".to_string(),
                "Feature 22 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_22".to_string(),
            persona: "Leo 22".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 22 A".to_string(),
                "Feature 22 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_22".to_string(),
            persona: "Fatima 22".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 22 A".to_string(),
                "Feature 22 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_23".to_string(),
            persona: "Maya 23".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 23 A".to_string(),
                "Feature 23 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_23".to_string(),
            persona: "Carlos 23".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 23 A".to_string(),
                "Feature 23 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_23".to_string(),
            persona: "Priya 23".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 23 A".to_string(),
                "Feature 23 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_23".to_string(),
            persona: "Leo 23".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 23 A".to_string(),
                "Feature 23 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_23".to_string(),
            persona: "Fatima 23".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 23 A".to_string(),
                "Feature 23 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_24".to_string(),
            persona: "Maya 24".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 24 A".to_string(),
                "Feature 24 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_24".to_string(),
            persona: "Carlos 24".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 24 A".to_string(),
                "Feature 24 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_24".to_string(),
            persona: "Priya 24".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 24 A".to_string(),
                "Feature 24 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_24".to_string(),
            persona: "Leo 24".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 24 A".to_string(),
                "Feature 24 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_24".to_string(),
            persona: "Fatima 24".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 24 A".to_string(),
                "Feature 24 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_25".to_string(),
            persona: "Maya 25".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 25 A".to_string(),
                "Feature 25 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_25".to_string(),
            persona: "Carlos 25".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 25 A".to_string(),
                "Feature 25 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_25".to_string(),
            persona: "Priya 25".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 25 A".to_string(),
                "Feature 25 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_25".to_string(),
            persona: "Leo 25".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 25 A".to_string(),
                "Feature 25 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_25".to_string(),
            persona: "Fatima 25".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 25 A".to_string(),
                "Feature 25 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_26".to_string(),
            persona: "Maya 26".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 26 A".to_string(),
                "Feature 26 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_26".to_string(),
            persona: "Carlos 26".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 26 A".to_string(),
                "Feature 26 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_26".to_string(),
            persona: "Priya 26".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 26 A".to_string(),
                "Feature 26 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_26".to_string(),
            persona: "Leo 26".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 26 A".to_string(),
                "Feature 26 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_26".to_string(),
            persona: "Fatima 26".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 26 A".to_string(),
                "Feature 26 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_27".to_string(),
            persona: "Maya 27".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 27 A".to_string(),
                "Feature 27 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_27".to_string(),
            persona: "Carlos 27".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 27 A".to_string(),
                "Feature 27 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_27".to_string(),
            persona: "Priya 27".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 27 A".to_string(),
                "Feature 27 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_27".to_string(),
            persona: "Leo 27".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 27 A".to_string(),
                "Feature 27 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_27".to_string(),
            persona: "Fatima 27".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 27 A".to_string(),
                "Feature 27 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_28".to_string(),
            persona: "Maya 28".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 28 A".to_string(),
                "Feature 28 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_28".to_string(),
            persona: "Carlos 28".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 28 A".to_string(),
                "Feature 28 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_28".to_string(),
            persona: "Priya 28".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 28 A".to_string(),
                "Feature 28 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_28".to_string(),
            persona: "Leo 28".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 28 A".to_string(),
                "Feature 28 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_28".to_string(),
            persona: "Fatima 28".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 28 A".to_string(),
                "Feature 28 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_29".to_string(),
            persona: "Maya 29".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 29 A".to_string(),
                "Feature 29 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_29".to_string(),
            persona: "Carlos 29".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 29 A".to_string(),
                "Feature 29 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_29".to_string(),
            persona: "Priya 29".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 29 A".to_string(),
                "Feature 29 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_29".to_string(),
            persona: "Leo 29".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 29 A".to_string(),
                "Feature 29 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_29".to_string(),
            persona: "Fatima 29".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 29 A".to_string(),
                "Feature 29 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_30".to_string(),
            persona: "Maya 30".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 30 A".to_string(),
                "Feature 30 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_30".to_string(),
            persona: "Carlos 30".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 30 A".to_string(),
                "Feature 30 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_30".to_string(),
            persona: "Priya 30".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 30 A".to_string(),
                "Feature 30 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_30".to_string(),
            persona: "Leo 30".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 30 A".to_string(),
                "Feature 30 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_30".to_string(),
            persona: "Fatima 30".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 30 A".to_string(),
                "Feature 30 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_31".to_string(),
            persona: "Maya 31".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 31 A".to_string(),
                "Feature 31 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_31".to_string(),
            persona: "Carlos 31".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 31 A".to_string(),
                "Feature 31 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_31".to_string(),
            persona: "Priya 31".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 31 A".to_string(),
                "Feature 31 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_31".to_string(),
            persona: "Leo 31".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 31 A".to_string(),
                "Feature 31 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_31".to_string(),
            persona: "Fatima 31".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 31 A".to_string(),
                "Feature 31 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_32".to_string(),
            persona: "Maya 32".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 32 A".to_string(),
                "Feature 32 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_32".to_string(),
            persona: "Carlos 32".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 32 A".to_string(),
                "Feature 32 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_32".to_string(),
            persona: "Priya 32".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 32 A".to_string(),
                "Feature 32 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_32".to_string(),
            persona: "Leo 32".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 32 A".to_string(),
                "Feature 32 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_32".to_string(),
            persona: "Fatima 32".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 32 A".to_string(),
                "Feature 32 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_33".to_string(),
            persona: "Maya 33".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 33 A".to_string(),
                "Feature 33 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_33".to_string(),
            persona: "Carlos 33".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 33 A".to_string(),
                "Feature 33 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_33".to_string(),
            persona: "Priya 33".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 33 A".to_string(),
                "Feature 33 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_33".to_string(),
            persona: "Leo 33".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 33 A".to_string(),
                "Feature 33 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_33".to_string(),
            persona: "Fatima 33".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 33 A".to_string(),
                "Feature 33 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_34".to_string(),
            persona: "Maya 34".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 34 A".to_string(),
                "Feature 34 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_34".to_string(),
            persona: "Carlos 34".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 34 A".to_string(),
                "Feature 34 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_34".to_string(),
            persona: "Priya 34".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 34 A".to_string(),
                "Feature 34 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_34".to_string(),
            persona: "Leo 34".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 34 A".to_string(),
                "Feature 34 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_34".to_string(),
            persona: "Fatima 34".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 34 A".to_string(),
                "Feature 34 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_35".to_string(),
            persona: "Maya 35".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 35 A".to_string(),
                "Feature 35 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_35".to_string(),
            persona: "Carlos 35".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 35 A".to_string(),
                "Feature 35 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_35".to_string(),
            persona: "Priya 35".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 35 A".to_string(),
                "Feature 35 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_35".to_string(),
            persona: "Leo 35".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 35 A".to_string(),
                "Feature 35 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_35".to_string(),
            persona: "Fatima 35".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 35 A".to_string(),
                "Feature 35 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_36".to_string(),
            persona: "Maya 36".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 36 A".to_string(),
                "Feature 36 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_36".to_string(),
            persona: "Carlos 36".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 36 A".to_string(),
                "Feature 36 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_36".to_string(),
            persona: "Priya 36".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 36 A".to_string(),
                "Feature 36 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_36".to_string(),
            persona: "Leo 36".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 36 A".to_string(),
                "Feature 36 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_36".to_string(),
            persona: "Fatima 36".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 36 A".to_string(),
                "Feature 36 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_37".to_string(),
            persona: "Maya 37".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 37 A".to_string(),
                "Feature 37 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_37".to_string(),
            persona: "Carlos 37".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 37 A".to_string(),
                "Feature 37 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_37".to_string(),
            persona: "Priya 37".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 37 A".to_string(),
                "Feature 37 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_37".to_string(),
            persona: "Leo 37".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 37 A".to_string(),
                "Feature 37 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_37".to_string(),
            persona: "Fatima 37".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 37 A".to_string(),
                "Feature 37 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_38".to_string(),
            persona: "Maya 38".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 38 A".to_string(),
                "Feature 38 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_38".to_string(),
            persona: "Carlos 38".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 38 A".to_string(),
                "Feature 38 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_38".to_string(),
            persona: "Priya 38".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 38 A".to_string(),
                "Feature 38 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_38".to_string(),
            persona: "Leo 38".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 38 A".to_string(),
                "Feature 38 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_38".to_string(),
            persona: "Fatima 38".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 38 A".to_string(),
                "Feature 38 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_39".to_string(),
            persona: "Maya 39".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 39 A".to_string(),
                "Feature 39 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_39".to_string(),
            persona: "Carlos 39".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 39 A".to_string(),
                "Feature 39 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_39".to_string(),
            persona: "Priya 39".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 39 A".to_string(),
                "Feature 39 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_39".to_string(),
            persona: "Leo 39".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 39 A".to_string(),
                "Feature 39 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_39".to_string(),
            persona: "Fatima 39".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 39 A".to_string(),
                "Feature 39 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_40".to_string(),
            persona: "Maya 40".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 40 A".to_string(),
                "Feature 40 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_40".to_string(),
            persona: "Carlos 40".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 40 A".to_string(),
                "Feature 40 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_40".to_string(),
            persona: "Priya 40".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 40 A".to_string(),
                "Feature 40 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_40".to_string(),
            persona: "Leo 40".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 40 A".to_string(),
                "Feature 40 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_40".to_string(),
            persona: "Fatima 40".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 40 A".to_string(),
                "Feature 40 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_41".to_string(),
            persona: "Maya 41".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 41 A".to_string(),
                "Feature 41 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_41".to_string(),
            persona: "Carlos 41".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 41 A".to_string(),
                "Feature 41 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_41".to_string(),
            persona: "Priya 41".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 41 A".to_string(),
                "Feature 41 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_41".to_string(),
            persona: "Leo 41".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 41 A".to_string(),
                "Feature 41 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_41".to_string(),
            persona: "Fatima 41".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 41 A".to_string(),
                "Feature 41 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_42".to_string(),
            persona: "Maya 42".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 42 A".to_string(),
                "Feature 42 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_42".to_string(),
            persona: "Carlos 42".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 42 A".to_string(),
                "Feature 42 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_42".to_string(),
            persona: "Priya 42".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 42 A".to_string(),
                "Feature 42 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_42".to_string(),
            persona: "Leo 42".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 42 A".to_string(),
                "Feature 42 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_42".to_string(),
            persona: "Fatima 42".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 42 A".to_string(),
                "Feature 42 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_43".to_string(),
            persona: "Maya 43".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 43 A".to_string(),
                "Feature 43 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_43".to_string(),
            persona: "Carlos 43".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 43 A".to_string(),
                "Feature 43 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_43".to_string(),
            persona: "Priya 43".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 43 A".to_string(),
                "Feature 43 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_43".to_string(),
            persona: "Leo 43".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 43 A".to_string(),
                "Feature 43 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_43".to_string(),
            persona: "Fatima 43".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 43 A".to_string(),
                "Feature 43 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_44".to_string(),
            persona: "Maya 44".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 44 A".to_string(),
                "Feature 44 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_44".to_string(),
            persona: "Carlos 44".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 44 A".to_string(),
                "Feature 44 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_44".to_string(),
            persona: "Priya 44".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 44 A".to_string(),
                "Feature 44 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_44".to_string(),
            persona: "Leo 44".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 44 A".to_string(),
                "Feature 44 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_44".to_string(),
            persona: "Fatima 44".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 44 A".to_string(),
                "Feature 44 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_45".to_string(),
            persona: "Maya 45".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 45 A".to_string(),
                "Feature 45 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_45".to_string(),
            persona: "Carlos 45".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 45 A".to_string(),
                "Feature 45 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_45".to_string(),
            persona: "Priya 45".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 45 A".to_string(),
                "Feature 45 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_45".to_string(),
            persona: "Leo 45".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 45 A".to_string(),
                "Feature 45 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_45".to_string(),
            persona: "Fatima 45".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 45 A".to_string(),
                "Feature 45 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_46".to_string(),
            persona: "Maya 46".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 46 A".to_string(),
                "Feature 46 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_46".to_string(),
            persona: "Carlos 46".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 46 A".to_string(),
                "Feature 46 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_46".to_string(),
            persona: "Priya 46".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 46 A".to_string(),
                "Feature 46 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_46".to_string(),
            persona: "Leo 46".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 46 A".to_string(),
                "Feature 46 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_46".to_string(),
            persona: "Fatima 46".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 46 A".to_string(),
                "Feature 46 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_47".to_string(),
            persona: "Maya 47".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 47 A".to_string(),
                "Feature 47 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_47".to_string(),
            persona: "Carlos 47".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 47 A".to_string(),
                "Feature 47 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_47".to_string(),
            persona: "Priya 47".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 47 A".to_string(),
                "Feature 47 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_47".to_string(),
            persona: "Leo 47".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 47 A".to_string(),
                "Feature 47 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_47".to_string(),
            persona: "Fatima 47".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 47 A".to_string(),
                "Feature 47 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_48".to_string(),
            persona: "Maya 48".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 48 A".to_string(),
                "Feature 48 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_48".to_string(),
            persona: "Carlos 48".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 48 A".to_string(),
                "Feature 48 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_48".to_string(),
            persona: "Priya 48".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 48 A".to_string(),
                "Feature 48 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_48".to_string(),
            persona: "Leo 48".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 48 A".to_string(),
                "Feature 48 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_48".to_string(),
            persona: "Fatima 48".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 48 A".to_string(),
                "Feature 48 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_49".to_string(),
            persona: "Maya 49".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 49 A".to_string(),
                "Feature 49 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_49".to_string(),
            persona: "Carlos 49".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 49 A".to_string(),
                "Feature 49 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_49".to_string(),
            persona: "Priya 49".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 49 A".to_string(),
                "Feature 49 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_49".to_string(),
            persona: "Leo 49".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 49 A".to_string(),
                "Feature 49 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_49".to_string(),
            persona: "Fatima 49".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 49 A".to_string(),
                "Feature 49 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_50".to_string(),
            persona: "Maya 50".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 50 A".to_string(),
                "Feature 50 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_50".to_string(),
            persona: "Carlos 50".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 50 A".to_string(),
                "Feature 50 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_50".to_string(),
            persona: "Priya 50".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 50 A".to_string(),
                "Feature 50 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_50".to_string(),
            persona: "Leo 50".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 50 A".to_string(),
                "Feature 50 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_50".to_string(),
            persona: "Fatima 50".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 50 A".to_string(),
                "Feature 50 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_51".to_string(),
            persona: "Maya 51".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 51 A".to_string(),
                "Feature 51 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_51".to_string(),
            persona: "Carlos 51".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 51 A".to_string(),
                "Feature 51 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_51".to_string(),
            persona: "Priya 51".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 51 A".to_string(),
                "Feature 51 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_51".to_string(),
            persona: "Leo 51".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 51 A".to_string(),
                "Feature 51 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_51".to_string(),
            persona: "Fatima 51".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 51 A".to_string(),
                "Feature 51 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_52".to_string(),
            persona: "Maya 52".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 52 A".to_string(),
                "Feature 52 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_52".to_string(),
            persona: "Carlos 52".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 52 A".to_string(),
                "Feature 52 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_52".to_string(),
            persona: "Priya 52".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 52 A".to_string(),
                "Feature 52 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_52".to_string(),
            persona: "Leo 52".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 52 A".to_string(),
                "Feature 52 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_52".to_string(),
            persona: "Fatima 52".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 52 A".to_string(),
                "Feature 52 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_53".to_string(),
            persona: "Maya 53".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 53 A".to_string(),
                "Feature 53 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_53".to_string(),
            persona: "Carlos 53".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 53 A".to_string(),
                "Feature 53 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_53".to_string(),
            persona: "Priya 53".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 53 A".to_string(),
                "Feature 53 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_53".to_string(),
            persona: "Leo 53".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 53 A".to_string(),
                "Feature 53 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_53".to_string(),
            persona: "Fatima 53".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 53 A".to_string(),
                "Feature 53 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_54".to_string(),
            persona: "Maya 54".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 54 A".to_string(),
                "Feature 54 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_54".to_string(),
            persona: "Carlos 54".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 54 A".to_string(),
                "Feature 54 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_54".to_string(),
            persona: "Priya 54".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 54 A".to_string(),
                "Feature 54 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_54".to_string(),
            persona: "Leo 54".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 54 A".to_string(),
                "Feature 54 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_54".to_string(),
            persona: "Fatima 54".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 54 A".to_string(),
                "Feature 54 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_55".to_string(),
            persona: "Maya 55".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 55 A".to_string(),
                "Feature 55 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_55".to_string(),
            persona: "Carlos 55".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 55 A".to_string(),
                "Feature 55 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_55".to_string(),
            persona: "Priya 55".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 55 A".to_string(),
                "Feature 55 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_55".to_string(),
            persona: "Leo 55".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 55 A".to_string(),
                "Feature 55 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_55".to_string(),
            persona: "Fatima 55".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 55 A".to_string(),
                "Feature 55 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_56".to_string(),
            persona: "Maya 56".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 56 A".to_string(),
                "Feature 56 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_56".to_string(),
            persona: "Carlos 56".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 56 A".to_string(),
                "Feature 56 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_56".to_string(),
            persona: "Priya 56".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 56 A".to_string(),
                "Feature 56 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_56".to_string(),
            persona: "Leo 56".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 56 A".to_string(),
                "Feature 56 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_56".to_string(),
            persona: "Fatima 56".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 56 A".to_string(),
                "Feature 56 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_57".to_string(),
            persona: "Maya 57".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 57 A".to_string(),
                "Feature 57 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_57".to_string(),
            persona: "Carlos 57".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 57 A".to_string(),
                "Feature 57 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_57".to_string(),
            persona: "Priya 57".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 57 A".to_string(),
                "Feature 57 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_57".to_string(),
            persona: "Leo 57".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 57 A".to_string(),
                "Feature 57 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_57".to_string(),
            persona: "Fatima 57".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 57 A".to_string(),
                "Feature 57 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_58".to_string(),
            persona: "Maya 58".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 58 A".to_string(),
                "Feature 58 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_58".to_string(),
            persona: "Carlos 58".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 58 A".to_string(),
                "Feature 58 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_58".to_string(),
            persona: "Priya 58".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 58 A".to_string(),
                "Feature 58 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_58".to_string(),
            persona: "Leo 58".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 58 A".to_string(),
                "Feature 58 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_58".to_string(),
            persona: "Fatima 58".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 58 A".to_string(),
                "Feature 58 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_59".to_string(),
            persona: "Maya 59".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 59 A".to_string(),
                "Feature 59 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_59".to_string(),
            persona: "Carlos 59".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 59 A".to_string(),
                "Feature 59 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_59".to_string(),
            persona: "Priya 59".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 59 A".to_string(),
                "Feature 59 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_59".to_string(),
            persona: "Leo 59".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 59 A".to_string(),
                "Feature 59 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_59".to_string(),
            persona: "Fatima 59".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 59 A".to_string(),
                "Feature 59 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_60".to_string(),
            persona: "Maya 60".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 60 A".to_string(),
                "Feature 60 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_60".to_string(),
            persona: "Carlos 60".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 60 A".to_string(),
                "Feature 60 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_60".to_string(),
            persona: "Priya 60".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 60 A".to_string(),
                "Feature 60 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_60".to_string(),
            persona: "Leo 60".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 60 A".to_string(),
                "Feature 60 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_60".to_string(),
            persona: "Fatima 60".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 60 A".to_string(),
                "Feature 60 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_61".to_string(),
            persona: "Maya 61".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 61 A".to_string(),
                "Feature 61 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_61".to_string(),
            persona: "Carlos 61".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 61 A".to_string(),
                "Feature 61 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_61".to_string(),
            persona: "Priya 61".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 61 A".to_string(),
                "Feature 61 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_61".to_string(),
            persona: "Leo 61".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 61 A".to_string(),
                "Feature 61 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_61".to_string(),
            persona: "Fatima 61".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 61 A".to_string(),
                "Feature 61 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_62".to_string(),
            persona: "Maya 62".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 62 A".to_string(),
                "Feature 62 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_62".to_string(),
            persona: "Carlos 62".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 62 A".to_string(),
                "Feature 62 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_62".to_string(),
            persona: "Priya 62".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 62 A".to_string(),
                "Feature 62 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_62".to_string(),
            persona: "Leo 62".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 62 A".to_string(),
                "Feature 62 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_62".to_string(),
            persona: "Fatima 62".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 62 A".to_string(),
                "Feature 62 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_63".to_string(),
            persona: "Maya 63".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 63 A".to_string(),
                "Feature 63 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_63".to_string(),
            persona: "Carlos 63".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 63 A".to_string(),
                "Feature 63 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_63".to_string(),
            persona: "Priya 63".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 63 A".to_string(),
                "Feature 63 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_63".to_string(),
            persona: "Leo 63".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 63 A".to_string(),
                "Feature 63 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_63".to_string(),
            persona: "Fatima 63".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 63 A".to_string(),
                "Feature 63 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_64".to_string(),
            persona: "Maya 64".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 64 A".to_string(),
                "Feature 64 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_64".to_string(),
            persona: "Carlos 64".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 64 A".to_string(),
                "Feature 64 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_64".to_string(),
            persona: "Priya 64".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 64 A".to_string(),
                "Feature 64 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_64".to_string(),
            persona: "Leo 64".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 64 A".to_string(),
                "Feature 64 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_64".to_string(),
            persona: "Fatima 64".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 64 A".to_string(),
                "Feature 64 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_65".to_string(),
            persona: "Maya 65".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 65 A".to_string(),
                "Feature 65 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_65".to_string(),
            persona: "Carlos 65".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 65 A".to_string(),
                "Feature 65 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_65".to_string(),
            persona: "Priya 65".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 65 A".to_string(),
                "Feature 65 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_65".to_string(),
            persona: "Leo 65".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 65 A".to_string(),
                "Feature 65 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_65".to_string(),
            persona: "Fatima 65".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 65 A".to_string(),
                "Feature 65 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_66".to_string(),
            persona: "Maya 66".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 66 A".to_string(),
                "Feature 66 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_66".to_string(),
            persona: "Carlos 66".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 66 A".to_string(),
                "Feature 66 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_66".to_string(),
            persona: "Priya 66".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 66 A".to_string(),
                "Feature 66 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_66".to_string(),
            persona: "Leo 66".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 66 A".to_string(),
                "Feature 66 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_66".to_string(),
            persona: "Fatima 66".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 66 A".to_string(),
                "Feature 66 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_67".to_string(),
            persona: "Maya 67".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 67 A".to_string(),
                "Feature 67 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_67".to_string(),
            persona: "Carlos 67".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 67 A".to_string(),
                "Feature 67 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_67".to_string(),
            persona: "Priya 67".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 67 A".to_string(),
                "Feature 67 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_67".to_string(),
            persona: "Leo 67".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 67 A".to_string(),
                "Feature 67 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_67".to_string(),
            persona: "Fatima 67".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 67 A".to_string(),
                "Feature 67 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_68".to_string(),
            persona: "Maya 68".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 68 A".to_string(),
                "Feature 68 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_68".to_string(),
            persona: "Carlos 68".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 68 A".to_string(),
                "Feature 68 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_68".to_string(),
            persona: "Priya 68".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 68 A".to_string(),
                "Feature 68 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_68".to_string(),
            persona: "Leo 68".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 68 A".to_string(),
                "Feature 68 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_68".to_string(),
            persona: "Fatima 68".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 68 A".to_string(),
                "Feature 68 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_69".to_string(),
            persona: "Maya 69".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 69 A".to_string(),
                "Feature 69 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_69".to_string(),
            persona: "Carlos 69".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 69 A".to_string(),
                "Feature 69 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_69".to_string(),
            persona: "Priya 69".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 69 A".to_string(),
                "Feature 69 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_69".to_string(),
            persona: "Leo 69".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 69 A".to_string(),
                "Feature 69 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_69".to_string(),
            persona: "Fatima 69".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 69 A".to_string(),
                "Feature 69 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_70".to_string(),
            persona: "Maya 70".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 70 A".to_string(),
                "Feature 70 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_70".to_string(),
            persona: "Carlos 70".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 70 A".to_string(),
                "Feature 70 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_70".to_string(),
            persona: "Priya 70".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 70 A".to_string(),
                "Feature 70 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_70".to_string(),
            persona: "Leo 70".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 70 A".to_string(),
                "Feature 70 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_70".to_string(),
            persona: "Fatima 70".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 70 A".to_string(),
                "Feature 70 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_71".to_string(),
            persona: "Maya 71".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 71 A".to_string(),
                "Feature 71 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_71".to_string(),
            persona: "Carlos 71".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 71 A".to_string(),
                "Feature 71 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_71".to_string(),
            persona: "Priya 71".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 71 A".to_string(),
                "Feature 71 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_71".to_string(),
            persona: "Leo 71".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 71 A".to_string(),
                "Feature 71 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_71".to_string(),
            persona: "Fatima 71".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 71 A".to_string(),
                "Feature 71 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_72".to_string(),
            persona: "Maya 72".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 72 A".to_string(),
                "Feature 72 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_72".to_string(),
            persona: "Carlos 72".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 72 A".to_string(),
                "Feature 72 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_72".to_string(),
            persona: "Priya 72".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 72 A".to_string(),
                "Feature 72 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_72".to_string(),
            persona: "Leo 72".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 72 A".to_string(),
                "Feature 72 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_72".to_string(),
            persona: "Fatima 72".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 72 A".to_string(),
                "Feature 72 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_73".to_string(),
            persona: "Maya 73".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 73 A".to_string(),
                "Feature 73 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_73".to_string(),
            persona: "Carlos 73".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 73 A".to_string(),
                "Feature 73 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_73".to_string(),
            persona: "Priya 73".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 73 A".to_string(),
                "Feature 73 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_73".to_string(),
            persona: "Leo 73".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 73 A".to_string(),
                "Feature 73 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_73".to_string(),
            persona: "Fatima 73".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 73 A".to_string(),
                "Feature 73 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_74".to_string(),
            persona: "Maya 74".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 74 A".to_string(),
                "Feature 74 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_74".to_string(),
            persona: "Carlos 74".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 74 A".to_string(),
                "Feature 74 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_74".to_string(),
            persona: "Priya 74".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 74 A".to_string(),
                "Feature 74 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_74".to_string(),
            persona: "Leo 74".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 74 A".to_string(),
                "Feature 74 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_74".to_string(),
            persona: "Fatima 74".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 74 A".to_string(),
                "Feature 74 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_75".to_string(),
            persona: "Maya 75".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 75 A".to_string(),
                "Feature 75 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_75".to_string(),
            persona: "Carlos 75".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 75 A".to_string(),
                "Feature 75 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_75".to_string(),
            persona: "Priya 75".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 75 A".to_string(),
                "Feature 75 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_75".to_string(),
            persona: "Leo 75".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 75 A".to_string(),
                "Feature 75 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_75".to_string(),
            persona: "Fatima 75".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 75 A".to_string(),
                "Feature 75 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_76".to_string(),
            persona: "Maya 76".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 76 A".to_string(),
                "Feature 76 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_76".to_string(),
            persona: "Carlos 76".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 76 A".to_string(),
                "Feature 76 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_76".to_string(),
            persona: "Priya 76".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 76 A".to_string(),
                "Feature 76 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_76".to_string(),
            persona: "Leo 76".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 76 A".to_string(),
                "Feature 76 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_76".to_string(),
            persona: "Fatima 76".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 76 A".to_string(),
                "Feature 76 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_77".to_string(),
            persona: "Maya 77".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 77 A".to_string(),
                "Feature 77 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_77".to_string(),
            persona: "Carlos 77".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 77 A".to_string(),
                "Feature 77 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_77".to_string(),
            persona: "Priya 77".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 77 A".to_string(),
                "Feature 77 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_77".to_string(),
            persona: "Leo 77".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 77 A".to_string(),
                "Feature 77 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_77".to_string(),
            persona: "Fatima 77".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 77 A".to_string(),
                "Feature 77 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_78".to_string(),
            persona: "Maya 78".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 78 A".to_string(),
                "Feature 78 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_78".to_string(),
            persona: "Carlos 78".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 78 A".to_string(),
                "Feature 78 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_78".to_string(),
            persona: "Priya 78".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 78 A".to_string(),
                "Feature 78 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_78".to_string(),
            persona: "Leo 78".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 78 A".to_string(),
                "Feature 78 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_78".to_string(),
            persona: "Fatima 78".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 78 A".to_string(),
                "Feature 78 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_79".to_string(),
            persona: "Maya 79".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 79 A".to_string(),
                "Feature 79 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_79".to_string(),
            persona: "Carlos 79".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 79 A".to_string(),
                "Feature 79 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_79".to_string(),
            persona: "Priya 79".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 79 A".to_string(),
                "Feature 79 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_79".to_string(),
            persona: "Leo 79".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 79 A".to_string(),
                "Feature 79 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_79".to_string(),
            persona: "Fatima 79".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 79 A".to_string(),
                "Feature 79 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_80".to_string(),
            persona: "Maya 80".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 80 A".to_string(),
                "Feature 80 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_80".to_string(),
            persona: "Carlos 80".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 80 A".to_string(),
                "Feature 80 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_80".to_string(),
            persona: "Priya 80".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 80 A".to_string(),
                "Feature 80 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_80".to_string(),
            persona: "Leo 80".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 80 A".to_string(),
                "Feature 80 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_80".to_string(),
            persona: "Fatima 80".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 80 A".to_string(),
                "Feature 80 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_81".to_string(),
            persona: "Maya 81".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 81 A".to_string(),
                "Feature 81 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_81".to_string(),
            persona: "Carlos 81".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 81 A".to_string(),
                "Feature 81 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_81".to_string(),
            persona: "Priya 81".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 81 A".to_string(),
                "Feature 81 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_81".to_string(),
            persona: "Leo 81".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 81 A".to_string(),
                "Feature 81 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_81".to_string(),
            persona: "Fatima 81".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 81 A".to_string(),
                "Feature 81 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_82".to_string(),
            persona: "Maya 82".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 82 A".to_string(),
                "Feature 82 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_82".to_string(),
            persona: "Carlos 82".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 82 A".to_string(),
                "Feature 82 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_82".to_string(),
            persona: "Priya 82".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 82 A".to_string(),
                "Feature 82 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_82".to_string(),
            persona: "Leo 82".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 82 A".to_string(),
                "Feature 82 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_82".to_string(),
            persona: "Fatima 82".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 82 A".to_string(),
                "Feature 82 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_83".to_string(),
            persona: "Maya 83".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 83 A".to_string(),
                "Feature 83 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_83".to_string(),
            persona: "Carlos 83".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 83 A".to_string(),
                "Feature 83 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_83".to_string(),
            persona: "Priya 83".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 83 A".to_string(),
                "Feature 83 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_83".to_string(),
            persona: "Leo 83".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 83 A".to_string(),
                "Feature 83 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_83".to_string(),
            persona: "Fatima 83".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 83 A".to_string(),
                "Feature 83 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_84".to_string(),
            persona: "Maya 84".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 84 A".to_string(),
                "Feature 84 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_84".to_string(),
            persona: "Carlos 84".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 84 A".to_string(),
                "Feature 84 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_84".to_string(),
            persona: "Priya 84".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 84 A".to_string(),
                "Feature 84 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_84".to_string(),
            persona: "Leo 84".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 84 A".to_string(),
                "Feature 84 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_84".to_string(),
            persona: "Fatima 84".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 84 A".to_string(),
                "Feature 84 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_85".to_string(),
            persona: "Maya 85".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 85 A".to_string(),
                "Feature 85 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_85".to_string(),
            persona: "Carlos 85".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 85 A".to_string(),
                "Feature 85 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_85".to_string(),
            persona: "Priya 85".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 85 A".to_string(),
                "Feature 85 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_85".to_string(),
            persona: "Leo 85".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 85 A".to_string(),
                "Feature 85 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_85".to_string(),
            persona: "Fatima 85".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 85 A".to_string(),
                "Feature 85 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_86".to_string(),
            persona: "Maya 86".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 86 A".to_string(),
                "Feature 86 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_86".to_string(),
            persona: "Carlos 86".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 86 A".to_string(),
                "Feature 86 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_86".to_string(),
            persona: "Priya 86".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 86 A".to_string(),
                "Feature 86 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_86".to_string(),
            persona: "Leo 86".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 86 A".to_string(),
                "Feature 86 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_86".to_string(),
            persona: "Fatima 86".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 86 A".to_string(),
                "Feature 86 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_87".to_string(),
            persona: "Maya 87".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 87 A".to_string(),
                "Feature 87 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_87".to_string(),
            persona: "Carlos 87".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 87 A".to_string(),
                "Feature 87 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_87".to_string(),
            persona: "Priya 87".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 87 A".to_string(),
                "Feature 87 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_87".to_string(),
            persona: "Leo 87".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 87 A".to_string(),
                "Feature 87 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_87".to_string(),
            persona: "Fatima 87".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 87 A".to_string(),
                "Feature 87 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_88".to_string(),
            persona: "Maya 88".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 88 A".to_string(),
                "Feature 88 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_88".to_string(),
            persona: "Carlos 88".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 88 A".to_string(),
                "Feature 88 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_88".to_string(),
            persona: "Priya 88".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 88 A".to_string(),
                "Feature 88 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_88".to_string(),
            persona: "Leo 88".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 88 A".to_string(),
                "Feature 88 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_88".to_string(),
            persona: "Fatima 88".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 88 A".to_string(),
                "Feature 88 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_89".to_string(),
            persona: "Maya 89".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 89 A".to_string(),
                "Feature 89 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_89".to_string(),
            persona: "Carlos 89".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 89 A".to_string(),
                "Feature 89 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_89".to_string(),
            persona: "Priya 89".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 89 A".to_string(),
                "Feature 89 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_89".to_string(),
            persona: "Leo 89".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 89 A".to_string(),
                "Feature 89 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_89".to_string(),
            persona: "Fatima 89".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 89 A".to_string(),
                "Feature 89 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_90".to_string(),
            persona: "Maya 90".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 90 A".to_string(),
                "Feature 90 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_90".to_string(),
            persona: "Carlos 90".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 90 A".to_string(),
                "Feature 90 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_90".to_string(),
            persona: "Priya 90".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 90 A".to_string(),
                "Feature 90 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_90".to_string(),
            persona: "Leo 90".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 90 A".to_string(),
                "Feature 90 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_90".to_string(),
            persona: "Fatima 90".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 90 A".to_string(),
                "Feature 90 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_91".to_string(),
            persona: "Maya 91".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 91 A".to_string(),
                "Feature 91 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_91".to_string(),
            persona: "Carlos 91".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 91 A".to_string(),
                "Feature 91 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_91".to_string(),
            persona: "Priya 91".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 91 A".to_string(),
                "Feature 91 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_91".to_string(),
            persona: "Leo 91".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 91 A".to_string(),
                "Feature 91 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_91".to_string(),
            persona: "Fatima 91".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 91 A".to_string(),
                "Feature 91 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_92".to_string(),
            persona: "Maya 92".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 92 A".to_string(),
                "Feature 92 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_92".to_string(),
            persona: "Carlos 92".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 92 A".to_string(),
                "Feature 92 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_92".to_string(),
            persona: "Priya 92".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 92 A".to_string(),
                "Feature 92 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_92".to_string(),
            persona: "Leo 92".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 92 A".to_string(),
                "Feature 92 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_92".to_string(),
            persona: "Fatima 92".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 92 A".to_string(),
                "Feature 92 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_93".to_string(),
            persona: "Maya 93".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 93 A".to_string(),
                "Feature 93 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_93".to_string(),
            persona: "Carlos 93".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 93 A".to_string(),
                "Feature 93 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_93".to_string(),
            persona: "Priya 93".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 93 A".to_string(),
                "Feature 93 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_93".to_string(),
            persona: "Leo 93".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 93 A".to_string(),
                "Feature 93 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_93".to_string(),
            persona: "Fatima 93".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 93 A".to_string(),
                "Feature 93 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_94".to_string(),
            persona: "Maya 94".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 94 A".to_string(),
                "Feature 94 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_94".to_string(),
            persona: "Carlos 94".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 94 A".to_string(),
                "Feature 94 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_94".to_string(),
            persona: "Priya 94".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 94 A".to_string(),
                "Feature 94 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_94".to_string(),
            persona: "Leo 94".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 94 A".to_string(),
                "Feature 94 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_94".to_string(),
            persona: "Fatima 94".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 94 A".to_string(),
                "Feature 94 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_95".to_string(),
            persona: "Maya 95".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 95 A".to_string(),
                "Feature 95 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_95".to_string(),
            persona: "Carlos 95".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 95 A".to_string(),
                "Feature 95 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_95".to_string(),
            persona: "Priya 95".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 95 A".to_string(),
                "Feature 95 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_95".to_string(),
            persona: "Leo 95".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 95 A".to_string(),
                "Feature 95 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_95".to_string(),
            persona: "Fatima 95".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 95 A".to_string(),
                "Feature 95 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_96".to_string(),
            persona: "Maya 96".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 96 A".to_string(),
                "Feature 96 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_96".to_string(),
            persona: "Carlos 96".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 96 A".to_string(),
                "Feature 96 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_96".to_string(),
            persona: "Priya 96".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 96 A".to_string(),
                "Feature 96 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_96".to_string(),
            persona: "Leo 96".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 96 A".to_string(),
                "Feature 96 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_96".to_string(),
            persona: "Fatima 96".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 96 A".to_string(),
                "Feature 96 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_97".to_string(),
            persona: "Maya 97".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 97 A".to_string(),
                "Feature 97 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_97".to_string(),
            persona: "Carlos 97".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 97 A".to_string(),
                "Feature 97 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_97".to_string(),
            persona: "Priya 97".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 97 A".to_string(),
                "Feature 97 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_97".to_string(),
            persona: "Leo 97".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 97 A".to_string(),
                "Feature 97 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_97".to_string(),
            persona: "Fatima 97".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 97 A".to_string(),
                "Feature 97 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_98".to_string(),
            persona: "Maya 98".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 98 A".to_string(),
                "Feature 98 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_98".to_string(),
            persona: "Carlos 98".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 98 A".to_string(),
                "Feature 98 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_98".to_string(),
            persona: "Priya 98".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 98 A".to_string(),
                "Feature 98 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_98".to_string(),
            persona: "Leo 98".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 98 A".to_string(),
                "Feature 98 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_98".to_string(),
            persona: "Fatima 98".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 98 A".to_string(),
                "Feature 98 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_99".to_string(),
            persona: "Maya 99".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 99 A".to_string(),
                "Feature 99 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_99".to_string(),
            persona: "Carlos 99".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 99 A".to_string(),
                "Feature 99 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_99".to_string(),
            persona: "Priya 99".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 99 A".to_string(),
                "Feature 99 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_99".to_string(),
            persona: "Leo 99".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 99 A".to_string(),
                "Feature 99 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_99".to_string(),
            persona: "Fatima 99".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 99 A".to_string(),
                "Feature 99 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_100".to_string(),
            persona: "Maya 100".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 100 A".to_string(),
                "Feature 100 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_100".to_string(),
            persona: "Carlos 100".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 100 A".to_string(),
                "Feature 100 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_100".to_string(),
            persona: "Priya 100".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 100 A".to_string(),
                "Feature 100 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_100".to_string(),
            persona: "Leo 100".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 100 A".to_string(),
                "Feature 100 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_100".to_string(),
            persona: "Fatima 100".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 100 A".to_string(),
                "Feature 100 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_101".to_string(),
            persona: "Maya 101".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 101 A".to_string(),
                "Feature 101 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_101".to_string(),
            persona: "Carlos 101".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 101 A".to_string(),
                "Feature 101 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_101".to_string(),
            persona: "Priya 101".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 101 A".to_string(),
                "Feature 101 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_101".to_string(),
            persona: "Leo 101".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 101 A".to_string(),
                "Feature 101 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_101".to_string(),
            persona: "Fatima 101".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 101 A".to_string(),
                "Feature 101 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_102".to_string(),
            persona: "Maya 102".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 102 A".to_string(),
                "Feature 102 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_102".to_string(),
            persona: "Carlos 102".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 102 A".to_string(),
                "Feature 102 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_102".to_string(),
            persona: "Priya 102".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 102 A".to_string(),
                "Feature 102 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_102".to_string(),
            persona: "Leo 102".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 102 A".to_string(),
                "Feature 102 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_102".to_string(),
            persona: "Fatima 102".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 102 A".to_string(),
                "Feature 102 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_103".to_string(),
            persona: "Maya 103".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 103 A".to_string(),
                "Feature 103 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_103".to_string(),
            persona: "Carlos 103".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 103 A".to_string(),
                "Feature 103 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_103".to_string(),
            persona: "Priya 103".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 103 A".to_string(),
                "Feature 103 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_103".to_string(),
            persona: "Leo 103".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 103 A".to_string(),
                "Feature 103 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_103".to_string(),
            persona: "Fatima 103".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 103 A".to_string(),
                "Feature 103 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_104".to_string(),
            persona: "Maya 104".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 104 A".to_string(),
                "Feature 104 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_104".to_string(),
            persona: "Carlos 104".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 104 A".to_string(),
                "Feature 104 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_104".to_string(),
            persona: "Priya 104".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 104 A".to_string(),
                "Feature 104 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_104".to_string(),
            persona: "Leo 104".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 104 A".to_string(),
                "Feature 104 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_104".to_string(),
            persona: "Fatima 104".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 104 A".to_string(),
                "Feature 104 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_105".to_string(),
            persona: "Maya 105".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 105 A".to_string(),
                "Feature 105 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_105".to_string(),
            persona: "Carlos 105".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 105 A".to_string(),
                "Feature 105 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_105".to_string(),
            persona: "Priya 105".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 105 A".to_string(),
                "Feature 105 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_105".to_string(),
            persona: "Leo 105".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 105 A".to_string(),
                "Feature 105 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_105".to_string(),
            persona: "Fatima 105".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 105 A".to_string(),
                "Feature 105 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_106".to_string(),
            persona: "Maya 106".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 106 A".to_string(),
                "Feature 106 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_106".to_string(),
            persona: "Carlos 106".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 106 A".to_string(),
                "Feature 106 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_106".to_string(),
            persona: "Priya 106".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 106 A".to_string(),
                "Feature 106 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_106".to_string(),
            persona: "Leo 106".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 106 A".to_string(),
                "Feature 106 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_106".to_string(),
            persona: "Fatima 106".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 106 A".to_string(),
                "Feature 106 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_107".to_string(),
            persona: "Maya 107".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 107 A".to_string(),
                "Feature 107 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_107".to_string(),
            persona: "Carlos 107".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 107 A".to_string(),
                "Feature 107 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_107".to_string(),
            persona: "Priya 107".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 107 A".to_string(),
                "Feature 107 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_107".to_string(),
            persona: "Leo 107".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 107 A".to_string(),
                "Feature 107 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_107".to_string(),
            persona: "Fatima 107".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 107 A".to_string(),
                "Feature 107 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_108".to_string(),
            persona: "Maya 108".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 108 A".to_string(),
                "Feature 108 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_108".to_string(),
            persona: "Carlos 108".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 108 A".to_string(),
                "Feature 108 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_108".to_string(),
            persona: "Priya 108".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 108 A".to_string(),
                "Feature 108 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_108".to_string(),
            persona: "Leo 108".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 108 A".to_string(),
                "Feature 108 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_108".to_string(),
            persona: "Fatima 108".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 108 A".to_string(),
                "Feature 108 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_109".to_string(),
            persona: "Maya 109".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 109 A".to_string(),
                "Feature 109 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_109".to_string(),
            persona: "Carlos 109".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 109 A".to_string(),
                "Feature 109 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_109".to_string(),
            persona: "Priya 109".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 109 A".to_string(),
                "Feature 109 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_109".to_string(),
            persona: "Leo 109".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 109 A".to_string(),
                "Feature 109 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_109".to_string(),
            persona: "Fatima 109".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 109 A".to_string(),
                "Feature 109 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_110".to_string(),
            persona: "Maya 110".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 110 A".to_string(),
                "Feature 110 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_110".to_string(),
            persona: "Carlos 110".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 110 A".to_string(),
                "Feature 110 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_110".to_string(),
            persona: "Priya 110".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 110 A".to_string(),
                "Feature 110 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_110".to_string(),
            persona: "Leo 110".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 110 A".to_string(),
                "Feature 110 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_110".to_string(),
            persona: "Fatima 110".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 110 A".to_string(),
                "Feature 110 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_111".to_string(),
            persona: "Maya 111".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 111 A".to_string(),
                "Feature 111 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_111".to_string(),
            persona: "Carlos 111".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 111 A".to_string(),
                "Feature 111 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_111".to_string(),
            persona: "Priya 111".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 111 A".to_string(),
                "Feature 111 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_111".to_string(),
            persona: "Leo 111".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 111 A".to_string(),
                "Feature 111 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_111".to_string(),
            persona: "Fatima 111".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 111 A".to_string(),
                "Feature 111 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_112".to_string(),
            persona: "Maya 112".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 112 A".to_string(),
                "Feature 112 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_112".to_string(),
            persona: "Carlos 112".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 112 A".to_string(),
                "Feature 112 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_112".to_string(),
            persona: "Priya 112".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 112 A".to_string(),
                "Feature 112 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_112".to_string(),
            persona: "Leo 112".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 112 A".to_string(),
                "Feature 112 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_112".to_string(),
            persona: "Fatima 112".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 112 A".to_string(),
                "Feature 112 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_113".to_string(),
            persona: "Maya 113".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 113 A".to_string(),
                "Feature 113 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_113".to_string(),
            persona: "Carlos 113".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 113 A".to_string(),
                "Feature 113 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_113".to_string(),
            persona: "Priya 113".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 113 A".to_string(),
                "Feature 113 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_113".to_string(),
            persona: "Leo 113".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 113 A".to_string(),
                "Feature 113 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_113".to_string(),
            persona: "Fatima 113".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 113 A".to_string(),
                "Feature 113 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_114".to_string(),
            persona: "Maya 114".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 114 A".to_string(),
                "Feature 114 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_114".to_string(),
            persona: "Carlos 114".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 114 A".to_string(),
                "Feature 114 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_114".to_string(),
            persona: "Priya 114".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 114 A".to_string(),
                "Feature 114 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_114".to_string(),
            persona: "Leo 114".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 114 A".to_string(),
                "Feature 114 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_114".to_string(),
            persona: "Fatima 114".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 114 A".to_string(),
                "Feature 114 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_115".to_string(),
            persona: "Maya 115".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 115 A".to_string(),
                "Feature 115 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_115".to_string(),
            persona: "Carlos 115".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 115 A".to_string(),
                "Feature 115 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_115".to_string(),
            persona: "Priya 115".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 115 A".to_string(),
                "Feature 115 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_115".to_string(),
            persona: "Leo 115".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 115 A".to_string(),
                "Feature 115 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_115".to_string(),
            persona: "Fatima 115".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 115 A".to_string(),
                "Feature 115 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_116".to_string(),
            persona: "Maya 116".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 116 A".to_string(),
                "Feature 116 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_116".to_string(),
            persona: "Carlos 116".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 116 A".to_string(),
                "Feature 116 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_116".to_string(),
            persona: "Priya 116".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 116 A".to_string(),
                "Feature 116 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_116".to_string(),
            persona: "Leo 116".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 116 A".to_string(),
                "Feature 116 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_116".to_string(),
            persona: "Fatima 116".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 116 A".to_string(),
                "Feature 116 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_117".to_string(),
            persona: "Maya 117".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 117 A".to_string(),
                "Feature 117 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_117".to_string(),
            persona: "Carlos 117".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 117 A".to_string(),
                "Feature 117 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_117".to_string(),
            persona: "Priya 117".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 117 A".to_string(),
                "Feature 117 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_117".to_string(),
            persona: "Leo 117".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 117 A".to_string(),
                "Feature 117 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_117".to_string(),
            persona: "Fatima 117".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 117 A".to_string(),
                "Feature 117 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_118".to_string(),
            persona: "Maya 118".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 118 A".to_string(),
                "Feature 118 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_118".to_string(),
            persona: "Carlos 118".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 118 A".to_string(),
                "Feature 118 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_118".to_string(),
            persona: "Priya 118".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 118 A".to_string(),
                "Feature 118 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_118".to_string(),
            persona: "Leo 118".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 118 A".to_string(),
                "Feature 118 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_118".to_string(),
            persona: "Fatima 118".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 118 A".to_string(),
                "Feature 118 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_119".to_string(),
            persona: "Maya 119".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 119 A".to_string(),
                "Feature 119 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_119".to_string(),
            persona: "Carlos 119".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 119 A".to_string(),
                "Feature 119 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_119".to_string(),
            persona: "Priya 119".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 119 A".to_string(),
                "Feature 119 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_119".to_string(),
            persona: "Leo 119".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 119 A".to_string(),
                "Feature 119 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_119".to_string(),
            persona: "Fatima 119".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 119 A".to_string(),
                "Feature 119 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_120".to_string(),
            persona: "Maya 120".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 120 A".to_string(),
                "Feature 120 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_120".to_string(),
            persona: "Carlos 120".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 120 A".to_string(),
                "Feature 120 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_120".to_string(),
            persona: "Priya 120".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 120 A".to_string(),
                "Feature 120 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_120".to_string(),
            persona: "Leo 120".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 120 A".to_string(),
                "Feature 120 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_120".to_string(),
            persona: "Fatima 120".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 120 A".to_string(),
                "Feature 120 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_121".to_string(),
            persona: "Maya 121".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 121 A".to_string(),
                "Feature 121 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_121".to_string(),
            persona: "Carlos 121".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 121 A".to_string(),
                "Feature 121 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_121".to_string(),
            persona: "Priya 121".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 121 A".to_string(),
                "Feature 121 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_121".to_string(),
            persona: "Leo 121".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 121 A".to_string(),
                "Feature 121 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_121".to_string(),
            persona: "Fatima 121".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 121 A".to_string(),
                "Feature 121 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_122".to_string(),
            persona: "Maya 122".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 122 A".to_string(),
                "Feature 122 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_122".to_string(),
            persona: "Carlos 122".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 122 A".to_string(),
                "Feature 122 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_122".to_string(),
            persona: "Priya 122".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 122 A".to_string(),
                "Feature 122 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_122".to_string(),
            persona: "Leo 122".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 122 A".to_string(),
                "Feature 122 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_122".to_string(),
            persona: "Fatima 122".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 122 A".to_string(),
                "Feature 122 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_123".to_string(),
            persona: "Maya 123".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 123 A".to_string(),
                "Feature 123 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_123".to_string(),
            persona: "Carlos 123".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 123 A".to_string(),
                "Feature 123 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_123".to_string(),
            persona: "Priya 123".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 123 A".to_string(),
                "Feature 123 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_123".to_string(),
            persona: "Leo 123".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 123 A".to_string(),
                "Feature 123 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_123".to_string(),
            persona: "Fatima 123".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 123 A".to_string(),
                "Feature 123 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_124".to_string(),
            persona: "Maya 124".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 124 A".to_string(),
                "Feature 124 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_124".to_string(),
            persona: "Carlos 124".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 124 A".to_string(),
                "Feature 124 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_124".to_string(),
            persona: "Priya 124".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 124 A".to_string(),
                "Feature 124 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_124".to_string(),
            persona: "Leo 124".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 124 A".to_string(),
                "Feature 124 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_124".to_string(),
            persona: "Fatima 124".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 124 A".to_string(),
                "Feature 124 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_125".to_string(),
            persona: "Maya 125".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 125 A".to_string(),
                "Feature 125 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_125".to_string(),
            persona: "Carlos 125".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 125 A".to_string(),
                "Feature 125 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_125".to_string(),
            persona: "Priya 125".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 125 A".to_string(),
                "Feature 125 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_125".to_string(),
            persona: "Leo 125".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 125 A".to_string(),
                "Feature 125 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_125".to_string(),
            persona: "Fatima 125".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 125 A".to_string(),
                "Feature 125 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_126".to_string(),
            persona: "Maya 126".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 126 A".to_string(),
                "Feature 126 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_126".to_string(),
            persona: "Carlos 126".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 126 A".to_string(),
                "Feature 126 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_126".to_string(),
            persona: "Priya 126".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 126 A".to_string(),
                "Feature 126 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_126".to_string(),
            persona: "Leo 126".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 126 A".to_string(),
                "Feature 126 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_126".to_string(),
            persona: "Fatima 126".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 126 A".to_string(),
                "Feature 126 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_127".to_string(),
            persona: "Maya 127".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 127 A".to_string(),
                "Feature 127 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_127".to_string(),
            persona: "Carlos 127".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 127 A".to_string(),
                "Feature 127 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_127".to_string(),
            persona: "Priya 127".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 127 A".to_string(),
                "Feature 127 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_127".to_string(),
            persona: "Leo 127".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 127 A".to_string(),
                "Feature 127 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_127".to_string(),
            persona: "Fatima 127".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 127 A".to_string(),
                "Feature 127 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_128".to_string(),
            persona: "Maya 128".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 128 A".to_string(),
                "Feature 128 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_128".to_string(),
            persona: "Carlos 128".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 128 A".to_string(),
                "Feature 128 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_128".to_string(),
            persona: "Priya 128".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 128 A".to_string(),
                "Feature 128 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_128".to_string(),
            persona: "Leo 128".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 128 A".to_string(),
                "Feature 128 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_128".to_string(),
            persona: "Fatima 128".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 128 A".to_string(),
                "Feature 128 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_129".to_string(),
            persona: "Maya 129".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 129 A".to_string(),
                "Feature 129 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_129".to_string(),
            persona: "Carlos 129".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 129 A".to_string(),
                "Feature 129 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_129".to_string(),
            persona: "Priya 129".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 129 A".to_string(),
                "Feature 129 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_129".to_string(),
            persona: "Leo 129".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 129 A".to_string(),
                "Feature 129 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_129".to_string(),
            persona: "Fatima 129".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 129 A".to_string(),
                "Feature 129 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_130".to_string(),
            persona: "Maya 130".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 130 A".to_string(),
                "Feature 130 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_130".to_string(),
            persona: "Carlos 130".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 130 A".to_string(),
                "Feature 130 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_130".to_string(),
            persona: "Priya 130".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 130 A".to_string(),
                "Feature 130 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_130".to_string(),
            persona: "Leo 130".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 130 A".to_string(),
                "Feature 130 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_130".to_string(),
            persona: "Fatima 130".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 130 A".to_string(),
                "Feature 130 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_131".to_string(),
            persona: "Maya 131".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 131 A".to_string(),
                "Feature 131 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_131".to_string(),
            persona: "Carlos 131".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 131 A".to_string(),
                "Feature 131 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_131".to_string(),
            persona: "Priya 131".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 131 A".to_string(),
                "Feature 131 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_131".to_string(),
            persona: "Leo 131".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 131 A".to_string(),
                "Feature 131 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_131".to_string(),
            persona: "Fatima 131".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 131 A".to_string(),
                "Feature 131 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_132".to_string(),
            persona: "Maya 132".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 132 A".to_string(),
                "Feature 132 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_132".to_string(),
            persona: "Carlos 132".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 132 A".to_string(),
                "Feature 132 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_132".to_string(),
            persona: "Priya 132".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 132 A".to_string(),
                "Feature 132 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_132".to_string(),
            persona: "Leo 132".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 132 A".to_string(),
                "Feature 132 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_132".to_string(),
            persona: "Fatima 132".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 132 A".to_string(),
                "Feature 132 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_133".to_string(),
            persona: "Maya 133".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 133 A".to_string(),
                "Feature 133 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_133".to_string(),
            persona: "Carlos 133".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 133 A".to_string(),
                "Feature 133 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_133".to_string(),
            persona: "Priya 133".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 133 A".to_string(),
                "Feature 133 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_133".to_string(),
            persona: "Leo 133".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 133 A".to_string(),
                "Feature 133 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_133".to_string(),
            persona: "Fatima 133".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 133 A".to_string(),
                "Feature 133 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_134".to_string(),
            persona: "Maya 134".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 134 A".to_string(),
                "Feature 134 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_134".to_string(),
            persona: "Carlos 134".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 134 A".to_string(),
                "Feature 134 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_134".to_string(),
            persona: "Priya 134".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 134 A".to_string(),
                "Feature 134 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_134".to_string(),
            persona: "Leo 134".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 134 A".to_string(),
                "Feature 134 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_134".to_string(),
            persona: "Fatima 134".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 134 A".to_string(),
                "Feature 134 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_135".to_string(),
            persona: "Maya 135".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 135 A".to_string(),
                "Feature 135 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_135".to_string(),
            persona: "Carlos 135".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 135 A".to_string(),
                "Feature 135 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_135".to_string(),
            persona: "Priya 135".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 135 A".to_string(),
                "Feature 135 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_135".to_string(),
            persona: "Leo 135".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 135 A".to_string(),
                "Feature 135 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_135".to_string(),
            persona: "Fatima 135".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 135 A".to_string(),
                "Feature 135 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_136".to_string(),
            persona: "Maya 136".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 136 A".to_string(),
                "Feature 136 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_136".to_string(),
            persona: "Carlos 136".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 136 A".to_string(),
                "Feature 136 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_136".to_string(),
            persona: "Priya 136".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 136 A".to_string(),
                "Feature 136 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_136".to_string(),
            persona: "Leo 136".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 136 A".to_string(),
                "Feature 136 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_136".to_string(),
            persona: "Fatima 136".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 136 A".to_string(),
                "Feature 136 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_137".to_string(),
            persona: "Maya 137".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 137 A".to_string(),
                "Feature 137 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_137".to_string(),
            persona: "Carlos 137".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 137 A".to_string(),
                "Feature 137 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_137".to_string(),
            persona: "Priya 137".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 137 A".to_string(),
                "Feature 137 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_137".to_string(),
            persona: "Leo 137".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 137 A".to_string(),
                "Feature 137 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_137".to_string(),
            persona: "Fatima 137".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 137 A".to_string(),
                "Feature 137 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_138".to_string(),
            persona: "Maya 138".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 138 A".to_string(),
                "Feature 138 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_138".to_string(),
            persona: "Carlos 138".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 138 A".to_string(),
                "Feature 138 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_138".to_string(),
            persona: "Priya 138".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 138 A".to_string(),
                "Feature 138 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_138".to_string(),
            persona: "Leo 138".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 138 A".to_string(),
                "Feature 138 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_138".to_string(),
            persona: "Fatima 138".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 138 A".to_string(),
                "Feature 138 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_139".to_string(),
            persona: "Maya 139".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 139 A".to_string(),
                "Feature 139 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_139".to_string(),
            persona: "Carlos 139".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 139 A".to_string(),
                "Feature 139 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_139".to_string(),
            persona: "Priya 139".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 139 A".to_string(),
                "Feature 139 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_139".to_string(),
            persona: "Leo 139".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 139 A".to_string(),
                "Feature 139 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_139".to_string(),
            persona: "Fatima 139".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 139 A".to_string(),
                "Feature 139 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_140".to_string(),
            persona: "Maya 140".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 140 A".to_string(),
                "Feature 140 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_140".to_string(),
            persona: "Carlos 140".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 140 A".to_string(),
                "Feature 140 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_140".to_string(),
            persona: "Priya 140".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 140 A".to_string(),
                "Feature 140 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_140".to_string(),
            persona: "Leo 140".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 140 A".to_string(),
                "Feature 140 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_140".to_string(),
            persona: "Fatima 140".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 140 A".to_string(),
                "Feature 140 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_141".to_string(),
            persona: "Maya 141".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 141 A".to_string(),
                "Feature 141 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_141".to_string(),
            persona: "Carlos 141".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 141 A".to_string(),
                "Feature 141 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_141".to_string(),
            persona: "Priya 141".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 141 A".to_string(),
                "Feature 141 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_141".to_string(),
            persona: "Leo 141".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 141 A".to_string(),
                "Feature 141 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_141".to_string(),
            persona: "Fatima 141".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 141 A".to_string(),
                "Feature 141 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_142".to_string(),
            persona: "Maya 142".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 142 A".to_string(),
                "Feature 142 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_142".to_string(),
            persona: "Carlos 142".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 142 A".to_string(),
                "Feature 142 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_142".to_string(),
            persona: "Priya 142".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 142 A".to_string(),
                "Feature 142 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_142".to_string(),
            persona: "Leo 142".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 142 A".to_string(),
                "Feature 142 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_142".to_string(),
            persona: "Fatima 142".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 142 A".to_string(),
                "Feature 142 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_143".to_string(),
            persona: "Maya 143".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 143 A".to_string(),
                "Feature 143 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_143".to_string(),
            persona: "Carlos 143".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 143 A".to_string(),
                "Feature 143 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_143".to_string(),
            persona: "Priya 143".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 143 A".to_string(),
                "Feature 143 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_143".to_string(),
            persona: "Leo 143".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 143 A".to_string(),
                "Feature 143 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_143".to_string(),
            persona: "Fatima 143".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 143 A".to_string(),
                "Feature 143 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_144".to_string(),
            persona: "Maya 144".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 144 A".to_string(),
                "Feature 144 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_144".to_string(),
            persona: "Carlos 144".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 144 A".to_string(),
                "Feature 144 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_144".to_string(),
            persona: "Priya 144".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 144 A".to_string(),
                "Feature 144 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_144".to_string(),
            persona: "Leo 144".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 144 A".to_string(),
                "Feature 144 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_144".to_string(),
            persona: "Fatima 144".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 144 A".to_string(),
                "Feature 144 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_145".to_string(),
            persona: "Maya 145".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 145 A".to_string(),
                "Feature 145 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_145".to_string(),
            persona: "Carlos 145".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 145 A".to_string(),
                "Feature 145 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_145".to_string(),
            persona: "Priya 145".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 145 A".to_string(),
                "Feature 145 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_145".to_string(),
            persona: "Leo 145".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 145 A".to_string(),
                "Feature 145 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_145".to_string(),
            persona: "Fatima 145".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 145 A".to_string(),
                "Feature 145 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_146".to_string(),
            persona: "Maya 146".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 146 A".to_string(),
                "Feature 146 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_146".to_string(),
            persona: "Carlos 146".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 146 A".to_string(),
                "Feature 146 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_146".to_string(),
            persona: "Priya 146".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 146 A".to_string(),
                "Feature 146 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_146".to_string(),
            persona: "Leo 146".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 146 A".to_string(),
                "Feature 146 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_146".to_string(),
            persona: "Fatima 146".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 146 A".to_string(),
                "Feature 146 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_147".to_string(),
            persona: "Maya 147".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 147 A".to_string(),
                "Feature 147 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_147".to_string(),
            persona: "Carlos 147".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 147 A".to_string(),
                "Feature 147 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_147".to_string(),
            persona: "Priya 147".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 147 A".to_string(),
                "Feature 147 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_147".to_string(),
            persona: "Leo 147".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 147 A".to_string(),
                "Feature 147 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_147".to_string(),
            persona: "Fatima 147".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 147 A".to_string(),
                "Feature 147 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_148".to_string(),
            persona: "Maya 148".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 148 A".to_string(),
                "Feature 148 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_148".to_string(),
            persona: "Carlos 148".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 148 A".to_string(),
                "Feature 148 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_148".to_string(),
            persona: "Priya 148".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 148 A".to_string(),
                "Feature 148 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_148".to_string(),
            persona: "Leo 148".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 148 A".to_string(),
                "Feature 148 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_148".to_string(),
            persona: "Fatima 148".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 148 A".to_string(),
                "Feature 148 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_149".to_string(),
            persona: "Maya 149".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 149 A".to_string(),
                "Feature 149 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_149".to_string(),
            persona: "Carlos 149".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 149 A".to_string(),
                "Feature 149 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_149".to_string(),
            persona: "Priya 149".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 149 A".to_string(),
                "Feature 149 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_149".to_string(),
            persona: "Leo 149".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 149 A".to_string(),
                "Feature 149 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_149".to_string(),
            persona: "Fatima 149".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 149 A".to_string(),
                "Feature 149 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_150".to_string(),
            persona: "Maya 150".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 150 A".to_string(),
                "Feature 150 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_150".to_string(),
            persona: "Carlos 150".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 150 A".to_string(),
                "Feature 150 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_150".to_string(),
            persona: "Priya 150".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 150 A".to_string(),
                "Feature 150 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_150".to_string(),
            persona: "Leo 150".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 150 A".to_string(),
                "Feature 150 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_150".to_string(),
            persona: "Fatima 150".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 150 A".to_string(),
                "Feature 150 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_151".to_string(),
            persona: "Maya 151".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 151 A".to_string(),
                "Feature 151 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_151".to_string(),
            persona: "Carlos 151".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 151 A".to_string(),
                "Feature 151 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_151".to_string(),
            persona: "Priya 151".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 151 A".to_string(),
                "Feature 151 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_151".to_string(),
            persona: "Leo 151".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 151 A".to_string(),
                "Feature 151 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_151".to_string(),
            persona: "Fatima 151".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 151 A".to_string(),
                "Feature 151 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_152".to_string(),
            persona: "Maya 152".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 152 A".to_string(),
                "Feature 152 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_152".to_string(),
            persona: "Carlos 152".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 152 A".to_string(),
                "Feature 152 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_152".to_string(),
            persona: "Priya 152".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 152 A".to_string(),
                "Feature 152 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_152".to_string(),
            persona: "Leo 152".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 152 A".to_string(),
                "Feature 152 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_152".to_string(),
            persona: "Fatima 152".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 152 A".to_string(),
                "Feature 152 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_153".to_string(),
            persona: "Maya 153".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 153 A".to_string(),
                "Feature 153 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_153".to_string(),
            persona: "Carlos 153".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 153 A".to_string(),
                "Feature 153 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_153".to_string(),
            persona: "Priya 153".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 153 A".to_string(),
                "Feature 153 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_153".to_string(),
            persona: "Leo 153".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 153 A".to_string(),
                "Feature 153 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_153".to_string(),
            persona: "Fatima 153".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 153 A".to_string(),
                "Feature 153 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_154".to_string(),
            persona: "Maya 154".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 154 A".to_string(),
                "Feature 154 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_154".to_string(),
            persona: "Carlos 154".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 154 A".to_string(),
                "Feature 154 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_154".to_string(),
            persona: "Priya 154".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 154 A".to_string(),
                "Feature 154 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_154".to_string(),
            persona: "Leo 154".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 154 A".to_string(),
                "Feature 154 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_154".to_string(),
            persona: "Fatima 154".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 154 A".to_string(),
                "Feature 154 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_155".to_string(),
            persona: "Maya 155".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 155 A".to_string(),
                "Feature 155 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_155".to_string(),
            persona: "Carlos 155".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 155 A".to_string(),
                "Feature 155 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_155".to_string(),
            persona: "Priya 155".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 155 A".to_string(),
                "Feature 155 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_155".to_string(),
            persona: "Leo 155".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 155 A".to_string(),
                "Feature 155 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_155".to_string(),
            persona: "Fatima 155".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 155 A".to_string(),
                "Feature 155 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_156".to_string(),
            persona: "Maya 156".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 156 A".to_string(),
                "Feature 156 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_156".to_string(),
            persona: "Carlos 156".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 156 A".to_string(),
                "Feature 156 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_156".to_string(),
            persona: "Priya 156".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 156 A".to_string(),
                "Feature 156 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_156".to_string(),
            persona: "Leo 156".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 156 A".to_string(),
                "Feature 156 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_156".to_string(),
            persona: "Fatima 156".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 156 A".to_string(),
                "Feature 156 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_157".to_string(),
            persona: "Maya 157".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 157 A".to_string(),
                "Feature 157 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_157".to_string(),
            persona: "Carlos 157".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 157 A".to_string(),
                "Feature 157 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_157".to_string(),
            persona: "Priya 157".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 157 A".to_string(),
                "Feature 157 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_157".to_string(),
            persona: "Leo 157".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 157 A".to_string(),
                "Feature 157 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_157".to_string(),
            persona: "Fatima 157".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 157 A".to_string(),
                "Feature 157 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_158".to_string(),
            persona: "Maya 158".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 158 A".to_string(),
                "Feature 158 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_158".to_string(),
            persona: "Carlos 158".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 158 A".to_string(),
                "Feature 158 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_158".to_string(),
            persona: "Priya 158".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 158 A".to_string(),
                "Feature 158 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_158".to_string(),
            persona: "Leo 158".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 158 A".to_string(),
                "Feature 158 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_158".to_string(),
            persona: "Fatima 158".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 158 A".to_string(),
                "Feature 158 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_159".to_string(),
            persona: "Maya 159".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 159 A".to_string(),
                "Feature 159 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_159".to_string(),
            persona: "Carlos 159".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 159 A".to_string(),
                "Feature 159 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_159".to_string(),
            persona: "Priya 159".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 159 A".to_string(),
                "Feature 159 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_159".to_string(),
            persona: "Leo 159".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 159 A".to_string(),
                "Feature 159 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_159".to_string(),
            persona: "Fatima 159".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 159 A".to_string(),
                "Feature 159 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_160".to_string(),
            persona: "Maya 160".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 160 A".to_string(),
                "Feature 160 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_160".to_string(),
            persona: "Carlos 160".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 160 A".to_string(),
                "Feature 160 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_160".to_string(),
            persona: "Priya 160".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 160 A".to_string(),
                "Feature 160 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_160".to_string(),
            persona: "Leo 160".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 160 A".to_string(),
                "Feature 160 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_160".to_string(),
            persona: "Fatima 160".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 160 A".to_string(),
                "Feature 160 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_161".to_string(),
            persona: "Maya 161".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 161 A".to_string(),
                "Feature 161 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_161".to_string(),
            persona: "Carlos 161".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 161 A".to_string(),
                "Feature 161 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_161".to_string(),
            persona: "Priya 161".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 161 A".to_string(),
                "Feature 161 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_161".to_string(),
            persona: "Leo 161".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 161 A".to_string(),
                "Feature 161 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_161".to_string(),
            persona: "Fatima 161".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 161 A".to_string(),
                "Feature 161 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_162".to_string(),
            persona: "Maya 162".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 162 A".to_string(),
                "Feature 162 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_162".to_string(),
            persona: "Carlos 162".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 162 A".to_string(),
                "Feature 162 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_162".to_string(),
            persona: "Priya 162".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 162 A".to_string(),
                "Feature 162 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_162".to_string(),
            persona: "Leo 162".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 162 A".to_string(),
                "Feature 162 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_162".to_string(),
            persona: "Fatima 162".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 162 A".to_string(),
                "Feature 162 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_163".to_string(),
            persona: "Maya 163".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 163 A".to_string(),
                "Feature 163 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_163".to_string(),
            persona: "Carlos 163".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 163 A".to_string(),
                "Feature 163 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_163".to_string(),
            persona: "Priya 163".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 163 A".to_string(),
                "Feature 163 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_163".to_string(),
            persona: "Leo 163".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 163 A".to_string(),
                "Feature 163 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_163".to_string(),
            persona: "Fatima 163".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 163 A".to_string(),
                "Feature 163 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_164".to_string(),
            persona: "Maya 164".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 164 A".to_string(),
                "Feature 164 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_164".to_string(),
            persona: "Carlos 164".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 164 A".to_string(),
                "Feature 164 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_164".to_string(),
            persona: "Priya 164".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 164 A".to_string(),
                "Feature 164 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_164".to_string(),
            persona: "Leo 164".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 164 A".to_string(),
                "Feature 164 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_164".to_string(),
            persona: "Fatima 164".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 164 A".to_string(),
                "Feature 164 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_165".to_string(),
            persona: "Maya 165".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 165 A".to_string(),
                "Feature 165 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_165".to_string(),
            persona: "Carlos 165".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 165 A".to_string(),
                "Feature 165 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_165".to_string(),
            persona: "Priya 165".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 165 A".to_string(),
                "Feature 165 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_165".to_string(),
            persona: "Leo 165".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 165 A".to_string(),
                "Feature 165 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_165".to_string(),
            persona: "Fatima 165".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 165 A".to_string(),
                "Feature 165 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_166".to_string(),
            persona: "Maya 166".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 166 A".to_string(),
                "Feature 166 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_166".to_string(),
            persona: "Carlos 166".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 166 A".to_string(),
                "Feature 166 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_166".to_string(),
            persona: "Priya 166".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 166 A".to_string(),
                "Feature 166 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_166".to_string(),
            persona: "Leo 166".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 166 A".to_string(),
                "Feature 166 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_166".to_string(),
            persona: "Fatima 166".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 166 A".to_string(),
                "Feature 166 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_167".to_string(),
            persona: "Maya 167".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 167 A".to_string(),
                "Feature 167 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_167".to_string(),
            persona: "Carlos 167".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 167 A".to_string(),
                "Feature 167 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_167".to_string(),
            persona: "Priya 167".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 167 A".to_string(),
                "Feature 167 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_167".to_string(),
            persona: "Leo 167".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 167 A".to_string(),
                "Feature 167 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_167".to_string(),
            persona: "Fatima 167".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 167 A".to_string(),
                "Feature 167 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_168".to_string(),
            persona: "Maya 168".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 168 A".to_string(),
                "Feature 168 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_168".to_string(),
            persona: "Carlos 168".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 168 A".to_string(),
                "Feature 168 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_168".to_string(),
            persona: "Priya 168".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 168 A".to_string(),
                "Feature 168 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_168".to_string(),
            persona: "Leo 168".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 168 A".to_string(),
                "Feature 168 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_168".to_string(),
            persona: "Fatima 168".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 168 A".to_string(),
                "Feature 168 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_169".to_string(),
            persona: "Maya 169".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 169 A".to_string(),
                "Feature 169 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_169".to_string(),
            persona: "Carlos 169".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 169 A".to_string(),
                "Feature 169 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_169".to_string(),
            persona: "Priya 169".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 169 A".to_string(),
                "Feature 169 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_169".to_string(),
            persona: "Leo 169".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 169 A".to_string(),
                "Feature 169 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_169".to_string(),
            persona: "Fatima 169".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 169 A".to_string(),
                "Feature 169 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_170".to_string(),
            persona: "Maya 170".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 170 A".to_string(),
                "Feature 170 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_170".to_string(),
            persona: "Carlos 170".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 170 A".to_string(),
                "Feature 170 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_170".to_string(),
            persona: "Priya 170".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 170 A".to_string(),
                "Feature 170 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_170".to_string(),
            persona: "Leo 170".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 170 A".to_string(),
                "Feature 170 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_170".to_string(),
            persona: "Fatima 170".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 170 A".to_string(),
                "Feature 170 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_171".to_string(),
            persona: "Maya 171".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 171 A".to_string(),
                "Feature 171 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_171".to_string(),
            persona: "Carlos 171".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 171 A".to_string(),
                "Feature 171 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_171".to_string(),
            persona: "Priya 171".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 171 A".to_string(),
                "Feature 171 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_171".to_string(),
            persona: "Leo 171".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 171 A".to_string(),
                "Feature 171 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_171".to_string(),
            persona: "Fatima 171".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 171 A".to_string(),
                "Feature 171 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_172".to_string(),
            persona: "Maya 172".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 172 A".to_string(),
                "Feature 172 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_172".to_string(),
            persona: "Carlos 172".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 172 A".to_string(),
                "Feature 172 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_172".to_string(),
            persona: "Priya 172".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 172 A".to_string(),
                "Feature 172 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_172".to_string(),
            persona: "Leo 172".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 172 A".to_string(),
                "Feature 172 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_172".to_string(),
            persona: "Fatima 172".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 172 A".to_string(),
                "Feature 172 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_173".to_string(),
            persona: "Maya 173".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 173 A".to_string(),
                "Feature 173 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_173".to_string(),
            persona: "Carlos 173".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 173 A".to_string(),
                "Feature 173 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_173".to_string(),
            persona: "Priya 173".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 173 A".to_string(),
                "Feature 173 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_173".to_string(),
            persona: "Leo 173".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 173 A".to_string(),
                "Feature 173 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_173".to_string(),
            persona: "Fatima 173".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 173 A".to_string(),
                "Feature 173 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_174".to_string(),
            persona: "Maya 174".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 174 A".to_string(),
                "Feature 174 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_174".to_string(),
            persona: "Carlos 174".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 174 A".to_string(),
                "Feature 174 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_174".to_string(),
            persona: "Priya 174".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 174 A".to_string(),
                "Feature 174 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_174".to_string(),
            persona: "Leo 174".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 174 A".to_string(),
                "Feature 174 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_174".to_string(),
            persona: "Fatima 174".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 174 A".to_string(),
                "Feature 174 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_175".to_string(),
            persona: "Maya 175".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 175 A".to_string(),
                "Feature 175 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_175".to_string(),
            persona: "Carlos 175".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 175 A".to_string(),
                "Feature 175 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_175".to_string(),
            persona: "Priya 175".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 175 A".to_string(),
                "Feature 175 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_175".to_string(),
            persona: "Leo 175".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 175 A".to_string(),
                "Feature 175 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_175".to_string(),
            persona: "Fatima 175".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 175 A".to_string(),
                "Feature 175 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_176".to_string(),
            persona: "Maya 176".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 176 A".to_string(),
                "Feature 176 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_176".to_string(),
            persona: "Carlos 176".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 176 A".to_string(),
                "Feature 176 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_176".to_string(),
            persona: "Priya 176".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 176 A".to_string(),
                "Feature 176 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_176".to_string(),
            persona: "Leo 176".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 176 A".to_string(),
                "Feature 176 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_176".to_string(),
            persona: "Fatima 176".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 176 A".to_string(),
                "Feature 176 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_177".to_string(),
            persona: "Maya 177".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 177 A".to_string(),
                "Feature 177 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_177".to_string(),
            persona: "Carlos 177".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 177 A".to_string(),
                "Feature 177 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_177".to_string(),
            persona: "Priya 177".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 177 A".to_string(),
                "Feature 177 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_177".to_string(),
            persona: "Leo 177".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 177 A".to_string(),
                "Feature 177 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_177".to_string(),
            persona: "Fatima 177".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 177 A".to_string(),
                "Feature 177 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_178".to_string(),
            persona: "Maya 178".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 178 A".to_string(),
                "Feature 178 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_178".to_string(),
            persona: "Carlos 178".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 178 A".to_string(),
                "Feature 178 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_178".to_string(),
            persona: "Priya 178".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 178 A".to_string(),
                "Feature 178 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_178".to_string(),
            persona: "Leo 178".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 178 A".to_string(),
                "Feature 178 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_178".to_string(),
            persona: "Fatima 178".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 178 A".to_string(),
                "Feature 178 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_179".to_string(),
            persona: "Maya 179".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 179 A".to_string(),
                "Feature 179 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_179".to_string(),
            persona: "Carlos 179".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 179 A".to_string(),
                "Feature 179 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_179".to_string(),
            persona: "Priya 179".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 179 A".to_string(),
                "Feature 179 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_179".to_string(),
            persona: "Leo 179".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 179 A".to_string(),
                "Feature 179 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_179".to_string(),
            persona: "Fatima 179".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 179 A".to_string(),
                "Feature 179 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_180".to_string(),
            persona: "Maya 180".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 180 A".to_string(),
                "Feature 180 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_180".to_string(),
            persona: "Carlos 180".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 180 A".to_string(),
                "Feature 180 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_180".to_string(),
            persona: "Priya 180".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 180 A".to_string(),
                "Feature 180 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_180".to_string(),
            persona: "Leo 180".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 180 A".to_string(),
                "Feature 180 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_180".to_string(),
            persona: "Fatima 180".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 180 A".to_string(),
                "Feature 180 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_181".to_string(),
            persona: "Maya 181".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 181 A".to_string(),
                "Feature 181 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_181".to_string(),
            persona: "Carlos 181".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 181 A".to_string(),
                "Feature 181 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_181".to_string(),
            persona: "Priya 181".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 181 A".to_string(),
                "Feature 181 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_181".to_string(),
            persona: "Leo 181".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 181 A".to_string(),
                "Feature 181 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_181".to_string(),
            persona: "Fatima 181".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 181 A".to_string(),
                "Feature 181 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_182".to_string(),
            persona: "Maya 182".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 182 A".to_string(),
                "Feature 182 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_182".to_string(),
            persona: "Carlos 182".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 182 A".to_string(),
                "Feature 182 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_182".to_string(),
            persona: "Priya 182".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 182 A".to_string(),
                "Feature 182 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_182".to_string(),
            persona: "Leo 182".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 182 A".to_string(),
                "Feature 182 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_182".to_string(),
            persona: "Fatima 182".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 182 A".to_string(),
                "Feature 182 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_183".to_string(),
            persona: "Maya 183".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 183 A".to_string(),
                "Feature 183 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_183".to_string(),
            persona: "Carlos 183".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 183 A".to_string(),
                "Feature 183 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_183".to_string(),
            persona: "Priya 183".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 183 A".to_string(),
                "Feature 183 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_183".to_string(),
            persona: "Leo 183".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 183 A".to_string(),
                "Feature 183 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_183".to_string(),
            persona: "Fatima 183".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 183 A".to_string(),
                "Feature 183 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_184".to_string(),
            persona: "Maya 184".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 184 A".to_string(),
                "Feature 184 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_184".to_string(),
            persona: "Carlos 184".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 184 A".to_string(),
                "Feature 184 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_184".to_string(),
            persona: "Priya 184".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 184 A".to_string(),
                "Feature 184 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_184".to_string(),
            persona: "Leo 184".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 184 A".to_string(),
                "Feature 184 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_184".to_string(),
            persona: "Fatima 184".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 184 A".to_string(),
                "Feature 184 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_185".to_string(),
            persona: "Maya 185".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 185 A".to_string(),
                "Feature 185 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_185".to_string(),
            persona: "Carlos 185".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 185 A".to_string(),
                "Feature 185 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_185".to_string(),
            persona: "Priya 185".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 185 A".to_string(),
                "Feature 185 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_185".to_string(),
            persona: "Leo 185".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 185 A".to_string(),
                "Feature 185 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_185".to_string(),
            persona: "Fatima 185".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 185 A".to_string(),
                "Feature 185 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_186".to_string(),
            persona: "Maya 186".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 186 A".to_string(),
                "Feature 186 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_186".to_string(),
            persona: "Carlos 186".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 186 A".to_string(),
                "Feature 186 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_186".to_string(),
            persona: "Priya 186".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 186 A".to_string(),
                "Feature 186 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_186".to_string(),
            persona: "Leo 186".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 186 A".to_string(),
                "Feature 186 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_186".to_string(),
            persona: "Fatima 186".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 186 A".to_string(),
                "Feature 186 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_187".to_string(),
            persona: "Maya 187".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 187 A".to_string(),
                "Feature 187 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_187".to_string(),
            persona: "Carlos 187".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 187 A".to_string(),
                "Feature 187 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_187".to_string(),
            persona: "Priya 187".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 187 A".to_string(),
                "Feature 187 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_187".to_string(),
            persona: "Leo 187".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 187 A".to_string(),
                "Feature 187 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_187".to_string(),
            persona: "Fatima 187".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 187 A".to_string(),
                "Feature 187 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_188".to_string(),
            persona: "Maya 188".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 188 A".to_string(),
                "Feature 188 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_188".to_string(),
            persona: "Carlos 188".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 188 A".to_string(),
                "Feature 188 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_188".to_string(),
            persona: "Priya 188".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 188 A".to_string(),
                "Feature 188 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_188".to_string(),
            persona: "Leo 188".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 188 A".to_string(),
                "Feature 188 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_188".to_string(),
            persona: "Fatima 188".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 188 A".to_string(),
                "Feature 188 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_189".to_string(),
            persona: "Maya 189".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 189 A".to_string(),
                "Feature 189 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_189".to_string(),
            persona: "Carlos 189".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 189 A".to_string(),
                "Feature 189 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_189".to_string(),
            persona: "Priya 189".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 189 A".to_string(),
                "Feature 189 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_189".to_string(),
            persona: "Leo 189".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 189 A".to_string(),
                "Feature 189 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_189".to_string(),
            persona: "Fatima 189".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 189 A".to_string(),
                "Feature 189 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_190".to_string(),
            persona: "Maya 190".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 190 A".to_string(),
                "Feature 190 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_190".to_string(),
            persona: "Carlos 190".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 190 A".to_string(),
                "Feature 190 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_190".to_string(),
            persona: "Priya 190".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 190 A".to_string(),
                "Feature 190 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_190".to_string(),
            persona: "Leo 190".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 190 A".to_string(),
                "Feature 190 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_190".to_string(),
            persona: "Fatima 190".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 190 A".to_string(),
                "Feature 190 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_191".to_string(),
            persona: "Maya 191".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 191 A".to_string(),
                "Feature 191 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_191".to_string(),
            persona: "Carlos 191".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 191 A".to_string(),
                "Feature 191 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_191".to_string(),
            persona: "Priya 191".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 191 A".to_string(),
                "Feature 191 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_191".to_string(),
            persona: "Leo 191".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 191 A".to_string(),
                "Feature 191 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_191".to_string(),
            persona: "Fatima 191".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 191 A".to_string(),
                "Feature 191 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_192".to_string(),
            persona: "Maya 192".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 192 A".to_string(),
                "Feature 192 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_192".to_string(),
            persona: "Carlos 192".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 192 A".to_string(),
                "Feature 192 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_192".to_string(),
            persona: "Priya 192".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 192 A".to_string(),
                "Feature 192 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_192".to_string(),
            persona: "Leo 192".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 192 A".to_string(),
                "Feature 192 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_192".to_string(),
            persona: "Fatima 192".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 192 A".to_string(),
                "Feature 192 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_193".to_string(),
            persona: "Maya 193".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 193 A".to_string(),
                "Feature 193 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_193".to_string(),
            persona: "Carlos 193".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 193 A".to_string(),
                "Feature 193 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_193".to_string(),
            persona: "Priya 193".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 193 A".to_string(),
                "Feature 193 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_193".to_string(),
            persona: "Leo 193".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 193 A".to_string(),
                "Feature 193 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_193".to_string(),
            persona: "Fatima 193".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 193 A".to_string(),
                "Feature 193 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_194".to_string(),
            persona: "Maya 194".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 194 A".to_string(),
                "Feature 194 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_194".to_string(),
            persona: "Carlos 194".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 194 A".to_string(),
                "Feature 194 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_194".to_string(),
            persona: "Priya 194".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 194 A".to_string(),
                "Feature 194 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_194".to_string(),
            persona: "Leo 194".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 194 A".to_string(),
                "Feature 194 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_194".to_string(),
            persona: "Fatima 194".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 194 A".to_string(),
                "Feature 194 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_195".to_string(),
            persona: "Maya 195".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 195 A".to_string(),
                "Feature 195 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_195".to_string(),
            persona: "Carlos 195".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 195 A".to_string(),
                "Feature 195 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_195".to_string(),
            persona: "Priya 195".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 195 A".to_string(),
                "Feature 195 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_195".to_string(),
            persona: "Leo 195".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 195 A".to_string(),
                "Feature 195 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_195".to_string(),
            persona: "Fatima 195".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 195 A".to_string(),
                "Feature 195 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_196".to_string(),
            persona: "Maya 196".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 196 A".to_string(),
                "Feature 196 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_196".to_string(),
            persona: "Carlos 196".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 196 A".to_string(),
                "Feature 196 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_196".to_string(),
            persona: "Priya 196".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 196 A".to_string(),
                "Feature 196 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_196".to_string(),
            persona: "Leo 196".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 196 A".to_string(),
                "Feature 196 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_196".to_string(),
            persona: "Fatima 196".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 196 A".to_string(),
                "Feature 196 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_197".to_string(),
            persona: "Maya 197".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 197 A".to_string(),
                "Feature 197 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_197".to_string(),
            persona: "Carlos 197".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 197 A".to_string(),
                "Feature 197 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_197".to_string(),
            persona: "Priya 197".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 197 A".to_string(),
                "Feature 197 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_197".to_string(),
            persona: "Leo 197".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 197 A".to_string(),
                "Feature 197 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_197".to_string(),
            persona: "Fatima 197".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 197 A".to_string(),
                "Feature 197 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_198".to_string(),
            persona: "Maya 198".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 198 A".to_string(),
                "Feature 198 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_198".to_string(),
            persona: "Carlos 198".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 198 A".to_string(),
                "Feature 198 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_198".to_string(),
            persona: "Priya 198".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 198 A".to_string(),
                "Feature 198 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_198".to_string(),
            persona: "Leo 198".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 198 A".to_string(),
                "Feature 198 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_198".to_string(),
            persona: "Fatima 198".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 198 A".to_string(),
                "Feature 198 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "baker_199".to_string(),
            persona: "Maya 199".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Marketing".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 199 A".to_string(),
                "Feature 199 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "handyman_199".to_string(),
            persona: "Carlos 199".to_string(),
            agents: vec![
                "Sales".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 199 A".to_string(),
                "Feature 199 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "boutique_199".to_string(),
            persona: "Priya 199".to_string(),
            agents: vec![
                "Finance".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 199 A".to_string(),
                "Feature 199 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "tutor_199".to_string(),
            persona: "Leo 199".to_string(),
            agents: vec![
                "Customer Success".to_string(),
                "Operations".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 199 A".to_string(),
                "Feature 199 B".to_string(),
            ],
        },
        BusinessJourney {
            id: "foodcart_199".to_string(),
            persona: "Fatima 199".to_string(),
            agents: vec![
                "Operations".to_string(),
                "Advisory".to_string(),
            ],
            features: vec![
                "Glassmorphism UI".to_string(),
                "Mobile-first 375px wizard".to_string(),
                "Multilingual Support (RTL)".to_string(),
                "Zero technical jargon (Grandmother test)".to_string(),
                "Feature 199 A".to_string(),
                "Feature 199 B".to_string(),
            ],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_business_journeys_len() {
        let journeys = get_business_journeys();
        assert_eq!(journeys.len(), 1000);
    }
    #[test]
    fn test_journey_feature_0() {
        let journeys = get_business_journeys();
        assert!(journeys[0].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_1() {
        let journeys = get_business_journeys();
        assert!(journeys[1].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_2() {
        let journeys = get_business_journeys();
        assert!(journeys[2].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_3() {
        let journeys = get_business_journeys();
        assert!(journeys[3].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_4() {
        let journeys = get_business_journeys();
        assert!(journeys[4].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_5() {
        let journeys = get_business_journeys();
        assert!(journeys[5].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_6() {
        let journeys = get_business_journeys();
        assert!(journeys[6].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_7() {
        let journeys = get_business_journeys();
        assert!(journeys[7].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_8() {
        let journeys = get_business_journeys();
        assert!(journeys[8].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_9() {
        let journeys = get_business_journeys();
        assert!(journeys[9].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_10() {
        let journeys = get_business_journeys();
        assert!(journeys[10].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_11() {
        let journeys = get_business_journeys();
        assert!(journeys[11].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_12() {
        let journeys = get_business_journeys();
        assert!(journeys[12].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_13() {
        let journeys = get_business_journeys();
        assert!(journeys[13].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_14() {
        let journeys = get_business_journeys();
        assert!(journeys[14].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_15() {
        let journeys = get_business_journeys();
        assert!(journeys[15].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_16() {
        let journeys = get_business_journeys();
        assert!(journeys[16].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_17() {
        let journeys = get_business_journeys();
        assert!(journeys[17].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_18() {
        let journeys = get_business_journeys();
        assert!(journeys[18].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_19() {
        let journeys = get_business_journeys();
        assert!(journeys[19].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_20() {
        let journeys = get_business_journeys();
        assert!(journeys[20].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_21() {
        let journeys = get_business_journeys();
        assert!(journeys[21].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_22() {
        let journeys = get_business_journeys();
        assert!(journeys[22].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_23() {
        let journeys = get_business_journeys();
        assert!(journeys[23].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_24() {
        let journeys = get_business_journeys();
        assert!(journeys[24].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_25() {
        let journeys = get_business_journeys();
        assert!(journeys[25].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_26() {
        let journeys = get_business_journeys();
        assert!(journeys[26].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_27() {
        let journeys = get_business_journeys();
        assert!(journeys[27].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_28() {
        let journeys = get_business_journeys();
        assert!(journeys[28].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_29() {
        let journeys = get_business_journeys();
        assert!(journeys[29].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_30() {
        let journeys = get_business_journeys();
        assert!(journeys[30].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_31() {
        let journeys = get_business_journeys();
        assert!(journeys[31].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_32() {
        let journeys = get_business_journeys();
        assert!(journeys[32].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_33() {
        let journeys = get_business_journeys();
        assert!(journeys[33].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_34() {
        let journeys = get_business_journeys();
        assert!(journeys[34].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_35() {
        let journeys = get_business_journeys();
        assert!(journeys[35].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_36() {
        let journeys = get_business_journeys();
        assert!(journeys[36].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_37() {
        let journeys = get_business_journeys();
        assert!(journeys[37].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_38() {
        let journeys = get_business_journeys();
        assert!(journeys[38].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_39() {
        let journeys = get_business_journeys();
        assert!(journeys[39].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_40() {
        let journeys = get_business_journeys();
        assert!(journeys[40].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_41() {
        let journeys = get_business_journeys();
        assert!(journeys[41].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_42() {
        let journeys = get_business_journeys();
        assert!(journeys[42].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_43() {
        let journeys = get_business_journeys();
        assert!(journeys[43].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_44() {
        let journeys = get_business_journeys();
        assert!(journeys[44].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_45() {
        let journeys = get_business_journeys();
        assert!(journeys[45].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_46() {
        let journeys = get_business_journeys();
        assert!(journeys[46].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_47() {
        let journeys = get_business_journeys();
        assert!(journeys[47].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_48() {
        let journeys = get_business_journeys();
        assert!(journeys[48].features.contains(&"Glassmorphism UI".to_string()));
    }
    #[test]
    fn test_journey_feature_49() {
        let journeys = get_business_journeys();
        assert!(journeys[49].features.contains(&"Glassmorphism UI".to_string()));
    }
}
