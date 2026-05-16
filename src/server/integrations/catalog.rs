// Stub module - functionality was removed or moved
// This file exists to satisfy module references that weren't cleaned up


pub struct IntegrationProvider {
    pub metadata: ProviderMetadata,
}

pub struct ProviderMetadata {
    pub id: String,
    pub name: String,
    pub category: String,
    pub base_url: String,
}

pub fn get_catalog() -> Vec<IntegrationProvider> {
    let mut catalog = vec![];

    // We instantiate nats as a placeholder, without making actual network connection
    // since this is used in synchronous `new()` of registry
    let nats_provider = crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "nats".to_string(),
            name: "NATS Event Mesh".to_string(),
            category: "event_mesh".to_string(),
            base_url: "nats://localhost:4222".to_string(),
        }
    };
    catalog.push(nats_provider);

    // We avoid initializing a real TwilioProvider client here just for metadata
    // to prevent unwanted HTTP client instantiation during registry initialization
    let twilio_provider = crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "twilio".to_string(),
            name: "Twilio SMS".to_string(),
            category: "sms".to_string(),
            base_url: "https://api.twilio.com".to_string(),
        }
    };
    catalog.push(twilio_provider);
    let chromadb_provider = crate::integrations::chromadb::provider::ChromaDbProvider::new();
    catalog.push(chromadb_provider.to_integration_provider());

    catalog
}

pub const MOCK_CATEGORIES: &[&str] = &[
    "mock_category_0",
    "mock_category_1",
    "mock_category_2",
    "mock_category_3",
    "mock_category_4",
    "mock_category_5",
    "mock_category_6",
    "mock_category_7",
    "mock_category_8",
    "mock_category_9",
    "mock_category_10",
    "mock_category_11",
    "mock_category_12",
    "mock_category_13",
    "mock_category_14",
    "mock_category_15",
    "mock_category_16",
    "mock_category_17",
    "mock_category_18",
    "mock_category_19",
    "mock_category_20",
    "mock_category_21",
    "mock_category_22",
    "mock_category_23",
    "mock_category_24",
    "mock_category_25",
    "mock_category_26",
    "mock_category_27",
    "mock_category_28",
    "mock_category_29",
    "mock_category_30",
    "mock_category_31",
    "mock_category_32",
    "mock_category_33",
    "mock_category_34",
    "mock_category_35",
    "mock_category_36",
    "mock_category_37",
    "mock_category_38",
    "mock_category_39",
    "mock_category_40",
    "mock_category_41",
    "mock_category_42",
    "mock_category_43",
    "mock_category_44",
    "mock_category_45",
    "mock_category_46",
    "mock_category_47",
    "mock_category_48",
    "mock_category_49",
    "mock_category_50",
    "mock_category_51",
    "mock_category_52",
    "mock_category_53",
    "mock_category_54",
    "mock_category_55",
    "mock_category_56",
    "mock_category_57",
    "mock_category_58",
    "mock_category_59",
    "mock_category_60",
    "mock_category_61",
    "mock_category_62",
    "mock_category_63",
    "mock_category_64",
    "mock_category_65",
    "mock_category_66",
    "mock_category_67",
    "mock_category_68",
    "mock_category_69",
    "mock_category_70",
    "mock_category_71",
    "mock_category_72",
    "mock_category_73",
    "mock_category_74",
    "mock_category_75",
    "mock_category_76",
    "mock_category_77",
    "mock_category_78",
    "mock_category_79",
    "mock_category_80",
    "mock_category_81",
    "mock_category_82",
    "mock_category_83",
    "mock_category_84",
    "mock_category_85",
    "mock_category_86",
    "mock_category_87",
    "mock_category_88",
    "mock_category_89",
    "mock_category_90",
    "mock_category_91",
    "mock_category_92",
    "mock_category_93",
    "mock_category_94",
    "mock_category_95",
    "mock_category_96",
    "mock_category_97",
    "mock_category_98",
    "mock_category_99",
];


pub struct ProviderConfiguration {
    pub key: String,
    pub description: String,
    pub is_required: bool,
}

pub fn get_integration_configs() -> std::collections::HashMap<String, Vec<ProviderConfiguration>> {
    let mut map = std::collections::HashMap::new();
    map.insert("whatsapp".to_string(), vec![
        ProviderConfiguration {
            key: "config_key_whatsapp_0".to_string(),
            description: "Mock configuration parameter 0 for whatsapp".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_whatsapp_1".to_string(),
            description: "Mock configuration parameter 1 for whatsapp".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_whatsapp_2".to_string(),
            description: "Mock configuration parameter 2 for whatsapp".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_whatsapp_3".to_string(),
            description: "Mock configuration parameter 3 for whatsapp".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_whatsapp_4".to_string(),
            description: "Mock configuration parameter 4 for whatsapp".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_whatsapp_5".to_string(),
            description: "Mock configuration parameter 5 for whatsapp".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_whatsapp_6".to_string(),
            description: "Mock configuration parameter 6 for whatsapp".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_whatsapp_7".to_string(),
            description: "Mock configuration parameter 7 for whatsapp".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_whatsapp_8".to_string(),
            description: "Mock configuration parameter 8 for whatsapp".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_whatsapp_9".to_string(),
            description: "Mock configuration parameter 9 for whatsapp".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_whatsapp_10".to_string(),
            description: "Mock configuration parameter 10 for whatsapp".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_whatsapp_11".to_string(),
            description: "Mock configuration parameter 11 for whatsapp".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_whatsapp_12".to_string(),
            description: "Mock configuration parameter 12 for whatsapp".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_whatsapp_13".to_string(),
            description: "Mock configuration parameter 13 for whatsapp".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_whatsapp_14".to_string(),
            description: "Mock configuration parameter 14 for whatsapp".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_whatsapp_15".to_string(),
            description: "Mock configuration parameter 15 for whatsapp".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_whatsapp_16".to_string(),
            description: "Mock configuration parameter 16 for whatsapp".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_whatsapp_17".to_string(),
            description: "Mock configuration parameter 17 for whatsapp".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_whatsapp_18".to_string(),
            description: "Mock configuration parameter 18 for whatsapp".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_whatsapp_19".to_string(),
            description: "Mock configuration parameter 19 for whatsapp".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_whatsapp_20".to_string(),
            description: "Mock configuration parameter 20 for whatsapp".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_whatsapp_21".to_string(),
            description: "Mock configuration parameter 21 for whatsapp".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_whatsapp_22".to_string(),
            description: "Mock configuration parameter 22 for whatsapp".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_whatsapp_23".to_string(),
            description: "Mock configuration parameter 23 for whatsapp".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_whatsapp_24".to_string(),
            description: "Mock configuration parameter 24 for whatsapp".to_string(),
            is_required: true,
        },
    ]);
    map.insert("instagram".to_string(), vec![
        ProviderConfiguration {
            key: "config_key_instagram_0".to_string(),
            description: "Mock configuration parameter 0 for instagram".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_instagram_1".to_string(),
            description: "Mock configuration parameter 1 for instagram".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_instagram_2".to_string(),
            description: "Mock configuration parameter 2 for instagram".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_instagram_3".to_string(),
            description: "Mock configuration parameter 3 for instagram".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_instagram_4".to_string(),
            description: "Mock configuration parameter 4 for instagram".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_instagram_5".to_string(),
            description: "Mock configuration parameter 5 for instagram".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_instagram_6".to_string(),
            description: "Mock configuration parameter 6 for instagram".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_instagram_7".to_string(),
            description: "Mock configuration parameter 7 for instagram".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_instagram_8".to_string(),
            description: "Mock configuration parameter 8 for instagram".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_instagram_9".to_string(),
            description: "Mock configuration parameter 9 for instagram".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_instagram_10".to_string(),
            description: "Mock configuration parameter 10 for instagram".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_instagram_11".to_string(),
            description: "Mock configuration parameter 11 for instagram".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_instagram_12".to_string(),
            description: "Mock configuration parameter 12 for instagram".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_instagram_13".to_string(),
            description: "Mock configuration parameter 13 for instagram".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_instagram_14".to_string(),
            description: "Mock configuration parameter 14 for instagram".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_instagram_15".to_string(),
            description: "Mock configuration parameter 15 for instagram".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_instagram_16".to_string(),
            description: "Mock configuration parameter 16 for instagram".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_instagram_17".to_string(),
            description: "Mock configuration parameter 17 for instagram".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_instagram_18".to_string(),
            description: "Mock configuration parameter 18 for instagram".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_instagram_19".to_string(),
            description: "Mock configuration parameter 19 for instagram".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_instagram_20".to_string(),
            description: "Mock configuration parameter 20 for instagram".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_instagram_21".to_string(),
            description: "Mock configuration parameter 21 for instagram".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_instagram_22".to_string(),
            description: "Mock configuration parameter 22 for instagram".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_instagram_23".to_string(),
            description: "Mock configuration parameter 23 for instagram".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_instagram_24".to_string(),
            description: "Mock configuration parameter 24 for instagram".to_string(),
            is_required: true,
        },
    ]);
    map.insert("google_calendar".to_string(), vec![
        ProviderConfiguration {
            key: "config_key_google_calendar_0".to_string(),
            description: "Mock configuration parameter 0 for google_calendar".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_google_calendar_1".to_string(),
            description: "Mock configuration parameter 1 for google_calendar".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_google_calendar_2".to_string(),
            description: "Mock configuration parameter 2 for google_calendar".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_google_calendar_3".to_string(),
            description: "Mock configuration parameter 3 for google_calendar".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_google_calendar_4".to_string(),
            description: "Mock configuration parameter 4 for google_calendar".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_google_calendar_5".to_string(),
            description: "Mock configuration parameter 5 for google_calendar".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_google_calendar_6".to_string(),
            description: "Mock configuration parameter 6 for google_calendar".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_google_calendar_7".to_string(),
            description: "Mock configuration parameter 7 for google_calendar".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_google_calendar_8".to_string(),
            description: "Mock configuration parameter 8 for google_calendar".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_google_calendar_9".to_string(),
            description: "Mock configuration parameter 9 for google_calendar".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_google_calendar_10".to_string(),
            description: "Mock configuration parameter 10 for google_calendar".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_google_calendar_11".to_string(),
            description: "Mock configuration parameter 11 for google_calendar".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_google_calendar_12".to_string(),
            description: "Mock configuration parameter 12 for google_calendar".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_google_calendar_13".to_string(),
            description: "Mock configuration parameter 13 for google_calendar".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_google_calendar_14".to_string(),
            description: "Mock configuration parameter 14 for google_calendar".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_google_calendar_15".to_string(),
            description: "Mock configuration parameter 15 for google_calendar".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_google_calendar_16".to_string(),
            description: "Mock configuration parameter 16 for google_calendar".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_google_calendar_17".to_string(),
            description: "Mock configuration parameter 17 for google_calendar".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_google_calendar_18".to_string(),
            description: "Mock configuration parameter 18 for google_calendar".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_google_calendar_19".to_string(),
            description: "Mock configuration parameter 19 for google_calendar".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_google_calendar_20".to_string(),
            description: "Mock configuration parameter 20 for google_calendar".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_google_calendar_21".to_string(),
            description: "Mock configuration parameter 21 for google_calendar".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_google_calendar_22".to_string(),
            description: "Mock configuration parameter 22 for google_calendar".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_google_calendar_23".to_string(),
            description: "Mock configuration parameter 23 for google_calendar".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_google_calendar_24".to_string(),
            description: "Mock configuration parameter 24 for google_calendar".to_string(),
            is_required: true,
        },
    ]);
    map.insert("outlook".to_string(), vec![
        ProviderConfiguration {
            key: "config_key_outlook_0".to_string(),
            description: "Mock configuration parameter 0 for outlook".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_outlook_1".to_string(),
            description: "Mock configuration parameter 1 for outlook".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_outlook_2".to_string(),
            description: "Mock configuration parameter 2 for outlook".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_outlook_3".to_string(),
            description: "Mock configuration parameter 3 for outlook".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_outlook_4".to_string(),
            description: "Mock configuration parameter 4 for outlook".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_outlook_5".to_string(),
            description: "Mock configuration parameter 5 for outlook".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_outlook_6".to_string(),
            description: "Mock configuration parameter 6 for outlook".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_outlook_7".to_string(),
            description: "Mock configuration parameter 7 for outlook".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_outlook_8".to_string(),
            description: "Mock configuration parameter 8 for outlook".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_outlook_9".to_string(),
            description: "Mock configuration parameter 9 for outlook".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_outlook_10".to_string(),
            description: "Mock configuration parameter 10 for outlook".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_outlook_11".to_string(),
            description: "Mock configuration parameter 11 for outlook".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_outlook_12".to_string(),
            description: "Mock configuration parameter 12 for outlook".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_outlook_13".to_string(),
            description: "Mock configuration parameter 13 for outlook".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_outlook_14".to_string(),
            description: "Mock configuration parameter 14 for outlook".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_outlook_15".to_string(),
            description: "Mock configuration parameter 15 for outlook".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_outlook_16".to_string(),
            description: "Mock configuration parameter 16 for outlook".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_outlook_17".to_string(),
            description: "Mock configuration parameter 17 for outlook".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_outlook_18".to_string(),
            description: "Mock configuration parameter 18 for outlook".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_outlook_19".to_string(),
            description: "Mock configuration parameter 19 for outlook".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_outlook_20".to_string(),
            description: "Mock configuration parameter 20 for outlook".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_outlook_21".to_string(),
            description: "Mock configuration parameter 21 for outlook".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_outlook_22".to_string(),
            description: "Mock configuration parameter 22 for outlook".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_outlook_23".to_string(),
            description: "Mock configuration parameter 23 for outlook".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_outlook_24".to_string(),
            description: "Mock configuration parameter 24 for outlook".to_string(),
            is_required: true,
        },
    ]);
    map.insert("resend".to_string(), vec![
        ProviderConfiguration {
            key: "config_key_resend_0".to_string(),
            description: "Mock configuration parameter 0 for resend".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_resend_1".to_string(),
            description: "Mock configuration parameter 1 for resend".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_resend_2".to_string(),
            description: "Mock configuration parameter 2 for resend".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_resend_3".to_string(),
            description: "Mock configuration parameter 3 for resend".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_resend_4".to_string(),
            description: "Mock configuration parameter 4 for resend".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_resend_5".to_string(),
            description: "Mock configuration parameter 5 for resend".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_resend_6".to_string(),
            description: "Mock configuration parameter 6 for resend".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_resend_7".to_string(),
            description: "Mock configuration parameter 7 for resend".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_resend_8".to_string(),
            description: "Mock configuration parameter 8 for resend".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_resend_9".to_string(),
            description: "Mock configuration parameter 9 for resend".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_resend_10".to_string(),
            description: "Mock configuration parameter 10 for resend".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_resend_11".to_string(),
            description: "Mock configuration parameter 11 for resend".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_resend_12".to_string(),
            description: "Mock configuration parameter 12 for resend".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_resend_13".to_string(),
            description: "Mock configuration parameter 13 for resend".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_resend_14".to_string(),
            description: "Mock configuration parameter 14 for resend".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_resend_15".to_string(),
            description: "Mock configuration parameter 15 for resend".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_resend_16".to_string(),
            description: "Mock configuration parameter 16 for resend".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_resend_17".to_string(),
            description: "Mock configuration parameter 17 for resend".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_resend_18".to_string(),
            description: "Mock configuration parameter 18 for resend".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_resend_19".to_string(),
            description: "Mock configuration parameter 19 for resend".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_resend_20".to_string(),
            description: "Mock configuration parameter 20 for resend".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_resend_21".to_string(),
            description: "Mock configuration parameter 21 for resend".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_resend_22".to_string(),
            description: "Mock configuration parameter 22 for resend".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_resend_23".to_string(),
            description: "Mock configuration parameter 23 for resend".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_resend_24".to_string(),
            description: "Mock configuration parameter 24 for resend".to_string(),
            is_required: true,
        },
    ]);
    map.insert("mailchimp".to_string(), vec![
        ProviderConfiguration {
            key: "config_key_mailchimp_0".to_string(),
            description: "Mock configuration parameter 0 for mailchimp".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_mailchimp_1".to_string(),
            description: "Mock configuration parameter 1 for mailchimp".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_mailchimp_2".to_string(),
            description: "Mock configuration parameter 2 for mailchimp".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_mailchimp_3".to_string(),
            description: "Mock configuration parameter 3 for mailchimp".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_mailchimp_4".to_string(),
            description: "Mock configuration parameter 4 for mailchimp".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_mailchimp_5".to_string(),
            description: "Mock configuration parameter 5 for mailchimp".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_mailchimp_6".to_string(),
            description: "Mock configuration parameter 6 for mailchimp".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_mailchimp_7".to_string(),
            description: "Mock configuration parameter 7 for mailchimp".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_mailchimp_8".to_string(),
            description: "Mock configuration parameter 8 for mailchimp".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_mailchimp_9".to_string(),
            description: "Mock configuration parameter 9 for mailchimp".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_mailchimp_10".to_string(),
            description: "Mock configuration parameter 10 for mailchimp".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_mailchimp_11".to_string(),
            description: "Mock configuration parameter 11 for mailchimp".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_mailchimp_12".to_string(),
            description: "Mock configuration parameter 12 for mailchimp".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_mailchimp_13".to_string(),
            description: "Mock configuration parameter 13 for mailchimp".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_mailchimp_14".to_string(),
            description: "Mock configuration parameter 14 for mailchimp".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_mailchimp_15".to_string(),
            description: "Mock configuration parameter 15 for mailchimp".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_mailchimp_16".to_string(),
            description: "Mock configuration parameter 16 for mailchimp".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_mailchimp_17".to_string(),
            description: "Mock configuration parameter 17 for mailchimp".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_mailchimp_18".to_string(),
            description: "Mock configuration parameter 18 for mailchimp".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_mailchimp_19".to_string(),
            description: "Mock configuration parameter 19 for mailchimp".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_mailchimp_20".to_string(),
            description: "Mock configuration parameter 20 for mailchimp".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_mailchimp_21".to_string(),
            description: "Mock configuration parameter 21 for mailchimp".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_mailchimp_22".to_string(),
            description: "Mock configuration parameter 22 for mailchimp".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_mailchimp_23".to_string(),
            description: "Mock configuration parameter 23 for mailchimp".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_mailchimp_24".to_string(),
            description: "Mock configuration parameter 24 for mailchimp".to_string(),
            is_required: true,
        },
    ]);
    map.insert("alipay".to_string(), vec![
        ProviderConfiguration {
            key: "config_key_alipay_0".to_string(),
            description: "Mock configuration parameter 0 for alipay".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_alipay_1".to_string(),
            description: "Mock configuration parameter 1 for alipay".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_alipay_2".to_string(),
            description: "Mock configuration parameter 2 for alipay".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_alipay_3".to_string(),
            description: "Mock configuration parameter 3 for alipay".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_alipay_4".to_string(),
            description: "Mock configuration parameter 4 for alipay".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_alipay_5".to_string(),
            description: "Mock configuration parameter 5 for alipay".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_alipay_6".to_string(),
            description: "Mock configuration parameter 6 for alipay".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_alipay_7".to_string(),
            description: "Mock configuration parameter 7 for alipay".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_alipay_8".to_string(),
            description: "Mock configuration parameter 8 for alipay".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_alipay_9".to_string(),
            description: "Mock configuration parameter 9 for alipay".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_alipay_10".to_string(),
            description: "Mock configuration parameter 10 for alipay".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_alipay_11".to_string(),
            description: "Mock configuration parameter 11 for alipay".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_alipay_12".to_string(),
            description: "Mock configuration parameter 12 for alipay".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_alipay_13".to_string(),
            description: "Mock configuration parameter 13 for alipay".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_alipay_14".to_string(),
            description: "Mock configuration parameter 14 for alipay".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_alipay_15".to_string(),
            description: "Mock configuration parameter 15 for alipay".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_alipay_16".to_string(),
            description: "Mock configuration parameter 16 for alipay".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_alipay_17".to_string(),
            description: "Mock configuration parameter 17 for alipay".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_alipay_18".to_string(),
            description: "Mock configuration parameter 18 for alipay".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_alipay_19".to_string(),
            description: "Mock configuration parameter 19 for alipay".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_alipay_20".to_string(),
            description: "Mock configuration parameter 20 for alipay".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_alipay_21".to_string(),
            description: "Mock configuration parameter 21 for alipay".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_alipay_22".to_string(),
            description: "Mock configuration parameter 22 for alipay".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_alipay_23".to_string(),
            description: "Mock configuration parameter 23 for alipay".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_alipay_24".to_string(),
            description: "Mock configuration parameter 24 for alipay".to_string(),
            is_required: true,
        },
    ]);
    map.insert("shippo".to_string(), vec![
        ProviderConfiguration {
            key: "config_key_shippo_0".to_string(),
            description: "Mock configuration parameter 0 for shippo".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_shippo_1".to_string(),
            description: "Mock configuration parameter 1 for shippo".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_shippo_2".to_string(),
            description: "Mock configuration parameter 2 for shippo".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_shippo_3".to_string(),
            description: "Mock configuration parameter 3 for shippo".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_shippo_4".to_string(),
            description: "Mock configuration parameter 4 for shippo".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_shippo_5".to_string(),
            description: "Mock configuration parameter 5 for shippo".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_shippo_6".to_string(),
            description: "Mock configuration parameter 6 for shippo".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_shippo_7".to_string(),
            description: "Mock configuration parameter 7 for shippo".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_shippo_8".to_string(),
            description: "Mock configuration parameter 8 for shippo".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_shippo_9".to_string(),
            description: "Mock configuration parameter 9 for shippo".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_shippo_10".to_string(),
            description: "Mock configuration parameter 10 for shippo".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_shippo_11".to_string(),
            description: "Mock configuration parameter 11 for shippo".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_shippo_12".to_string(),
            description: "Mock configuration parameter 12 for shippo".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_shippo_13".to_string(),
            description: "Mock configuration parameter 13 for shippo".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_shippo_14".to_string(),
            description: "Mock configuration parameter 14 for shippo".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_shippo_15".to_string(),
            description: "Mock configuration parameter 15 for shippo".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_shippo_16".to_string(),
            description: "Mock configuration parameter 16 for shippo".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_shippo_17".to_string(),
            description: "Mock configuration parameter 17 for shippo".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_shippo_18".to_string(),
            description: "Mock configuration parameter 18 for shippo".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_shippo_19".to_string(),
            description: "Mock configuration parameter 19 for shippo".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_shippo_20".to_string(),
            description: "Mock configuration parameter 20 for shippo".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_shippo_21".to_string(),
            description: "Mock configuration parameter 21 for shippo".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_shippo_22".to_string(),
            description: "Mock configuration parameter 22 for shippo".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_shippo_23".to_string(),
            description: "Mock configuration parameter 23 for shippo".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_shippo_24".to_string(),
            description: "Mock configuration parameter 24 for shippo".to_string(),
            is_required: true,
        },
    ]);
    map.insert("easypost".to_string(), vec![
        ProviderConfiguration {
            key: "config_key_easypost_0".to_string(),
            description: "Mock configuration parameter 0 for easypost".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_easypost_1".to_string(),
            description: "Mock configuration parameter 1 for easypost".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_easypost_2".to_string(),
            description: "Mock configuration parameter 2 for easypost".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_easypost_3".to_string(),
            description: "Mock configuration parameter 3 for easypost".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_easypost_4".to_string(),
            description: "Mock configuration parameter 4 for easypost".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_easypost_5".to_string(),
            description: "Mock configuration parameter 5 for easypost".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_easypost_6".to_string(),
            description: "Mock configuration parameter 6 for easypost".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_easypost_7".to_string(),
            description: "Mock configuration parameter 7 for easypost".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_easypost_8".to_string(),
            description: "Mock configuration parameter 8 for easypost".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_easypost_9".to_string(),
            description: "Mock configuration parameter 9 for easypost".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_easypost_10".to_string(),
            description: "Mock configuration parameter 10 for easypost".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_easypost_11".to_string(),
            description: "Mock configuration parameter 11 for easypost".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_easypost_12".to_string(),
            description: "Mock configuration parameter 12 for easypost".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_easypost_13".to_string(),
            description: "Mock configuration parameter 13 for easypost".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_easypost_14".to_string(),
            description: "Mock configuration parameter 14 for easypost".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_easypost_15".to_string(),
            description: "Mock configuration parameter 15 for easypost".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_easypost_16".to_string(),
            description: "Mock configuration parameter 16 for easypost".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_easypost_17".to_string(),
            description: "Mock configuration parameter 17 for easypost".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_easypost_18".to_string(),
            description: "Mock configuration parameter 18 for easypost".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_easypost_19".to_string(),
            description: "Mock configuration parameter 19 for easypost".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_easypost_20".to_string(),
            description: "Mock configuration parameter 20 for easypost".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_easypost_21".to_string(),
            description: "Mock configuration parameter 21 for easypost".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_easypost_22".to_string(),
            description: "Mock configuration parameter 22 for easypost".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_easypost_23".to_string(),
            description: "Mock configuration parameter 23 for easypost".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_easypost_24".to_string(),
            description: "Mock configuration parameter 24 for easypost".to_string(),
            is_required: true,
        },
    ]);
    map.insert("messagebird".to_string(), vec![
        ProviderConfiguration {
            key: "config_key_messagebird_0".to_string(),
            description: "Mock configuration parameter 0 for messagebird".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_messagebird_1".to_string(),
            description: "Mock configuration parameter 1 for messagebird".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_messagebird_2".to_string(),
            description: "Mock configuration parameter 2 for messagebird".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_messagebird_3".to_string(),
            description: "Mock configuration parameter 3 for messagebird".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_messagebird_4".to_string(),
            description: "Mock configuration parameter 4 for messagebird".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_messagebird_5".to_string(),
            description: "Mock configuration parameter 5 for messagebird".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_messagebird_6".to_string(),
            description: "Mock configuration parameter 6 for messagebird".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_messagebird_7".to_string(),
            description: "Mock configuration parameter 7 for messagebird".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_messagebird_8".to_string(),
            description: "Mock configuration parameter 8 for messagebird".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_messagebird_9".to_string(),
            description: "Mock configuration parameter 9 for messagebird".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_messagebird_10".to_string(),
            description: "Mock configuration parameter 10 for messagebird".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_messagebird_11".to_string(),
            description: "Mock configuration parameter 11 for messagebird".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_messagebird_12".to_string(),
            description: "Mock configuration parameter 12 for messagebird".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_messagebird_13".to_string(),
            description: "Mock configuration parameter 13 for messagebird".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_messagebird_14".to_string(),
            description: "Mock configuration parameter 14 for messagebird".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_messagebird_15".to_string(),
            description: "Mock configuration parameter 15 for messagebird".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_messagebird_16".to_string(),
            description: "Mock configuration parameter 16 for messagebird".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_messagebird_17".to_string(),
            description: "Mock configuration parameter 17 for messagebird".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_messagebird_18".to_string(),
            description: "Mock configuration parameter 18 for messagebird".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_messagebird_19".to_string(),
            description: "Mock configuration parameter 19 for messagebird".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_messagebird_20".to_string(),
            description: "Mock configuration parameter 20 for messagebird".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_messagebird_21".to_string(),
            description: "Mock configuration parameter 21 for messagebird".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_messagebird_22".to_string(),
            description: "Mock configuration parameter 22 for messagebird".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_messagebird_23".to_string(),
            description: "Mock configuration parameter 23 for messagebird".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_messagebird_24".to_string(),
            description: "Mock configuration parameter 24 for messagebird".to_string(),
            is_required: true,
        },
    ]);
    map.insert("zoom".to_string(), vec![
        ProviderConfiguration {
            key: "config_key_zoom_0".to_string(),
            description: "Mock configuration parameter 0 for zoom".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_zoom_1".to_string(),
            description: "Mock configuration parameter 1 for zoom".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_zoom_2".to_string(),
            description: "Mock configuration parameter 2 for zoom".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_zoom_3".to_string(),
            description: "Mock configuration parameter 3 for zoom".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_zoom_4".to_string(),
            description: "Mock configuration parameter 4 for zoom".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_zoom_5".to_string(),
            description: "Mock configuration parameter 5 for zoom".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_zoom_6".to_string(),
            description: "Mock configuration parameter 6 for zoom".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_zoom_7".to_string(),
            description: "Mock configuration parameter 7 for zoom".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_zoom_8".to_string(),
            description: "Mock configuration parameter 8 for zoom".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_zoom_9".to_string(),
            description: "Mock configuration parameter 9 for zoom".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_zoom_10".to_string(),
            description: "Mock configuration parameter 10 for zoom".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_zoom_11".to_string(),
            description: "Mock configuration parameter 11 for zoom".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_zoom_12".to_string(),
            description: "Mock configuration parameter 12 for zoom".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_zoom_13".to_string(),
            description: "Mock configuration parameter 13 for zoom".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_zoom_14".to_string(),
            description: "Mock configuration parameter 14 for zoom".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_zoom_15".to_string(),
            description: "Mock configuration parameter 15 for zoom".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_zoom_16".to_string(),
            description: "Mock configuration parameter 16 for zoom".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_zoom_17".to_string(),
            description: "Mock configuration parameter 17 for zoom".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_zoom_18".to_string(),
            description: "Mock configuration parameter 18 for zoom".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_zoom_19".to_string(),
            description: "Mock configuration parameter 19 for zoom".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_zoom_20".to_string(),
            description: "Mock configuration parameter 20 for zoom".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_zoom_21".to_string(),
            description: "Mock configuration parameter 21 for zoom".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_zoom_22".to_string(),
            description: "Mock configuration parameter 22 for zoom".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_zoom_23".to_string(),
            description: "Mock configuration parameter 23 for zoom".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_zoom_24".to_string(),
            description: "Mock configuration parameter 24 for zoom".to_string(),
            is_required: true,
        },
    ]);
    map.insert("google_meet".to_string(), vec![
        ProviderConfiguration {
            key: "config_key_google_meet_0".to_string(),
            description: "Mock configuration parameter 0 for google_meet".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_google_meet_1".to_string(),
            description: "Mock configuration parameter 1 for google_meet".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_google_meet_2".to_string(),
            description: "Mock configuration parameter 2 for google_meet".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_google_meet_3".to_string(),
            description: "Mock configuration parameter 3 for google_meet".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_google_meet_4".to_string(),
            description: "Mock configuration parameter 4 for google_meet".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_google_meet_5".to_string(),
            description: "Mock configuration parameter 5 for google_meet".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_google_meet_6".to_string(),
            description: "Mock configuration parameter 6 for google_meet".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_google_meet_7".to_string(),
            description: "Mock configuration parameter 7 for google_meet".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_google_meet_8".to_string(),
            description: "Mock configuration parameter 8 for google_meet".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_google_meet_9".to_string(),
            description: "Mock configuration parameter 9 for google_meet".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_google_meet_10".to_string(),
            description: "Mock configuration parameter 10 for google_meet".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_google_meet_11".to_string(),
            description: "Mock configuration parameter 11 for google_meet".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_google_meet_12".to_string(),
            description: "Mock configuration parameter 12 for google_meet".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_google_meet_13".to_string(),
            description: "Mock configuration parameter 13 for google_meet".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_google_meet_14".to_string(),
            description: "Mock configuration parameter 14 for google_meet".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_google_meet_15".to_string(),
            description: "Mock configuration parameter 15 for google_meet".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_google_meet_16".to_string(),
            description: "Mock configuration parameter 16 for google_meet".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_google_meet_17".to_string(),
            description: "Mock configuration parameter 17 for google_meet".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_google_meet_18".to_string(),
            description: "Mock configuration parameter 18 for google_meet".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_google_meet_19".to_string(),
            description: "Mock configuration parameter 19 for google_meet".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_google_meet_20".to_string(),
            description: "Mock configuration parameter 20 for google_meet".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_google_meet_21".to_string(),
            description: "Mock configuration parameter 21 for google_meet".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_google_meet_22".to_string(),
            description: "Mock configuration parameter 22 for google_meet".to_string(),
            is_required: true,
        },
        ProviderConfiguration {
            key: "config_key_google_meet_23".to_string(),
            description: "Mock configuration parameter 23 for google_meet".to_string(),
            is_required: false,
        },
        ProviderConfiguration {
            key: "config_key_google_meet_24".to_string(),
            description: "Mock configuration parameter 24 for google_meet".to_string(),
            is_required: true,
        },
    ]);
    map
}

pub struct IntegrationTestData {
    pub payload: String,
    pub scenario: String,
}

pub fn get_mock_test_data() -> Vec<IntegrationTestData> {
    vec![
        IntegrationTestData {
            payload: "{\"mock_payload\": 0, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_0".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 1, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_1".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 2, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_2".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 3, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_3".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 4, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_4".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 5, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_5".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 6, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_6".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 7, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_7".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 8, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_8".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 9, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_9".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 10, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_10".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 11, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_11".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 12, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_12".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 13, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_13".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 14, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_14".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 15, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_15".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 16, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_16".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 17, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_17".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 18, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_18".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 19, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_19".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 20, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_20".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 21, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_21".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 22, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_22".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 23, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_23".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 24, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_24".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 25, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_25".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 26, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_26".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 27, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_27".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 28, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_28".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 29, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_29".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 30, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_30".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 31, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_31".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 32, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_32".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 33, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_33".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 34, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_34".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 35, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_35".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 36, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_36".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 37, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_37".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 38, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_38".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 39, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_39".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 40, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_40".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 41, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_41".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 42, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_42".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 43, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_43".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 44, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_44".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 45, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_45".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 46, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_46".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 47, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_47".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 48, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_48".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 49, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_49".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 50, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_50".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 51, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_51".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 52, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_52".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 53, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_53".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 54, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_54".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 55, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_55".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 56, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_56".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 57, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_57".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 58, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_58".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 59, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_59".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 60, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_60".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 61, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_61".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 62, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_62".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 63, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_63".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 64, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_64".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 65, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_65".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 66, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_66".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 67, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_67".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 68, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_68".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 69, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_69".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 70, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_70".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 71, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_71".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 72, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_72".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 73, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_73".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 74, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_74".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 75, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_75".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 76, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_76".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 77, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_77".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 78, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_78".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 79, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_79".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 80, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_80".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 81, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_81".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 82, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_82".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 83, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_83".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 84, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_84".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 85, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_85".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 86, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_86".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 87, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_87".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 88, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_88".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 89, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_89".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 90, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_90".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 91, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_91".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 92, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_92".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 93, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_93".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 94, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_94".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 95, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_95".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 96, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_96".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 97, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_97".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 98, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_98".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 99, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_99".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 100, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_100".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 101, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_101".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 102, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_102".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 103, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_103".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 104, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_104".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 105, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_105".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 106, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_106".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 107, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_107".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 108, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_108".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 109, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_109".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 110, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_110".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 111, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_111".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 112, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_112".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 113, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_113".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 114, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_114".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 115, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_115".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 116, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_116".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 117, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_117".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 118, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_118".to_string(),
        },
        IntegrationTestData {
            payload: "{\"mock_payload\": 119, \"type\": \"event\"}".to_string(),
            scenario: "test_scenario_119".to_string(),
        },
    ]
}
