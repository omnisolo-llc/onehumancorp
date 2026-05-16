
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaJourney {
    pub name: String,
    pub business_type: String,
    pub initial_state: String,
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
        steps: vec![

            JourneyStep {
                id: "step_1".to_string(),
                question: "Question 1 for Maya?".to_string(),
                ai_action: "Action 1".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_2".to_string(),
                question: "Question 2 for Maya?".to_string(),
                ai_action: "Action 2".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_3".to_string(),
                question: "Question 3 for Maya?".to_string(),
                ai_action: "Action 3".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_4".to_string(),
                question: "Question 4 for Maya?".to_string(),
                ai_action: "Action 4".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_5".to_string(),
                question: "Question 5 for Maya?".to_string(),
                ai_action: "Action 5".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_6".to_string(),
                question: "Question 6 for Maya?".to_string(),
                ai_action: "Action 6".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_7".to_string(),
                question: "Question 7 for Maya?".to_string(),
                ai_action: "Action 7".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_8".to_string(),
                question: "Question 8 for Maya?".to_string(),
                ai_action: "Action 8".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_9".to_string(),
                question: "Question 9 for Maya?".to_string(),
                ai_action: "Action 9".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_10".to_string(),
                question: "Question 10 for Maya?".to_string(),
                ai_action: "Action 10".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_11".to_string(),
                question: "Question 11 for Maya?".to_string(),
                ai_action: "Action 11".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_12".to_string(),
                question: "Question 12 for Maya?".to_string(),
                ai_action: "Action 12".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_13".to_string(),
                question: "Question 13 for Maya?".to_string(),
                ai_action: "Action 13".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_14".to_string(),
                question: "Question 14 for Maya?".to_string(),
                ai_action: "Action 14".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_15".to_string(),
                question: "Question 15 for Maya?".to_string(),
                ai_action: "Action 15".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_16".to_string(),
                question: "Question 16 for Maya?".to_string(),
                ai_action: "Action 16".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_17".to_string(),
                question: "Question 17 for Maya?".to_string(),
                ai_action: "Action 17".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_18".to_string(),
                question: "Question 18 for Maya?".to_string(),
                ai_action: "Action 18".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_19".to_string(),
                question: "Question 19 for Maya?".to_string(),
                ai_action: "Action 19".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_20".to_string(),
                question: "Question 20 for Maya?".to_string(),
                ai_action: "Action 20".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_21".to_string(),
                question: "Question 21 for Maya?".to_string(),
                ai_action: "Action 21".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_22".to_string(),
                question: "Question 22 for Maya?".to_string(),
                ai_action: "Action 22".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_23".to_string(),
                question: "Question 23 for Maya?".to_string(),
                ai_action: "Action 23".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_24".to_string(),
                question: "Question 24 for Maya?".to_string(),
                ai_action: "Action 24".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_25".to_string(),
                question: "Question 25 for Maya?".to_string(),
                ai_action: "Action 25".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_26".to_string(),
                question: "Question 26 for Maya?".to_string(),
                ai_action: "Action 26".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_27".to_string(),
                question: "Question 27 for Maya?".to_string(),
                ai_action: "Action 27".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_28".to_string(),
                question: "Question 28 for Maya?".to_string(),
                ai_action: "Action 28".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_29".to_string(),
                question: "Question 29 for Maya?".to_string(),
                ai_action: "Action 29".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_30".to_string(),
                question: "Question 30 for Maya?".to_string(),
                ai_action: "Action 30".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_31".to_string(),
                question: "Question 31 for Maya?".to_string(),
                ai_action: "Action 31".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_32".to_string(),
                question: "Question 32 for Maya?".to_string(),
                ai_action: "Action 32".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_33".to_string(),
                question: "Question 33 for Maya?".to_string(),
                ai_action: "Action 33".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_34".to_string(),
                question: "Question 34 for Maya?".to_string(),
                ai_action: "Action 34".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_35".to_string(),
                question: "Question 35 for Maya?".to_string(),
                ai_action: "Action 35".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_36".to_string(),
                question: "Question 36 for Maya?".to_string(),
                ai_action: "Action 36".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_37".to_string(),
                question: "Question 37 for Maya?".to_string(),
                ai_action: "Action 37".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_38".to_string(),
                question: "Question 38 for Maya?".to_string(),
                ai_action: "Action 38".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_39".to_string(),
                question: "Question 39 for Maya?".to_string(),
                ai_action: "Action 39".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_40".to_string(),
                question: "Question 40 for Maya?".to_string(),
                ai_action: "Action 40".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_41".to_string(),
                question: "Question 41 for Maya?".to_string(),
                ai_action: "Action 41".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_42".to_string(),
                question: "Question 42 for Maya?".to_string(),
                ai_action: "Action 42".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_43".to_string(),
                question: "Question 43 for Maya?".to_string(),
                ai_action: "Action 43".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_44".to_string(),
                question: "Question 44 for Maya?".to_string(),
                ai_action: "Action 44".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_45".to_string(),
                question: "Question 45 for Maya?".to_string(),
                ai_action: "Action 45".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_46".to_string(),
                question: "Question 46 for Maya?".to_string(),
                ai_action: "Action 46".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_47".to_string(),
                question: "Question 47 for Maya?".to_string(),
                ai_action: "Action 47".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_48".to_string(),
                question: "Question 48 for Maya?".to_string(),
                ai_action: "Action 48".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_49".to_string(),
                question: "Question 49 for Maya?".to_string(),
                ai_action: "Action 49".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_50".to_string(),
                question: "Question 50 for Maya?".to_string(),
                ai_action: "Action 50".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_51".to_string(),
                question: "Question 51 for Maya?".to_string(),
                ai_action: "Action 51".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_52".to_string(),
                question: "Question 52 for Maya?".to_string(),
                ai_action: "Action 52".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_53".to_string(),
                question: "Question 53 for Maya?".to_string(),
                ai_action: "Action 53".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_54".to_string(),
                question: "Question 54 for Maya?".to_string(),
                ai_action: "Action 54".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_55".to_string(),
                question: "Question 55 for Maya?".to_string(),
                ai_action: "Action 55".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_56".to_string(),
                question: "Question 56 for Maya?".to_string(),
                ai_action: "Action 56".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_57".to_string(),
                question: "Question 57 for Maya?".to_string(),
                ai_action: "Action 57".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_58".to_string(),
                question: "Question 58 for Maya?".to_string(),
                ai_action: "Action 58".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_59".to_string(),
                question: "Question 59 for Maya?".to_string(),
                ai_action: "Action 59".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_60".to_string(),
                question: "Question 60 for Maya?".to_string(),
                ai_action: "Action 60".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_61".to_string(),
                question: "Question 61 for Maya?".to_string(),
                ai_action: "Action 61".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_62".to_string(),
                question: "Question 62 for Maya?".to_string(),
                ai_action: "Action 62".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_63".to_string(),
                question: "Question 63 for Maya?".to_string(),
                ai_action: "Action 63".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_64".to_string(),
                question: "Question 64 for Maya?".to_string(),
                ai_action: "Action 64".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_65".to_string(),
                question: "Question 65 for Maya?".to_string(),
                ai_action: "Action 65".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_66".to_string(),
                question: "Question 66 for Maya?".to_string(),
                ai_action: "Action 66".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_67".to_string(),
                question: "Question 67 for Maya?".to_string(),
                ai_action: "Action 67".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_68".to_string(),
                question: "Question 68 for Maya?".to_string(),
                ai_action: "Action 68".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_69".to_string(),
                question: "Question 69 for Maya?".to_string(),
                ai_action: "Action 69".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_70".to_string(),
                question: "Question 70 for Maya?".to_string(),
                ai_action: "Action 70".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_71".to_string(),
                question: "Question 71 for Maya?".to_string(),
                ai_action: "Action 71".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_72".to_string(),
                question: "Question 72 for Maya?".to_string(),
                ai_action: "Action 72".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_73".to_string(),
                question: "Question 73 for Maya?".to_string(),
                ai_action: "Action 73".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_74".to_string(),
                question: "Question 74 for Maya?".to_string(),
                ai_action: "Action 74".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_75".to_string(),
                question: "Question 75 for Maya?".to_string(),
                ai_action: "Action 75".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_76".to_string(),
                question: "Question 76 for Maya?".to_string(),
                ai_action: "Action 76".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_77".to_string(),
                question: "Question 77 for Maya?".to_string(),
                ai_action: "Action 77".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_78".to_string(),
                question: "Question 78 for Maya?".to_string(),
                ai_action: "Action 78".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_79".to_string(),
                question: "Question 79 for Maya?".to_string(),
                ai_action: "Action 79".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_80".to_string(),
                question: "Question 80 for Maya?".to_string(),
                ai_action: "Action 80".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_81".to_string(),
                question: "Question 81 for Maya?".to_string(),
                ai_action: "Action 81".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_82".to_string(),
                question: "Question 82 for Maya?".to_string(),
                ai_action: "Action 82".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_83".to_string(),
                question: "Question 83 for Maya?".to_string(),
                ai_action: "Action 83".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_84".to_string(),
                question: "Question 84 for Maya?".to_string(),
                ai_action: "Action 84".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_85".to_string(),
                question: "Question 85 for Maya?".to_string(),
                ai_action: "Action 85".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_86".to_string(),
                question: "Question 86 for Maya?".to_string(),
                ai_action: "Action 86".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_87".to_string(),
                question: "Question 87 for Maya?".to_string(),
                ai_action: "Action 87".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_88".to_string(),
                question: "Question 88 for Maya?".to_string(),
                ai_action: "Action 88".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_89".to_string(),
                question: "Question 89 for Maya?".to_string(),
                ai_action: "Action 89".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_90".to_string(),
                question: "Question 90 for Maya?".to_string(),
                ai_action: "Action 90".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_91".to_string(),
                question: "Question 91 for Maya?".to_string(),
                ai_action: "Action 91".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_92".to_string(),
                question: "Question 92 for Maya?".to_string(),
                ai_action: "Action 92".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_93".to_string(),
                question: "Question 93 for Maya?".to_string(),
                ai_action: "Action 93".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_94".to_string(),
                question: "Question 94 for Maya?".to_string(),
                ai_action: "Action 94".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_95".to_string(),
                question: "Question 95 for Maya?".to_string(),
                ai_action: "Action 95".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_96".to_string(),
                question: "Question 96 for Maya?".to_string(),
                ai_action: "Action 96".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_97".to_string(),
                question: "Question 97 for Maya?".to_string(),
                ai_action: "Action 97".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_98".to_string(),
                question: "Question 98 for Maya?".to_string(),
                ai_action: "Action 98".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_99".to_string(),
                question: "Question 99 for Maya?".to_string(),
                ai_action: "Action 99".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_100".to_string(),
                question: "Question 100 for Maya?".to_string(),
                ai_action: "Action 100".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_101".to_string(),
                question: "Question 101 for Maya?".to_string(),
                ai_action: "Action 101".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_102".to_string(),
                question: "Question 102 for Maya?".to_string(),
                ai_action: "Action 102".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_103".to_string(),
                question: "Question 103 for Maya?".to_string(),
                ai_action: "Action 103".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_104".to_string(),
                question: "Question 104 for Maya?".to_string(),
                ai_action: "Action 104".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_105".to_string(),
                question: "Question 105 for Maya?".to_string(),
                ai_action: "Action 105".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_106".to_string(),
                question: "Question 106 for Maya?".to_string(),
                ai_action: "Action 106".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_107".to_string(),
                question: "Question 107 for Maya?".to_string(),
                ai_action: "Action 107".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_108".to_string(),
                question: "Question 108 for Maya?".to_string(),
                ai_action: "Action 108".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_109".to_string(),
                question: "Question 109 for Maya?".to_string(),
                ai_action: "Action 109".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_110".to_string(),
                question: "Question 110 for Maya?".to_string(),
                ai_action: "Action 110".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_111".to_string(),
                question: "Question 111 for Maya?".to_string(),
                ai_action: "Action 111".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_112".to_string(),
                question: "Question 112 for Maya?".to_string(),
                ai_action: "Action 112".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_113".to_string(),
                question: "Question 113 for Maya?".to_string(),
                ai_action: "Action 113".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_114".to_string(),
                question: "Question 114 for Maya?".to_string(),
                ai_action: "Action 114".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_115".to_string(),
                question: "Question 115 for Maya?".to_string(),
                ai_action: "Action 115".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_116".to_string(),
                question: "Question 116 for Maya?".to_string(),
                ai_action: "Action 116".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_117".to_string(),
                question: "Question 117 for Maya?".to_string(),
                ai_action: "Action 117".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_118".to_string(),
                question: "Question 118 for Maya?".to_string(),
                ai_action: "Action 118".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_119".to_string(),
                question: "Question 119 for Maya?".to_string(),
                ai_action: "Action 119".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_120".to_string(),
                question: "Question 120 for Maya?".to_string(),
                ai_action: "Action 120".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_121".to_string(),
                question: "Question 121 for Maya?".to_string(),
                ai_action: "Action 121".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_122".to_string(),
                question: "Question 122 for Maya?".to_string(),
                ai_action: "Action 122".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_123".to_string(),
                question: "Question 123 for Maya?".to_string(),
                ai_action: "Action 123".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_124".to_string(),
                question: "Question 124 for Maya?".to_string(),
                ai_action: "Action 124".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_125".to_string(),
                question: "Question 125 for Maya?".to_string(),
                ai_action: "Action 125".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_126".to_string(),
                question: "Question 126 for Maya?".to_string(),
                ai_action: "Action 126".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_127".to_string(),
                question: "Question 127 for Maya?".to_string(),
                ai_action: "Action 127".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_128".to_string(),
                question: "Question 128 for Maya?".to_string(),
                ai_action: "Action 128".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_129".to_string(),
                question: "Question 129 for Maya?".to_string(),
                ai_action: "Action 129".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_130".to_string(),
                question: "Question 130 for Maya?".to_string(),
                ai_action: "Action 130".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_131".to_string(),
                question: "Question 131 for Maya?".to_string(),
                ai_action: "Action 131".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_132".to_string(),
                question: "Question 132 for Maya?".to_string(),
                ai_action: "Action 132".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_133".to_string(),
                question: "Question 133 for Maya?".to_string(),
                ai_action: "Action 133".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_134".to_string(),
                question: "Question 134 for Maya?".to_string(),
                ai_action: "Action 134".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_135".to_string(),
                question: "Question 135 for Maya?".to_string(),
                ai_action: "Action 135".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_136".to_string(),
                question: "Question 136 for Maya?".to_string(),
                ai_action: "Action 136".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_137".to_string(),
                question: "Question 137 for Maya?".to_string(),
                ai_action: "Action 137".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_138".to_string(),
                question: "Question 138 for Maya?".to_string(),
                ai_action: "Action 138".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_139".to_string(),
                question: "Question 139 for Maya?".to_string(),
                ai_action: "Action 139".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_140".to_string(),
                question: "Question 140 for Maya?".to_string(),
                ai_action: "Action 140".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_141".to_string(),
                question: "Question 141 for Maya?".to_string(),
                ai_action: "Action 141".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_142".to_string(),
                question: "Question 142 for Maya?".to_string(),
                ai_action: "Action 142".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_143".to_string(),
                question: "Question 143 for Maya?".to_string(),
                ai_action: "Action 143".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_144".to_string(),
                question: "Question 144 for Maya?".to_string(),
                ai_action: "Action 144".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_145".to_string(),
                question: "Question 145 for Maya?".to_string(),
                ai_action: "Action 145".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_146".to_string(),
                question: "Question 146 for Maya?".to_string(),
                ai_action: "Action 146".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_147".to_string(),
                question: "Question 147 for Maya?".to_string(),
                ai_action: "Action 147".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_148".to_string(),
                question: "Question 148 for Maya?".to_string(),
                ai_action: "Action 148".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_149".to_string(),
                question: "Question 149 for Maya?".to_string(),
                ai_action: "Action 149".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

        ],
    });

    journeys.push(PersonaJourney {
        name: "Carlos".to_string(),
        business_type: "Handyman".to_string(),
        initial_state: "Services & Bookings".to_string(),
        steps: vec![

            JourneyStep {
                id: "step_1".to_string(),
                question: "Question 1 for Carlos?".to_string(),
                ai_action: "Action 1".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_2".to_string(),
                question: "Question 2 for Carlos?".to_string(),
                ai_action: "Action 2".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_3".to_string(),
                question: "Question 3 for Carlos?".to_string(),
                ai_action: "Action 3".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_4".to_string(),
                question: "Question 4 for Carlos?".to_string(),
                ai_action: "Action 4".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_5".to_string(),
                question: "Question 5 for Carlos?".to_string(),
                ai_action: "Action 5".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_6".to_string(),
                question: "Question 6 for Carlos?".to_string(),
                ai_action: "Action 6".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_7".to_string(),
                question: "Question 7 for Carlos?".to_string(),
                ai_action: "Action 7".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_8".to_string(),
                question: "Question 8 for Carlos?".to_string(),
                ai_action: "Action 8".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_9".to_string(),
                question: "Question 9 for Carlos?".to_string(),
                ai_action: "Action 9".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_10".to_string(),
                question: "Question 10 for Carlos?".to_string(),
                ai_action: "Action 10".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_11".to_string(),
                question: "Question 11 for Carlos?".to_string(),
                ai_action: "Action 11".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_12".to_string(),
                question: "Question 12 for Carlos?".to_string(),
                ai_action: "Action 12".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_13".to_string(),
                question: "Question 13 for Carlos?".to_string(),
                ai_action: "Action 13".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_14".to_string(),
                question: "Question 14 for Carlos?".to_string(),
                ai_action: "Action 14".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_15".to_string(),
                question: "Question 15 for Carlos?".to_string(),
                ai_action: "Action 15".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_16".to_string(),
                question: "Question 16 for Carlos?".to_string(),
                ai_action: "Action 16".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_17".to_string(),
                question: "Question 17 for Carlos?".to_string(),
                ai_action: "Action 17".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_18".to_string(),
                question: "Question 18 for Carlos?".to_string(),
                ai_action: "Action 18".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_19".to_string(),
                question: "Question 19 for Carlos?".to_string(),
                ai_action: "Action 19".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_20".to_string(),
                question: "Question 20 for Carlos?".to_string(),
                ai_action: "Action 20".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_21".to_string(),
                question: "Question 21 for Carlos?".to_string(),
                ai_action: "Action 21".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_22".to_string(),
                question: "Question 22 for Carlos?".to_string(),
                ai_action: "Action 22".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_23".to_string(),
                question: "Question 23 for Carlos?".to_string(),
                ai_action: "Action 23".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_24".to_string(),
                question: "Question 24 for Carlos?".to_string(),
                ai_action: "Action 24".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_25".to_string(),
                question: "Question 25 for Carlos?".to_string(),
                ai_action: "Action 25".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_26".to_string(),
                question: "Question 26 for Carlos?".to_string(),
                ai_action: "Action 26".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_27".to_string(),
                question: "Question 27 for Carlos?".to_string(),
                ai_action: "Action 27".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_28".to_string(),
                question: "Question 28 for Carlos?".to_string(),
                ai_action: "Action 28".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_29".to_string(),
                question: "Question 29 for Carlos?".to_string(),
                ai_action: "Action 29".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_30".to_string(),
                question: "Question 30 for Carlos?".to_string(),
                ai_action: "Action 30".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_31".to_string(),
                question: "Question 31 for Carlos?".to_string(),
                ai_action: "Action 31".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_32".to_string(),
                question: "Question 32 for Carlos?".to_string(),
                ai_action: "Action 32".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_33".to_string(),
                question: "Question 33 for Carlos?".to_string(),
                ai_action: "Action 33".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_34".to_string(),
                question: "Question 34 for Carlos?".to_string(),
                ai_action: "Action 34".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_35".to_string(),
                question: "Question 35 for Carlos?".to_string(),
                ai_action: "Action 35".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_36".to_string(),
                question: "Question 36 for Carlos?".to_string(),
                ai_action: "Action 36".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_37".to_string(),
                question: "Question 37 for Carlos?".to_string(),
                ai_action: "Action 37".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_38".to_string(),
                question: "Question 38 for Carlos?".to_string(),
                ai_action: "Action 38".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_39".to_string(),
                question: "Question 39 for Carlos?".to_string(),
                ai_action: "Action 39".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_40".to_string(),
                question: "Question 40 for Carlos?".to_string(),
                ai_action: "Action 40".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_41".to_string(),
                question: "Question 41 for Carlos?".to_string(),
                ai_action: "Action 41".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_42".to_string(),
                question: "Question 42 for Carlos?".to_string(),
                ai_action: "Action 42".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_43".to_string(),
                question: "Question 43 for Carlos?".to_string(),
                ai_action: "Action 43".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_44".to_string(),
                question: "Question 44 for Carlos?".to_string(),
                ai_action: "Action 44".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_45".to_string(),
                question: "Question 45 for Carlos?".to_string(),
                ai_action: "Action 45".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_46".to_string(),
                question: "Question 46 for Carlos?".to_string(),
                ai_action: "Action 46".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_47".to_string(),
                question: "Question 47 for Carlos?".to_string(),
                ai_action: "Action 47".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_48".to_string(),
                question: "Question 48 for Carlos?".to_string(),
                ai_action: "Action 48".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_49".to_string(),
                question: "Question 49 for Carlos?".to_string(),
                ai_action: "Action 49".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_50".to_string(),
                question: "Question 50 for Carlos?".to_string(),
                ai_action: "Action 50".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_51".to_string(),
                question: "Question 51 for Carlos?".to_string(),
                ai_action: "Action 51".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_52".to_string(),
                question: "Question 52 for Carlos?".to_string(),
                ai_action: "Action 52".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_53".to_string(),
                question: "Question 53 for Carlos?".to_string(),
                ai_action: "Action 53".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_54".to_string(),
                question: "Question 54 for Carlos?".to_string(),
                ai_action: "Action 54".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_55".to_string(),
                question: "Question 55 for Carlos?".to_string(),
                ai_action: "Action 55".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_56".to_string(),
                question: "Question 56 for Carlos?".to_string(),
                ai_action: "Action 56".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_57".to_string(),
                question: "Question 57 for Carlos?".to_string(),
                ai_action: "Action 57".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_58".to_string(),
                question: "Question 58 for Carlos?".to_string(),
                ai_action: "Action 58".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_59".to_string(),
                question: "Question 59 for Carlos?".to_string(),
                ai_action: "Action 59".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_60".to_string(),
                question: "Question 60 for Carlos?".to_string(),
                ai_action: "Action 60".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_61".to_string(),
                question: "Question 61 for Carlos?".to_string(),
                ai_action: "Action 61".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_62".to_string(),
                question: "Question 62 for Carlos?".to_string(),
                ai_action: "Action 62".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_63".to_string(),
                question: "Question 63 for Carlos?".to_string(),
                ai_action: "Action 63".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_64".to_string(),
                question: "Question 64 for Carlos?".to_string(),
                ai_action: "Action 64".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_65".to_string(),
                question: "Question 65 for Carlos?".to_string(),
                ai_action: "Action 65".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_66".to_string(),
                question: "Question 66 for Carlos?".to_string(),
                ai_action: "Action 66".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_67".to_string(),
                question: "Question 67 for Carlos?".to_string(),
                ai_action: "Action 67".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_68".to_string(),
                question: "Question 68 for Carlos?".to_string(),
                ai_action: "Action 68".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_69".to_string(),
                question: "Question 69 for Carlos?".to_string(),
                ai_action: "Action 69".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_70".to_string(),
                question: "Question 70 for Carlos?".to_string(),
                ai_action: "Action 70".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_71".to_string(),
                question: "Question 71 for Carlos?".to_string(),
                ai_action: "Action 71".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_72".to_string(),
                question: "Question 72 for Carlos?".to_string(),
                ai_action: "Action 72".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_73".to_string(),
                question: "Question 73 for Carlos?".to_string(),
                ai_action: "Action 73".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_74".to_string(),
                question: "Question 74 for Carlos?".to_string(),
                ai_action: "Action 74".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_75".to_string(),
                question: "Question 75 for Carlos?".to_string(),
                ai_action: "Action 75".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_76".to_string(),
                question: "Question 76 for Carlos?".to_string(),
                ai_action: "Action 76".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_77".to_string(),
                question: "Question 77 for Carlos?".to_string(),
                ai_action: "Action 77".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_78".to_string(),
                question: "Question 78 for Carlos?".to_string(),
                ai_action: "Action 78".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_79".to_string(),
                question: "Question 79 for Carlos?".to_string(),
                ai_action: "Action 79".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_80".to_string(),
                question: "Question 80 for Carlos?".to_string(),
                ai_action: "Action 80".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_81".to_string(),
                question: "Question 81 for Carlos?".to_string(),
                ai_action: "Action 81".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_82".to_string(),
                question: "Question 82 for Carlos?".to_string(),
                ai_action: "Action 82".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_83".to_string(),
                question: "Question 83 for Carlos?".to_string(),
                ai_action: "Action 83".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_84".to_string(),
                question: "Question 84 for Carlos?".to_string(),
                ai_action: "Action 84".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_85".to_string(),
                question: "Question 85 for Carlos?".to_string(),
                ai_action: "Action 85".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_86".to_string(),
                question: "Question 86 for Carlos?".to_string(),
                ai_action: "Action 86".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_87".to_string(),
                question: "Question 87 for Carlos?".to_string(),
                ai_action: "Action 87".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_88".to_string(),
                question: "Question 88 for Carlos?".to_string(),
                ai_action: "Action 88".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_89".to_string(),
                question: "Question 89 for Carlos?".to_string(),
                ai_action: "Action 89".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_90".to_string(),
                question: "Question 90 for Carlos?".to_string(),
                ai_action: "Action 90".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_91".to_string(),
                question: "Question 91 for Carlos?".to_string(),
                ai_action: "Action 91".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_92".to_string(),
                question: "Question 92 for Carlos?".to_string(),
                ai_action: "Action 92".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_93".to_string(),
                question: "Question 93 for Carlos?".to_string(),
                ai_action: "Action 93".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_94".to_string(),
                question: "Question 94 for Carlos?".to_string(),
                ai_action: "Action 94".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_95".to_string(),
                question: "Question 95 for Carlos?".to_string(),
                ai_action: "Action 95".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_96".to_string(),
                question: "Question 96 for Carlos?".to_string(),
                ai_action: "Action 96".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_97".to_string(),
                question: "Question 97 for Carlos?".to_string(),
                ai_action: "Action 97".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_98".to_string(),
                question: "Question 98 for Carlos?".to_string(),
                ai_action: "Action 98".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_99".to_string(),
                question: "Question 99 for Carlos?".to_string(),
                ai_action: "Action 99".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_100".to_string(),
                question: "Question 100 for Carlos?".to_string(),
                ai_action: "Action 100".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_101".to_string(),
                question: "Question 101 for Carlos?".to_string(),
                ai_action: "Action 101".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_102".to_string(),
                question: "Question 102 for Carlos?".to_string(),
                ai_action: "Action 102".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_103".to_string(),
                question: "Question 103 for Carlos?".to_string(),
                ai_action: "Action 103".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_104".to_string(),
                question: "Question 104 for Carlos?".to_string(),
                ai_action: "Action 104".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_105".to_string(),
                question: "Question 105 for Carlos?".to_string(),
                ai_action: "Action 105".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_106".to_string(),
                question: "Question 106 for Carlos?".to_string(),
                ai_action: "Action 106".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_107".to_string(),
                question: "Question 107 for Carlos?".to_string(),
                ai_action: "Action 107".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_108".to_string(),
                question: "Question 108 for Carlos?".to_string(),
                ai_action: "Action 108".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_109".to_string(),
                question: "Question 109 for Carlos?".to_string(),
                ai_action: "Action 109".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_110".to_string(),
                question: "Question 110 for Carlos?".to_string(),
                ai_action: "Action 110".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_111".to_string(),
                question: "Question 111 for Carlos?".to_string(),
                ai_action: "Action 111".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_112".to_string(),
                question: "Question 112 for Carlos?".to_string(),
                ai_action: "Action 112".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_113".to_string(),
                question: "Question 113 for Carlos?".to_string(),
                ai_action: "Action 113".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_114".to_string(),
                question: "Question 114 for Carlos?".to_string(),
                ai_action: "Action 114".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_115".to_string(),
                question: "Question 115 for Carlos?".to_string(),
                ai_action: "Action 115".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_116".to_string(),
                question: "Question 116 for Carlos?".to_string(),
                ai_action: "Action 116".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_117".to_string(),
                question: "Question 117 for Carlos?".to_string(),
                ai_action: "Action 117".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_118".to_string(),
                question: "Question 118 for Carlos?".to_string(),
                ai_action: "Action 118".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_119".to_string(),
                question: "Question 119 for Carlos?".to_string(),
                ai_action: "Action 119".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_120".to_string(),
                question: "Question 120 for Carlos?".to_string(),
                ai_action: "Action 120".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_121".to_string(),
                question: "Question 121 for Carlos?".to_string(),
                ai_action: "Action 121".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_122".to_string(),
                question: "Question 122 for Carlos?".to_string(),
                ai_action: "Action 122".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_123".to_string(),
                question: "Question 123 for Carlos?".to_string(),
                ai_action: "Action 123".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_124".to_string(),
                question: "Question 124 for Carlos?".to_string(),
                ai_action: "Action 124".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_125".to_string(),
                question: "Question 125 for Carlos?".to_string(),
                ai_action: "Action 125".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_126".to_string(),
                question: "Question 126 for Carlos?".to_string(),
                ai_action: "Action 126".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_127".to_string(),
                question: "Question 127 for Carlos?".to_string(),
                ai_action: "Action 127".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_128".to_string(),
                question: "Question 128 for Carlos?".to_string(),
                ai_action: "Action 128".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_129".to_string(),
                question: "Question 129 for Carlos?".to_string(),
                ai_action: "Action 129".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_130".to_string(),
                question: "Question 130 for Carlos?".to_string(),
                ai_action: "Action 130".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_131".to_string(),
                question: "Question 131 for Carlos?".to_string(),
                ai_action: "Action 131".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_132".to_string(),
                question: "Question 132 for Carlos?".to_string(),
                ai_action: "Action 132".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_133".to_string(),
                question: "Question 133 for Carlos?".to_string(),
                ai_action: "Action 133".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_134".to_string(),
                question: "Question 134 for Carlos?".to_string(),
                ai_action: "Action 134".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_135".to_string(),
                question: "Question 135 for Carlos?".to_string(),
                ai_action: "Action 135".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_136".to_string(),
                question: "Question 136 for Carlos?".to_string(),
                ai_action: "Action 136".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_137".to_string(),
                question: "Question 137 for Carlos?".to_string(),
                ai_action: "Action 137".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_138".to_string(),
                question: "Question 138 for Carlos?".to_string(),
                ai_action: "Action 138".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_139".to_string(),
                question: "Question 139 for Carlos?".to_string(),
                ai_action: "Action 139".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_140".to_string(),
                question: "Question 140 for Carlos?".to_string(),
                ai_action: "Action 140".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_141".to_string(),
                question: "Question 141 for Carlos?".to_string(),
                ai_action: "Action 141".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_142".to_string(),
                question: "Question 142 for Carlos?".to_string(),
                ai_action: "Action 142".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_143".to_string(),
                question: "Question 143 for Carlos?".to_string(),
                ai_action: "Action 143".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_144".to_string(),
                question: "Question 144 for Carlos?".to_string(),
                ai_action: "Action 144".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_145".to_string(),
                question: "Question 145 for Carlos?".to_string(),
                ai_action: "Action 145".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_146".to_string(),
                question: "Question 146 for Carlos?".to_string(),
                ai_action: "Action 146".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_147".to_string(),
                question: "Question 147 for Carlos?".to_string(),
                ai_action: "Action 147".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_148".to_string(),
                question: "Question 148 for Carlos?".to_string(),
                ai_action: "Action 148".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_149".to_string(),
                question: "Question 149 for Carlos?".to_string(),
                ai_action: "Action 149".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

        ],
    });

    journeys.push(PersonaJourney {
        name: "Priya".to_string(),
        business_type: "Boutique Owner".to_string(),
        initial_state: "Omnichannel POS".to_string(),
        steps: vec![

            JourneyStep {
                id: "step_1".to_string(),
                question: "Question 1 for Priya?".to_string(),
                ai_action: "Action 1".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_2".to_string(),
                question: "Question 2 for Priya?".to_string(),
                ai_action: "Action 2".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_3".to_string(),
                question: "Question 3 for Priya?".to_string(),
                ai_action: "Action 3".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_4".to_string(),
                question: "Question 4 for Priya?".to_string(),
                ai_action: "Action 4".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_5".to_string(),
                question: "Question 5 for Priya?".to_string(),
                ai_action: "Action 5".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_6".to_string(),
                question: "Question 6 for Priya?".to_string(),
                ai_action: "Action 6".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_7".to_string(),
                question: "Question 7 for Priya?".to_string(),
                ai_action: "Action 7".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_8".to_string(),
                question: "Question 8 for Priya?".to_string(),
                ai_action: "Action 8".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_9".to_string(),
                question: "Question 9 for Priya?".to_string(),
                ai_action: "Action 9".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_10".to_string(),
                question: "Question 10 for Priya?".to_string(),
                ai_action: "Action 10".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_11".to_string(),
                question: "Question 11 for Priya?".to_string(),
                ai_action: "Action 11".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_12".to_string(),
                question: "Question 12 for Priya?".to_string(),
                ai_action: "Action 12".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_13".to_string(),
                question: "Question 13 for Priya?".to_string(),
                ai_action: "Action 13".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_14".to_string(),
                question: "Question 14 for Priya?".to_string(),
                ai_action: "Action 14".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_15".to_string(),
                question: "Question 15 for Priya?".to_string(),
                ai_action: "Action 15".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_16".to_string(),
                question: "Question 16 for Priya?".to_string(),
                ai_action: "Action 16".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_17".to_string(),
                question: "Question 17 for Priya?".to_string(),
                ai_action: "Action 17".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_18".to_string(),
                question: "Question 18 for Priya?".to_string(),
                ai_action: "Action 18".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_19".to_string(),
                question: "Question 19 for Priya?".to_string(),
                ai_action: "Action 19".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_20".to_string(),
                question: "Question 20 for Priya?".to_string(),
                ai_action: "Action 20".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_21".to_string(),
                question: "Question 21 for Priya?".to_string(),
                ai_action: "Action 21".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_22".to_string(),
                question: "Question 22 for Priya?".to_string(),
                ai_action: "Action 22".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_23".to_string(),
                question: "Question 23 for Priya?".to_string(),
                ai_action: "Action 23".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_24".to_string(),
                question: "Question 24 for Priya?".to_string(),
                ai_action: "Action 24".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_25".to_string(),
                question: "Question 25 for Priya?".to_string(),
                ai_action: "Action 25".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_26".to_string(),
                question: "Question 26 for Priya?".to_string(),
                ai_action: "Action 26".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_27".to_string(),
                question: "Question 27 for Priya?".to_string(),
                ai_action: "Action 27".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_28".to_string(),
                question: "Question 28 for Priya?".to_string(),
                ai_action: "Action 28".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_29".to_string(),
                question: "Question 29 for Priya?".to_string(),
                ai_action: "Action 29".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_30".to_string(),
                question: "Question 30 for Priya?".to_string(),
                ai_action: "Action 30".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_31".to_string(),
                question: "Question 31 for Priya?".to_string(),
                ai_action: "Action 31".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_32".to_string(),
                question: "Question 32 for Priya?".to_string(),
                ai_action: "Action 32".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_33".to_string(),
                question: "Question 33 for Priya?".to_string(),
                ai_action: "Action 33".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_34".to_string(),
                question: "Question 34 for Priya?".to_string(),
                ai_action: "Action 34".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_35".to_string(),
                question: "Question 35 for Priya?".to_string(),
                ai_action: "Action 35".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_36".to_string(),
                question: "Question 36 for Priya?".to_string(),
                ai_action: "Action 36".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_37".to_string(),
                question: "Question 37 for Priya?".to_string(),
                ai_action: "Action 37".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_38".to_string(),
                question: "Question 38 for Priya?".to_string(),
                ai_action: "Action 38".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_39".to_string(),
                question: "Question 39 for Priya?".to_string(),
                ai_action: "Action 39".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_40".to_string(),
                question: "Question 40 for Priya?".to_string(),
                ai_action: "Action 40".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_41".to_string(),
                question: "Question 41 for Priya?".to_string(),
                ai_action: "Action 41".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_42".to_string(),
                question: "Question 42 for Priya?".to_string(),
                ai_action: "Action 42".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_43".to_string(),
                question: "Question 43 for Priya?".to_string(),
                ai_action: "Action 43".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_44".to_string(),
                question: "Question 44 for Priya?".to_string(),
                ai_action: "Action 44".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_45".to_string(),
                question: "Question 45 for Priya?".to_string(),
                ai_action: "Action 45".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_46".to_string(),
                question: "Question 46 for Priya?".to_string(),
                ai_action: "Action 46".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_47".to_string(),
                question: "Question 47 for Priya?".to_string(),
                ai_action: "Action 47".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_48".to_string(),
                question: "Question 48 for Priya?".to_string(),
                ai_action: "Action 48".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_49".to_string(),
                question: "Question 49 for Priya?".to_string(),
                ai_action: "Action 49".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_50".to_string(),
                question: "Question 50 for Priya?".to_string(),
                ai_action: "Action 50".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_51".to_string(),
                question: "Question 51 for Priya?".to_string(),
                ai_action: "Action 51".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_52".to_string(),
                question: "Question 52 for Priya?".to_string(),
                ai_action: "Action 52".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_53".to_string(),
                question: "Question 53 for Priya?".to_string(),
                ai_action: "Action 53".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_54".to_string(),
                question: "Question 54 for Priya?".to_string(),
                ai_action: "Action 54".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_55".to_string(),
                question: "Question 55 for Priya?".to_string(),
                ai_action: "Action 55".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_56".to_string(),
                question: "Question 56 for Priya?".to_string(),
                ai_action: "Action 56".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_57".to_string(),
                question: "Question 57 for Priya?".to_string(),
                ai_action: "Action 57".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_58".to_string(),
                question: "Question 58 for Priya?".to_string(),
                ai_action: "Action 58".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_59".to_string(),
                question: "Question 59 for Priya?".to_string(),
                ai_action: "Action 59".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_60".to_string(),
                question: "Question 60 for Priya?".to_string(),
                ai_action: "Action 60".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_61".to_string(),
                question: "Question 61 for Priya?".to_string(),
                ai_action: "Action 61".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_62".to_string(),
                question: "Question 62 for Priya?".to_string(),
                ai_action: "Action 62".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_63".to_string(),
                question: "Question 63 for Priya?".to_string(),
                ai_action: "Action 63".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_64".to_string(),
                question: "Question 64 for Priya?".to_string(),
                ai_action: "Action 64".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_65".to_string(),
                question: "Question 65 for Priya?".to_string(),
                ai_action: "Action 65".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_66".to_string(),
                question: "Question 66 for Priya?".to_string(),
                ai_action: "Action 66".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_67".to_string(),
                question: "Question 67 for Priya?".to_string(),
                ai_action: "Action 67".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_68".to_string(),
                question: "Question 68 for Priya?".to_string(),
                ai_action: "Action 68".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_69".to_string(),
                question: "Question 69 for Priya?".to_string(),
                ai_action: "Action 69".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_70".to_string(),
                question: "Question 70 for Priya?".to_string(),
                ai_action: "Action 70".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_71".to_string(),
                question: "Question 71 for Priya?".to_string(),
                ai_action: "Action 71".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_72".to_string(),
                question: "Question 72 for Priya?".to_string(),
                ai_action: "Action 72".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_73".to_string(),
                question: "Question 73 for Priya?".to_string(),
                ai_action: "Action 73".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_74".to_string(),
                question: "Question 74 for Priya?".to_string(),
                ai_action: "Action 74".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_75".to_string(),
                question: "Question 75 for Priya?".to_string(),
                ai_action: "Action 75".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_76".to_string(),
                question: "Question 76 for Priya?".to_string(),
                ai_action: "Action 76".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_77".to_string(),
                question: "Question 77 for Priya?".to_string(),
                ai_action: "Action 77".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_78".to_string(),
                question: "Question 78 for Priya?".to_string(),
                ai_action: "Action 78".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_79".to_string(),
                question: "Question 79 for Priya?".to_string(),
                ai_action: "Action 79".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_80".to_string(),
                question: "Question 80 for Priya?".to_string(),
                ai_action: "Action 80".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_81".to_string(),
                question: "Question 81 for Priya?".to_string(),
                ai_action: "Action 81".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_82".to_string(),
                question: "Question 82 for Priya?".to_string(),
                ai_action: "Action 82".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_83".to_string(),
                question: "Question 83 for Priya?".to_string(),
                ai_action: "Action 83".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_84".to_string(),
                question: "Question 84 for Priya?".to_string(),
                ai_action: "Action 84".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_85".to_string(),
                question: "Question 85 for Priya?".to_string(),
                ai_action: "Action 85".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_86".to_string(),
                question: "Question 86 for Priya?".to_string(),
                ai_action: "Action 86".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_87".to_string(),
                question: "Question 87 for Priya?".to_string(),
                ai_action: "Action 87".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_88".to_string(),
                question: "Question 88 for Priya?".to_string(),
                ai_action: "Action 88".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_89".to_string(),
                question: "Question 89 for Priya?".to_string(),
                ai_action: "Action 89".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_90".to_string(),
                question: "Question 90 for Priya?".to_string(),
                ai_action: "Action 90".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_91".to_string(),
                question: "Question 91 for Priya?".to_string(),
                ai_action: "Action 91".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_92".to_string(),
                question: "Question 92 for Priya?".to_string(),
                ai_action: "Action 92".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_93".to_string(),
                question: "Question 93 for Priya?".to_string(),
                ai_action: "Action 93".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_94".to_string(),
                question: "Question 94 for Priya?".to_string(),
                ai_action: "Action 94".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_95".to_string(),
                question: "Question 95 for Priya?".to_string(),
                ai_action: "Action 95".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_96".to_string(),
                question: "Question 96 for Priya?".to_string(),
                ai_action: "Action 96".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_97".to_string(),
                question: "Question 97 for Priya?".to_string(),
                ai_action: "Action 97".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_98".to_string(),
                question: "Question 98 for Priya?".to_string(),
                ai_action: "Action 98".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_99".to_string(),
                question: "Question 99 for Priya?".to_string(),
                ai_action: "Action 99".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_100".to_string(),
                question: "Question 100 for Priya?".to_string(),
                ai_action: "Action 100".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_101".to_string(),
                question: "Question 101 for Priya?".to_string(),
                ai_action: "Action 101".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_102".to_string(),
                question: "Question 102 for Priya?".to_string(),
                ai_action: "Action 102".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_103".to_string(),
                question: "Question 103 for Priya?".to_string(),
                ai_action: "Action 103".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_104".to_string(),
                question: "Question 104 for Priya?".to_string(),
                ai_action: "Action 104".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_105".to_string(),
                question: "Question 105 for Priya?".to_string(),
                ai_action: "Action 105".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_106".to_string(),
                question: "Question 106 for Priya?".to_string(),
                ai_action: "Action 106".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_107".to_string(),
                question: "Question 107 for Priya?".to_string(),
                ai_action: "Action 107".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_108".to_string(),
                question: "Question 108 for Priya?".to_string(),
                ai_action: "Action 108".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_109".to_string(),
                question: "Question 109 for Priya?".to_string(),
                ai_action: "Action 109".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_110".to_string(),
                question: "Question 110 for Priya?".to_string(),
                ai_action: "Action 110".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_111".to_string(),
                question: "Question 111 for Priya?".to_string(),
                ai_action: "Action 111".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_112".to_string(),
                question: "Question 112 for Priya?".to_string(),
                ai_action: "Action 112".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_113".to_string(),
                question: "Question 113 for Priya?".to_string(),
                ai_action: "Action 113".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_114".to_string(),
                question: "Question 114 for Priya?".to_string(),
                ai_action: "Action 114".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_115".to_string(),
                question: "Question 115 for Priya?".to_string(),
                ai_action: "Action 115".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_116".to_string(),
                question: "Question 116 for Priya?".to_string(),
                ai_action: "Action 116".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_117".to_string(),
                question: "Question 117 for Priya?".to_string(),
                ai_action: "Action 117".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_118".to_string(),
                question: "Question 118 for Priya?".to_string(),
                ai_action: "Action 118".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_119".to_string(),
                question: "Question 119 for Priya?".to_string(),
                ai_action: "Action 119".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_120".to_string(),
                question: "Question 120 for Priya?".to_string(),
                ai_action: "Action 120".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_121".to_string(),
                question: "Question 121 for Priya?".to_string(),
                ai_action: "Action 121".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_122".to_string(),
                question: "Question 122 for Priya?".to_string(),
                ai_action: "Action 122".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_123".to_string(),
                question: "Question 123 for Priya?".to_string(),
                ai_action: "Action 123".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_124".to_string(),
                question: "Question 124 for Priya?".to_string(),
                ai_action: "Action 124".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_125".to_string(),
                question: "Question 125 for Priya?".to_string(),
                ai_action: "Action 125".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_126".to_string(),
                question: "Question 126 for Priya?".to_string(),
                ai_action: "Action 126".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_127".to_string(),
                question: "Question 127 for Priya?".to_string(),
                ai_action: "Action 127".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_128".to_string(),
                question: "Question 128 for Priya?".to_string(),
                ai_action: "Action 128".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_129".to_string(),
                question: "Question 129 for Priya?".to_string(),
                ai_action: "Action 129".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_130".to_string(),
                question: "Question 130 for Priya?".to_string(),
                ai_action: "Action 130".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_131".to_string(),
                question: "Question 131 for Priya?".to_string(),
                ai_action: "Action 131".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_132".to_string(),
                question: "Question 132 for Priya?".to_string(),
                ai_action: "Action 132".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_133".to_string(),
                question: "Question 133 for Priya?".to_string(),
                ai_action: "Action 133".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_134".to_string(),
                question: "Question 134 for Priya?".to_string(),
                ai_action: "Action 134".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_135".to_string(),
                question: "Question 135 for Priya?".to_string(),
                ai_action: "Action 135".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_136".to_string(),
                question: "Question 136 for Priya?".to_string(),
                ai_action: "Action 136".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_137".to_string(),
                question: "Question 137 for Priya?".to_string(),
                ai_action: "Action 137".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_138".to_string(),
                question: "Question 138 for Priya?".to_string(),
                ai_action: "Action 138".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_139".to_string(),
                question: "Question 139 for Priya?".to_string(),
                ai_action: "Action 139".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_140".to_string(),
                question: "Question 140 for Priya?".to_string(),
                ai_action: "Action 140".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_141".to_string(),
                question: "Question 141 for Priya?".to_string(),
                ai_action: "Action 141".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_142".to_string(),
                question: "Question 142 for Priya?".to_string(),
                ai_action: "Action 142".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_143".to_string(),
                question: "Question 143 for Priya?".to_string(),
                ai_action: "Action 143".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_144".to_string(),
                question: "Question 144 for Priya?".to_string(),
                ai_action: "Action 144".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_145".to_string(),
                question: "Question 145 for Priya?".to_string(),
                ai_action: "Action 145".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_146".to_string(),
                question: "Question 146 for Priya?".to_string(),
                ai_action: "Action 146".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_147".to_string(),
                question: "Question 147 for Priya?".to_string(),
                ai_action: "Action 147".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_148".to_string(),
                question: "Question 148 for Priya?".to_string(),
                ai_action: "Action 148".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_149".to_string(),
                question: "Question 149 for Priya?".to_string(),
                ai_action: "Action 149".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

        ],
    });

    journeys.push(PersonaJourney {
        name: "Leo".to_string(),
        business_type: "Music Tutor".to_string(),
        initial_state: "Subscriptions".to_string(),
        steps: vec![

            JourneyStep {
                id: "step_1".to_string(),
                question: "Question 1 for Leo?".to_string(),
                ai_action: "Action 1".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_2".to_string(),
                question: "Question 2 for Leo?".to_string(),
                ai_action: "Action 2".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_3".to_string(),
                question: "Question 3 for Leo?".to_string(),
                ai_action: "Action 3".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_4".to_string(),
                question: "Question 4 for Leo?".to_string(),
                ai_action: "Action 4".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_5".to_string(),
                question: "Question 5 for Leo?".to_string(),
                ai_action: "Action 5".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_6".to_string(),
                question: "Question 6 for Leo?".to_string(),
                ai_action: "Action 6".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_7".to_string(),
                question: "Question 7 for Leo?".to_string(),
                ai_action: "Action 7".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_8".to_string(),
                question: "Question 8 for Leo?".to_string(),
                ai_action: "Action 8".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_9".to_string(),
                question: "Question 9 for Leo?".to_string(),
                ai_action: "Action 9".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_10".to_string(),
                question: "Question 10 for Leo?".to_string(),
                ai_action: "Action 10".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_11".to_string(),
                question: "Question 11 for Leo?".to_string(),
                ai_action: "Action 11".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_12".to_string(),
                question: "Question 12 for Leo?".to_string(),
                ai_action: "Action 12".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_13".to_string(),
                question: "Question 13 for Leo?".to_string(),
                ai_action: "Action 13".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_14".to_string(),
                question: "Question 14 for Leo?".to_string(),
                ai_action: "Action 14".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_15".to_string(),
                question: "Question 15 for Leo?".to_string(),
                ai_action: "Action 15".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_16".to_string(),
                question: "Question 16 for Leo?".to_string(),
                ai_action: "Action 16".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_17".to_string(),
                question: "Question 17 for Leo?".to_string(),
                ai_action: "Action 17".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_18".to_string(),
                question: "Question 18 for Leo?".to_string(),
                ai_action: "Action 18".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_19".to_string(),
                question: "Question 19 for Leo?".to_string(),
                ai_action: "Action 19".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_20".to_string(),
                question: "Question 20 for Leo?".to_string(),
                ai_action: "Action 20".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_21".to_string(),
                question: "Question 21 for Leo?".to_string(),
                ai_action: "Action 21".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_22".to_string(),
                question: "Question 22 for Leo?".to_string(),
                ai_action: "Action 22".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_23".to_string(),
                question: "Question 23 for Leo?".to_string(),
                ai_action: "Action 23".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_24".to_string(),
                question: "Question 24 for Leo?".to_string(),
                ai_action: "Action 24".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_25".to_string(),
                question: "Question 25 for Leo?".to_string(),
                ai_action: "Action 25".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_26".to_string(),
                question: "Question 26 for Leo?".to_string(),
                ai_action: "Action 26".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_27".to_string(),
                question: "Question 27 for Leo?".to_string(),
                ai_action: "Action 27".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_28".to_string(),
                question: "Question 28 for Leo?".to_string(),
                ai_action: "Action 28".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_29".to_string(),
                question: "Question 29 for Leo?".to_string(),
                ai_action: "Action 29".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_30".to_string(),
                question: "Question 30 for Leo?".to_string(),
                ai_action: "Action 30".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_31".to_string(),
                question: "Question 31 for Leo?".to_string(),
                ai_action: "Action 31".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_32".to_string(),
                question: "Question 32 for Leo?".to_string(),
                ai_action: "Action 32".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_33".to_string(),
                question: "Question 33 for Leo?".to_string(),
                ai_action: "Action 33".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_34".to_string(),
                question: "Question 34 for Leo?".to_string(),
                ai_action: "Action 34".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_35".to_string(),
                question: "Question 35 for Leo?".to_string(),
                ai_action: "Action 35".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_36".to_string(),
                question: "Question 36 for Leo?".to_string(),
                ai_action: "Action 36".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_37".to_string(),
                question: "Question 37 for Leo?".to_string(),
                ai_action: "Action 37".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_38".to_string(),
                question: "Question 38 for Leo?".to_string(),
                ai_action: "Action 38".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_39".to_string(),
                question: "Question 39 for Leo?".to_string(),
                ai_action: "Action 39".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_40".to_string(),
                question: "Question 40 for Leo?".to_string(),
                ai_action: "Action 40".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_41".to_string(),
                question: "Question 41 for Leo?".to_string(),
                ai_action: "Action 41".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_42".to_string(),
                question: "Question 42 for Leo?".to_string(),
                ai_action: "Action 42".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_43".to_string(),
                question: "Question 43 for Leo?".to_string(),
                ai_action: "Action 43".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_44".to_string(),
                question: "Question 44 for Leo?".to_string(),
                ai_action: "Action 44".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_45".to_string(),
                question: "Question 45 for Leo?".to_string(),
                ai_action: "Action 45".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_46".to_string(),
                question: "Question 46 for Leo?".to_string(),
                ai_action: "Action 46".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_47".to_string(),
                question: "Question 47 for Leo?".to_string(),
                ai_action: "Action 47".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_48".to_string(),
                question: "Question 48 for Leo?".to_string(),
                ai_action: "Action 48".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_49".to_string(),
                question: "Question 49 for Leo?".to_string(),
                ai_action: "Action 49".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_50".to_string(),
                question: "Question 50 for Leo?".to_string(),
                ai_action: "Action 50".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_51".to_string(),
                question: "Question 51 for Leo?".to_string(),
                ai_action: "Action 51".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_52".to_string(),
                question: "Question 52 for Leo?".to_string(),
                ai_action: "Action 52".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_53".to_string(),
                question: "Question 53 for Leo?".to_string(),
                ai_action: "Action 53".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_54".to_string(),
                question: "Question 54 for Leo?".to_string(),
                ai_action: "Action 54".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_55".to_string(),
                question: "Question 55 for Leo?".to_string(),
                ai_action: "Action 55".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_56".to_string(),
                question: "Question 56 for Leo?".to_string(),
                ai_action: "Action 56".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_57".to_string(),
                question: "Question 57 for Leo?".to_string(),
                ai_action: "Action 57".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_58".to_string(),
                question: "Question 58 for Leo?".to_string(),
                ai_action: "Action 58".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_59".to_string(),
                question: "Question 59 for Leo?".to_string(),
                ai_action: "Action 59".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_60".to_string(),
                question: "Question 60 for Leo?".to_string(),
                ai_action: "Action 60".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_61".to_string(),
                question: "Question 61 for Leo?".to_string(),
                ai_action: "Action 61".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_62".to_string(),
                question: "Question 62 for Leo?".to_string(),
                ai_action: "Action 62".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_63".to_string(),
                question: "Question 63 for Leo?".to_string(),
                ai_action: "Action 63".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_64".to_string(),
                question: "Question 64 for Leo?".to_string(),
                ai_action: "Action 64".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_65".to_string(),
                question: "Question 65 for Leo?".to_string(),
                ai_action: "Action 65".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_66".to_string(),
                question: "Question 66 for Leo?".to_string(),
                ai_action: "Action 66".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_67".to_string(),
                question: "Question 67 for Leo?".to_string(),
                ai_action: "Action 67".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_68".to_string(),
                question: "Question 68 for Leo?".to_string(),
                ai_action: "Action 68".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_69".to_string(),
                question: "Question 69 for Leo?".to_string(),
                ai_action: "Action 69".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_70".to_string(),
                question: "Question 70 for Leo?".to_string(),
                ai_action: "Action 70".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_71".to_string(),
                question: "Question 71 for Leo?".to_string(),
                ai_action: "Action 71".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_72".to_string(),
                question: "Question 72 for Leo?".to_string(),
                ai_action: "Action 72".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_73".to_string(),
                question: "Question 73 for Leo?".to_string(),
                ai_action: "Action 73".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_74".to_string(),
                question: "Question 74 for Leo?".to_string(),
                ai_action: "Action 74".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_75".to_string(),
                question: "Question 75 for Leo?".to_string(),
                ai_action: "Action 75".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_76".to_string(),
                question: "Question 76 for Leo?".to_string(),
                ai_action: "Action 76".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_77".to_string(),
                question: "Question 77 for Leo?".to_string(),
                ai_action: "Action 77".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_78".to_string(),
                question: "Question 78 for Leo?".to_string(),
                ai_action: "Action 78".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_79".to_string(),
                question: "Question 79 for Leo?".to_string(),
                ai_action: "Action 79".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_80".to_string(),
                question: "Question 80 for Leo?".to_string(),
                ai_action: "Action 80".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_81".to_string(),
                question: "Question 81 for Leo?".to_string(),
                ai_action: "Action 81".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_82".to_string(),
                question: "Question 82 for Leo?".to_string(),
                ai_action: "Action 82".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_83".to_string(),
                question: "Question 83 for Leo?".to_string(),
                ai_action: "Action 83".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_84".to_string(),
                question: "Question 84 for Leo?".to_string(),
                ai_action: "Action 84".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_85".to_string(),
                question: "Question 85 for Leo?".to_string(),
                ai_action: "Action 85".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_86".to_string(),
                question: "Question 86 for Leo?".to_string(),
                ai_action: "Action 86".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_87".to_string(),
                question: "Question 87 for Leo?".to_string(),
                ai_action: "Action 87".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_88".to_string(),
                question: "Question 88 for Leo?".to_string(),
                ai_action: "Action 88".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_89".to_string(),
                question: "Question 89 for Leo?".to_string(),
                ai_action: "Action 89".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_90".to_string(),
                question: "Question 90 for Leo?".to_string(),
                ai_action: "Action 90".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_91".to_string(),
                question: "Question 91 for Leo?".to_string(),
                ai_action: "Action 91".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_92".to_string(),
                question: "Question 92 for Leo?".to_string(),
                ai_action: "Action 92".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_93".to_string(),
                question: "Question 93 for Leo?".to_string(),
                ai_action: "Action 93".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_94".to_string(),
                question: "Question 94 for Leo?".to_string(),
                ai_action: "Action 94".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_95".to_string(),
                question: "Question 95 for Leo?".to_string(),
                ai_action: "Action 95".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_96".to_string(),
                question: "Question 96 for Leo?".to_string(),
                ai_action: "Action 96".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_97".to_string(),
                question: "Question 97 for Leo?".to_string(),
                ai_action: "Action 97".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_98".to_string(),
                question: "Question 98 for Leo?".to_string(),
                ai_action: "Action 98".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_99".to_string(),
                question: "Question 99 for Leo?".to_string(),
                ai_action: "Action 99".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_100".to_string(),
                question: "Question 100 for Leo?".to_string(),
                ai_action: "Action 100".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_101".to_string(),
                question: "Question 101 for Leo?".to_string(),
                ai_action: "Action 101".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_102".to_string(),
                question: "Question 102 for Leo?".to_string(),
                ai_action: "Action 102".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_103".to_string(),
                question: "Question 103 for Leo?".to_string(),
                ai_action: "Action 103".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_104".to_string(),
                question: "Question 104 for Leo?".to_string(),
                ai_action: "Action 104".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_105".to_string(),
                question: "Question 105 for Leo?".to_string(),
                ai_action: "Action 105".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_106".to_string(),
                question: "Question 106 for Leo?".to_string(),
                ai_action: "Action 106".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_107".to_string(),
                question: "Question 107 for Leo?".to_string(),
                ai_action: "Action 107".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_108".to_string(),
                question: "Question 108 for Leo?".to_string(),
                ai_action: "Action 108".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_109".to_string(),
                question: "Question 109 for Leo?".to_string(),
                ai_action: "Action 109".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_110".to_string(),
                question: "Question 110 for Leo?".to_string(),
                ai_action: "Action 110".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_111".to_string(),
                question: "Question 111 for Leo?".to_string(),
                ai_action: "Action 111".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_112".to_string(),
                question: "Question 112 for Leo?".to_string(),
                ai_action: "Action 112".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_113".to_string(),
                question: "Question 113 for Leo?".to_string(),
                ai_action: "Action 113".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_114".to_string(),
                question: "Question 114 for Leo?".to_string(),
                ai_action: "Action 114".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_115".to_string(),
                question: "Question 115 for Leo?".to_string(),
                ai_action: "Action 115".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_116".to_string(),
                question: "Question 116 for Leo?".to_string(),
                ai_action: "Action 116".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_117".to_string(),
                question: "Question 117 for Leo?".to_string(),
                ai_action: "Action 117".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_118".to_string(),
                question: "Question 118 for Leo?".to_string(),
                ai_action: "Action 118".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_119".to_string(),
                question: "Question 119 for Leo?".to_string(),
                ai_action: "Action 119".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_120".to_string(),
                question: "Question 120 for Leo?".to_string(),
                ai_action: "Action 120".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_121".to_string(),
                question: "Question 121 for Leo?".to_string(),
                ai_action: "Action 121".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_122".to_string(),
                question: "Question 122 for Leo?".to_string(),
                ai_action: "Action 122".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_123".to_string(),
                question: "Question 123 for Leo?".to_string(),
                ai_action: "Action 123".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_124".to_string(),
                question: "Question 124 for Leo?".to_string(),
                ai_action: "Action 124".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_125".to_string(),
                question: "Question 125 for Leo?".to_string(),
                ai_action: "Action 125".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_126".to_string(),
                question: "Question 126 for Leo?".to_string(),
                ai_action: "Action 126".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_127".to_string(),
                question: "Question 127 for Leo?".to_string(),
                ai_action: "Action 127".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_128".to_string(),
                question: "Question 128 for Leo?".to_string(),
                ai_action: "Action 128".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_129".to_string(),
                question: "Question 129 for Leo?".to_string(),
                ai_action: "Action 129".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_130".to_string(),
                question: "Question 130 for Leo?".to_string(),
                ai_action: "Action 130".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_131".to_string(),
                question: "Question 131 for Leo?".to_string(),
                ai_action: "Action 131".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_132".to_string(),
                question: "Question 132 for Leo?".to_string(),
                ai_action: "Action 132".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_133".to_string(),
                question: "Question 133 for Leo?".to_string(),
                ai_action: "Action 133".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_134".to_string(),
                question: "Question 134 for Leo?".to_string(),
                ai_action: "Action 134".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_135".to_string(),
                question: "Question 135 for Leo?".to_string(),
                ai_action: "Action 135".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_136".to_string(),
                question: "Question 136 for Leo?".to_string(),
                ai_action: "Action 136".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_137".to_string(),
                question: "Question 137 for Leo?".to_string(),
                ai_action: "Action 137".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_138".to_string(),
                question: "Question 138 for Leo?".to_string(),
                ai_action: "Action 138".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_139".to_string(),
                question: "Question 139 for Leo?".to_string(),
                ai_action: "Action 139".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_140".to_string(),
                question: "Question 140 for Leo?".to_string(),
                ai_action: "Action 140".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_141".to_string(),
                question: "Question 141 for Leo?".to_string(),
                ai_action: "Action 141".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_142".to_string(),
                question: "Question 142 for Leo?".to_string(),
                ai_action: "Action 142".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_143".to_string(),
                question: "Question 143 for Leo?".to_string(),
                ai_action: "Action 143".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_144".to_string(),
                question: "Question 144 for Leo?".to_string(),
                ai_action: "Action 144".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_145".to_string(),
                question: "Question 145 for Leo?".to_string(),
                ai_action: "Action 145".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_146".to_string(),
                question: "Question 146 for Leo?".to_string(),
                ai_action: "Action 146".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_147".to_string(),
                question: "Question 147 for Leo?".to_string(),
                ai_action: "Action 147".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_148".to_string(),
                question: "Question 148 for Leo?".to_string(),
                ai_action: "Action 148".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_149".to_string(),
                question: "Question 149 for Leo?".to_string(),
                ai_action: "Action 149".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

        ],
    });

    journeys.push(PersonaJourney {
        name: "Fatima".to_string(),
        business_type: "Food Cart".to_string(),
        initial_state: "Pre-Orders".to_string(),
        steps: vec![

            JourneyStep {
                id: "step_1".to_string(),
                question: "Question 1 for Fatima?".to_string(),
                ai_action: "Action 1".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_2".to_string(),
                question: "Question 2 for Fatima?".to_string(),
                ai_action: "Action 2".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_3".to_string(),
                question: "Question 3 for Fatima?".to_string(),
                ai_action: "Action 3".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_4".to_string(),
                question: "Question 4 for Fatima?".to_string(),
                ai_action: "Action 4".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_5".to_string(),
                question: "Question 5 for Fatima?".to_string(),
                ai_action: "Action 5".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_6".to_string(),
                question: "Question 6 for Fatima?".to_string(),
                ai_action: "Action 6".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_7".to_string(),
                question: "Question 7 for Fatima?".to_string(),
                ai_action: "Action 7".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_8".to_string(),
                question: "Question 8 for Fatima?".to_string(),
                ai_action: "Action 8".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_9".to_string(),
                question: "Question 9 for Fatima?".to_string(),
                ai_action: "Action 9".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_10".to_string(),
                question: "Question 10 for Fatima?".to_string(),
                ai_action: "Action 10".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_11".to_string(),
                question: "Question 11 for Fatima?".to_string(),
                ai_action: "Action 11".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_12".to_string(),
                question: "Question 12 for Fatima?".to_string(),
                ai_action: "Action 12".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_13".to_string(),
                question: "Question 13 for Fatima?".to_string(),
                ai_action: "Action 13".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_14".to_string(),
                question: "Question 14 for Fatima?".to_string(),
                ai_action: "Action 14".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_15".to_string(),
                question: "Question 15 for Fatima?".to_string(),
                ai_action: "Action 15".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_16".to_string(),
                question: "Question 16 for Fatima?".to_string(),
                ai_action: "Action 16".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_17".to_string(),
                question: "Question 17 for Fatima?".to_string(),
                ai_action: "Action 17".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_18".to_string(),
                question: "Question 18 for Fatima?".to_string(),
                ai_action: "Action 18".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_19".to_string(),
                question: "Question 19 for Fatima?".to_string(),
                ai_action: "Action 19".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_20".to_string(),
                question: "Question 20 for Fatima?".to_string(),
                ai_action: "Action 20".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_21".to_string(),
                question: "Question 21 for Fatima?".to_string(),
                ai_action: "Action 21".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_22".to_string(),
                question: "Question 22 for Fatima?".to_string(),
                ai_action: "Action 22".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_23".to_string(),
                question: "Question 23 for Fatima?".to_string(),
                ai_action: "Action 23".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_24".to_string(),
                question: "Question 24 for Fatima?".to_string(),
                ai_action: "Action 24".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_25".to_string(),
                question: "Question 25 for Fatima?".to_string(),
                ai_action: "Action 25".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_26".to_string(),
                question: "Question 26 for Fatima?".to_string(),
                ai_action: "Action 26".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_27".to_string(),
                question: "Question 27 for Fatima?".to_string(),
                ai_action: "Action 27".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_28".to_string(),
                question: "Question 28 for Fatima?".to_string(),
                ai_action: "Action 28".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_29".to_string(),
                question: "Question 29 for Fatima?".to_string(),
                ai_action: "Action 29".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_30".to_string(),
                question: "Question 30 for Fatima?".to_string(),
                ai_action: "Action 30".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_31".to_string(),
                question: "Question 31 for Fatima?".to_string(),
                ai_action: "Action 31".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_32".to_string(),
                question: "Question 32 for Fatima?".to_string(),
                ai_action: "Action 32".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_33".to_string(),
                question: "Question 33 for Fatima?".to_string(),
                ai_action: "Action 33".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_34".to_string(),
                question: "Question 34 for Fatima?".to_string(),
                ai_action: "Action 34".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_35".to_string(),
                question: "Question 35 for Fatima?".to_string(),
                ai_action: "Action 35".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_36".to_string(),
                question: "Question 36 for Fatima?".to_string(),
                ai_action: "Action 36".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_37".to_string(),
                question: "Question 37 for Fatima?".to_string(),
                ai_action: "Action 37".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_38".to_string(),
                question: "Question 38 for Fatima?".to_string(),
                ai_action: "Action 38".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_39".to_string(),
                question: "Question 39 for Fatima?".to_string(),
                ai_action: "Action 39".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_40".to_string(),
                question: "Question 40 for Fatima?".to_string(),
                ai_action: "Action 40".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_41".to_string(),
                question: "Question 41 for Fatima?".to_string(),
                ai_action: "Action 41".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_42".to_string(),
                question: "Question 42 for Fatima?".to_string(),
                ai_action: "Action 42".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_43".to_string(),
                question: "Question 43 for Fatima?".to_string(),
                ai_action: "Action 43".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_44".to_string(),
                question: "Question 44 for Fatima?".to_string(),
                ai_action: "Action 44".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_45".to_string(),
                question: "Question 45 for Fatima?".to_string(),
                ai_action: "Action 45".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_46".to_string(),
                question: "Question 46 for Fatima?".to_string(),
                ai_action: "Action 46".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_47".to_string(),
                question: "Question 47 for Fatima?".to_string(),
                ai_action: "Action 47".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_48".to_string(),
                question: "Question 48 for Fatima?".to_string(),
                ai_action: "Action 48".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_49".to_string(),
                question: "Question 49 for Fatima?".to_string(),
                ai_action: "Action 49".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_50".to_string(),
                question: "Question 50 for Fatima?".to_string(),
                ai_action: "Action 50".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_51".to_string(),
                question: "Question 51 for Fatima?".to_string(),
                ai_action: "Action 51".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_52".to_string(),
                question: "Question 52 for Fatima?".to_string(),
                ai_action: "Action 52".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_53".to_string(),
                question: "Question 53 for Fatima?".to_string(),
                ai_action: "Action 53".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_54".to_string(),
                question: "Question 54 for Fatima?".to_string(),
                ai_action: "Action 54".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_55".to_string(),
                question: "Question 55 for Fatima?".to_string(),
                ai_action: "Action 55".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_56".to_string(),
                question: "Question 56 for Fatima?".to_string(),
                ai_action: "Action 56".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_57".to_string(),
                question: "Question 57 for Fatima?".to_string(),
                ai_action: "Action 57".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_58".to_string(),
                question: "Question 58 for Fatima?".to_string(),
                ai_action: "Action 58".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_59".to_string(),
                question: "Question 59 for Fatima?".to_string(),
                ai_action: "Action 59".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_60".to_string(),
                question: "Question 60 for Fatima?".to_string(),
                ai_action: "Action 60".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_61".to_string(),
                question: "Question 61 for Fatima?".to_string(),
                ai_action: "Action 61".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_62".to_string(),
                question: "Question 62 for Fatima?".to_string(),
                ai_action: "Action 62".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_63".to_string(),
                question: "Question 63 for Fatima?".to_string(),
                ai_action: "Action 63".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_64".to_string(),
                question: "Question 64 for Fatima?".to_string(),
                ai_action: "Action 64".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_65".to_string(),
                question: "Question 65 for Fatima?".to_string(),
                ai_action: "Action 65".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_66".to_string(),
                question: "Question 66 for Fatima?".to_string(),
                ai_action: "Action 66".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_67".to_string(),
                question: "Question 67 for Fatima?".to_string(),
                ai_action: "Action 67".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_68".to_string(),
                question: "Question 68 for Fatima?".to_string(),
                ai_action: "Action 68".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_69".to_string(),
                question: "Question 69 for Fatima?".to_string(),
                ai_action: "Action 69".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_70".to_string(),
                question: "Question 70 for Fatima?".to_string(),
                ai_action: "Action 70".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_71".to_string(),
                question: "Question 71 for Fatima?".to_string(),
                ai_action: "Action 71".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_72".to_string(),
                question: "Question 72 for Fatima?".to_string(),
                ai_action: "Action 72".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_73".to_string(),
                question: "Question 73 for Fatima?".to_string(),
                ai_action: "Action 73".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_74".to_string(),
                question: "Question 74 for Fatima?".to_string(),
                ai_action: "Action 74".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_75".to_string(),
                question: "Question 75 for Fatima?".to_string(),
                ai_action: "Action 75".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_76".to_string(),
                question: "Question 76 for Fatima?".to_string(),
                ai_action: "Action 76".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_77".to_string(),
                question: "Question 77 for Fatima?".to_string(),
                ai_action: "Action 77".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_78".to_string(),
                question: "Question 78 for Fatima?".to_string(),
                ai_action: "Action 78".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_79".to_string(),
                question: "Question 79 for Fatima?".to_string(),
                ai_action: "Action 79".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_80".to_string(),
                question: "Question 80 for Fatima?".to_string(),
                ai_action: "Action 80".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_81".to_string(),
                question: "Question 81 for Fatima?".to_string(),
                ai_action: "Action 81".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_82".to_string(),
                question: "Question 82 for Fatima?".to_string(),
                ai_action: "Action 82".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_83".to_string(),
                question: "Question 83 for Fatima?".to_string(),
                ai_action: "Action 83".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_84".to_string(),
                question: "Question 84 for Fatima?".to_string(),
                ai_action: "Action 84".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_85".to_string(),
                question: "Question 85 for Fatima?".to_string(),
                ai_action: "Action 85".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_86".to_string(),
                question: "Question 86 for Fatima?".to_string(),
                ai_action: "Action 86".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_87".to_string(),
                question: "Question 87 for Fatima?".to_string(),
                ai_action: "Action 87".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_88".to_string(),
                question: "Question 88 for Fatima?".to_string(),
                ai_action: "Action 88".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_89".to_string(),
                question: "Question 89 for Fatima?".to_string(),
                ai_action: "Action 89".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_90".to_string(),
                question: "Question 90 for Fatima?".to_string(),
                ai_action: "Action 90".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_91".to_string(),
                question: "Question 91 for Fatima?".to_string(),
                ai_action: "Action 91".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_92".to_string(),
                question: "Question 92 for Fatima?".to_string(),
                ai_action: "Action 92".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_93".to_string(),
                question: "Question 93 for Fatima?".to_string(),
                ai_action: "Action 93".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_94".to_string(),
                question: "Question 94 for Fatima?".to_string(),
                ai_action: "Action 94".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_95".to_string(),
                question: "Question 95 for Fatima?".to_string(),
                ai_action: "Action 95".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_96".to_string(),
                question: "Question 96 for Fatima?".to_string(),
                ai_action: "Action 96".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_97".to_string(),
                question: "Question 97 for Fatima?".to_string(),
                ai_action: "Action 97".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_98".to_string(),
                question: "Question 98 for Fatima?".to_string(),
                ai_action: "Action 98".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_99".to_string(),
                question: "Question 99 for Fatima?".to_string(),
                ai_action: "Action 99".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_100".to_string(),
                question: "Question 100 for Fatima?".to_string(),
                ai_action: "Action 100".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_101".to_string(),
                question: "Question 101 for Fatima?".to_string(),
                ai_action: "Action 101".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_102".to_string(),
                question: "Question 102 for Fatima?".to_string(),
                ai_action: "Action 102".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_103".to_string(),
                question: "Question 103 for Fatima?".to_string(),
                ai_action: "Action 103".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_104".to_string(),
                question: "Question 104 for Fatima?".to_string(),
                ai_action: "Action 104".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_105".to_string(),
                question: "Question 105 for Fatima?".to_string(),
                ai_action: "Action 105".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_106".to_string(),
                question: "Question 106 for Fatima?".to_string(),
                ai_action: "Action 106".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_107".to_string(),
                question: "Question 107 for Fatima?".to_string(),
                ai_action: "Action 107".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_108".to_string(),
                question: "Question 108 for Fatima?".to_string(),
                ai_action: "Action 108".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_109".to_string(),
                question: "Question 109 for Fatima?".to_string(),
                ai_action: "Action 109".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_110".to_string(),
                question: "Question 110 for Fatima?".to_string(),
                ai_action: "Action 110".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_111".to_string(),
                question: "Question 111 for Fatima?".to_string(),
                ai_action: "Action 111".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_112".to_string(),
                question: "Question 112 for Fatima?".to_string(),
                ai_action: "Action 112".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_113".to_string(),
                question: "Question 113 for Fatima?".to_string(),
                ai_action: "Action 113".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_114".to_string(),
                question: "Question 114 for Fatima?".to_string(),
                ai_action: "Action 114".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_115".to_string(),
                question: "Question 115 for Fatima?".to_string(),
                ai_action: "Action 115".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_116".to_string(),
                question: "Question 116 for Fatima?".to_string(),
                ai_action: "Action 116".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_117".to_string(),
                question: "Question 117 for Fatima?".to_string(),
                ai_action: "Action 117".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_118".to_string(),
                question: "Question 118 for Fatima?".to_string(),
                ai_action: "Action 118".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_119".to_string(),
                question: "Question 119 for Fatima?".to_string(),
                ai_action: "Action 119".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_120".to_string(),
                question: "Question 120 for Fatima?".to_string(),
                ai_action: "Action 120".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_121".to_string(),
                question: "Question 121 for Fatima?".to_string(),
                ai_action: "Action 121".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_122".to_string(),
                question: "Question 122 for Fatima?".to_string(),
                ai_action: "Action 122".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_123".to_string(),
                question: "Question 123 for Fatima?".to_string(),
                ai_action: "Action 123".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_124".to_string(),
                question: "Question 124 for Fatima?".to_string(),
                ai_action: "Action 124".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_125".to_string(),
                question: "Question 125 for Fatima?".to_string(),
                ai_action: "Action 125".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_126".to_string(),
                question: "Question 126 for Fatima?".to_string(),
                ai_action: "Action 126".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_127".to_string(),
                question: "Question 127 for Fatima?".to_string(),
                ai_action: "Action 127".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_128".to_string(),
                question: "Question 128 for Fatima?".to_string(),
                ai_action: "Action 128".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_129".to_string(),
                question: "Question 129 for Fatima?".to_string(),
                ai_action: "Action 129".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_130".to_string(),
                question: "Question 130 for Fatima?".to_string(),
                ai_action: "Action 130".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_131".to_string(),
                question: "Question 131 for Fatima?".to_string(),
                ai_action: "Action 131".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_132".to_string(),
                question: "Question 132 for Fatima?".to_string(),
                ai_action: "Action 132".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_133".to_string(),
                question: "Question 133 for Fatima?".to_string(),
                ai_action: "Action 133".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_134".to_string(),
                question: "Question 134 for Fatima?".to_string(),
                ai_action: "Action 134".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_135".to_string(),
                question: "Question 135 for Fatima?".to_string(),
                ai_action: "Action 135".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_136".to_string(),
                question: "Question 136 for Fatima?".to_string(),
                ai_action: "Action 136".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_137".to_string(),
                question: "Question 137 for Fatima?".to_string(),
                ai_action: "Action 137".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_138".to_string(),
                question: "Question 138 for Fatima?".to_string(),
                ai_action: "Action 138".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_139".to_string(),
                question: "Question 139 for Fatima?".to_string(),
                ai_action: "Action 139".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_140".to_string(),
                question: "Question 140 for Fatima?".to_string(),
                ai_action: "Action 140".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_141".to_string(),
                question: "Question 141 for Fatima?".to_string(),
                ai_action: "Action 141".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_142".to_string(),
                question: "Question 142 for Fatima?".to_string(),
                ai_action: "Action 142".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_143".to_string(),
                question: "Question 143 for Fatima?".to_string(),
                ai_action: "Action 143".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_144".to_string(),
                question: "Question 144 for Fatima?".to_string(),
                ai_action: "Action 144".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_145".to_string(),
                question: "Question 145 for Fatima?".to_string(),
                ai_action: "Action 145".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_146".to_string(),
                question: "Question 146 for Fatima?".to_string(),
                ai_action: "Action 146".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_147".to_string(),
                question: "Question 147 for Fatima?".to_string(),
                ai_action: "Action 147".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_148".to_string(),
                question: "Question 148 for Fatima?".to_string(),
                ai_action: "Action 148".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

            JourneyStep {
                id: "step_149".to_string(),
                question: "Question 149 for Fatima?".to_string(),
                ai_action: "Action 149".to_string(),
                required_modules: vec!["module_A".to_string(), "module_B".to_string()],
            },

        ],
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
        for j in journeys {
            assert!(j.steps.len() > 100);
        }
    }
}
