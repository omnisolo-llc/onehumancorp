use axum::{Json, Router, routing::get};
use std::sync::OnceLock;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct HelpArticle {
    pub id: &'static str,
    pub title: &'static str,
    pub content: &'static str,
    pub category: &'static str,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Tooltip {
    pub element_id: &'static str,
    pub text: &'static str,
}

static HELP_ARTICLES: OnceLock<Vec<HelpArticle>> = OnceLock::new();
static TOOLTIPS: OnceLock<Vec<Tooltip>> = OnceLock::new();

pub fn init_docs() {
    HELP_ARTICLES.get_or_init(|| {
        vec![
            HelpArticle {
                id: "article_1",
                title: "How to manage feature 1 for your small business",
                content: "Managing feature 1 is simple. Just go to your dashboard, click on the '1' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_2",
                title: "How to manage feature 2 for your small business",
                content: "Managing feature 2 is simple. Just go to your dashboard, click on the '2' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_3",
                title: "How to manage feature 3 for your small business",
                content: "Managing feature 3 is simple. Just go to your dashboard, click on the '3' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_4",
                title: "How to manage feature 4 for your small business",
                content: "Managing feature 4 is simple. Just go to your dashboard, click on the '4' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_5",
                title: "How to manage feature 5 for your small business",
                content: "Managing feature 5 is simple. Just go to your dashboard, click on the '5' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_6",
                title: "How to manage feature 6 for your small business",
                content: "Managing feature 6 is simple. Just go to your dashboard, click on the '6' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_7",
                title: "How to manage feature 7 for your small business",
                content: "Managing feature 7 is simple. Just go to your dashboard, click on the '7' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_8",
                title: "How to manage feature 8 for your small business",
                content: "Managing feature 8 is simple. Just go to your dashboard, click on the '8' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_9",
                title: "How to manage feature 9 for your small business",
                content: "Managing feature 9 is simple. Just go to your dashboard, click on the '9' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_10",
                title: "How to manage feature 10 for your small business",
                content: "Managing feature 10 is simple. Just go to your dashboard, click on the '10' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_11",
                title: "How to manage feature 11 for your small business",
                content: "Managing feature 11 is simple. Just go to your dashboard, click on the '11' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_12",
                title: "How to manage feature 12 for your small business",
                content: "Managing feature 12 is simple. Just go to your dashboard, click on the '12' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_13",
                title: "How to manage feature 13 for your small business",
                content: "Managing feature 13 is simple. Just go to your dashboard, click on the '13' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_14",
                title: "How to manage feature 14 for your small business",
                content: "Managing feature 14 is simple. Just go to your dashboard, click on the '14' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_15",
                title: "How to manage feature 15 for your small business",
                content: "Managing feature 15 is simple. Just go to your dashboard, click on the '15' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_16",
                title: "How to manage feature 16 for your small business",
                content: "Managing feature 16 is simple. Just go to your dashboard, click on the '16' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_17",
                title: "How to manage feature 17 for your small business",
                content: "Managing feature 17 is simple. Just go to your dashboard, click on the '17' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_18",
                title: "How to manage feature 18 for your small business",
                content: "Managing feature 18 is simple. Just go to your dashboard, click on the '18' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_19",
                title: "How to manage feature 19 for your small business",
                content: "Managing feature 19 is simple. Just go to your dashboard, click on the '19' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_20",
                title: "How to manage feature 20 for your small business",
                content: "Managing feature 20 is simple. Just go to your dashboard, click on the '20' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_21",
                title: "How to manage feature 21 for your small business",
                content: "Managing feature 21 is simple. Just go to your dashboard, click on the '21' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_22",
                title: "How to manage feature 22 for your small business",
                content: "Managing feature 22 is simple. Just go to your dashboard, click on the '22' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_23",
                title: "How to manage feature 23 for your small business",
                content: "Managing feature 23 is simple. Just go to your dashboard, click on the '23' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_24",
                title: "How to manage feature 24 for your small business",
                content: "Managing feature 24 is simple. Just go to your dashboard, click on the '24' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_25",
                title: "How to manage feature 25 for your small business",
                content: "Managing feature 25 is simple. Just go to your dashboard, click on the '25' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_26",
                title: "How to manage feature 26 for your small business",
                content: "Managing feature 26 is simple. Just go to your dashboard, click on the '26' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_27",
                title: "How to manage feature 27 for your small business",
                content: "Managing feature 27 is simple. Just go to your dashboard, click on the '27' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_28",
                title: "How to manage feature 28 for your small business",
                content: "Managing feature 28 is simple. Just go to your dashboard, click on the '28' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_29",
                title: "How to manage feature 29 for your small business",
                content: "Managing feature 29 is simple. Just go to your dashboard, click on the '29' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_30",
                title: "How to manage feature 30 for your small business",
                content: "Managing feature 30 is simple. Just go to your dashboard, click on the '30' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_31",
                title: "How to manage feature 31 for your small business",
                content: "Managing feature 31 is simple. Just go to your dashboard, click on the '31' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_32",
                title: "How to manage feature 32 for your small business",
                content: "Managing feature 32 is simple. Just go to your dashboard, click on the '32' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_33",
                title: "How to manage feature 33 for your small business",
                content: "Managing feature 33 is simple. Just go to your dashboard, click on the '33' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_34",
                title: "How to manage feature 34 for your small business",
                content: "Managing feature 34 is simple. Just go to your dashboard, click on the '34' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_35",
                title: "How to manage feature 35 for your small business",
                content: "Managing feature 35 is simple. Just go to your dashboard, click on the '35' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_36",
                title: "How to manage feature 36 for your small business",
                content: "Managing feature 36 is simple. Just go to your dashboard, click on the '36' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_37",
                title: "How to manage feature 37 for your small business",
                content: "Managing feature 37 is simple. Just go to your dashboard, click on the '37' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_38",
                title: "How to manage feature 38 for your small business",
                content: "Managing feature 38 is simple. Just go to your dashboard, click on the '38' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_39",
                title: "How to manage feature 39 for your small business",
                content: "Managing feature 39 is simple. Just go to your dashboard, click on the '39' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_40",
                title: "How to manage feature 40 for your small business",
                content: "Managing feature 40 is simple. Just go to your dashboard, click on the '40' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_41",
                title: "How to manage feature 41 for your small business",
                content: "Managing feature 41 is simple. Just go to your dashboard, click on the '41' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_42",
                title: "How to manage feature 42 for your small business",
                content: "Managing feature 42 is simple. Just go to your dashboard, click on the '42' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_43",
                title: "How to manage feature 43 for your small business",
                content: "Managing feature 43 is simple. Just go to your dashboard, click on the '43' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_44",
                title: "How to manage feature 44 for your small business",
                content: "Managing feature 44 is simple. Just go to your dashboard, click on the '44' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_45",
                title: "How to manage feature 45 for your small business",
                content: "Managing feature 45 is simple. Just go to your dashboard, click on the '45' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_46",
                title: "How to manage feature 46 for your small business",
                content: "Managing feature 46 is simple. Just go to your dashboard, click on the '46' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_47",
                title: "How to manage feature 47 for your small business",
                content: "Managing feature 47 is simple. Just go to your dashboard, click on the '47' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_48",
                title: "How to manage feature 48 for your small business",
                content: "Managing feature 48 is simple. Just go to your dashboard, click on the '48' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_49",
                title: "How to manage feature 49 for your small business",
                content: "Managing feature 49 is simple. Just go to your dashboard, click on the '49' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_50",
                title: "How to manage feature 50 for your small business",
                content: "Managing feature 50 is simple. Just go to your dashboard, click on the '50' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_51",
                title: "How to manage feature 51 for your small business",
                content: "Managing feature 51 is simple. Just go to your dashboard, click on the '51' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_52",
                title: "How to manage feature 52 for your small business",
                content: "Managing feature 52 is simple. Just go to your dashboard, click on the '52' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_53",
                title: "How to manage feature 53 for your small business",
                content: "Managing feature 53 is simple. Just go to your dashboard, click on the '53' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_54",
                title: "How to manage feature 54 for your small business",
                content: "Managing feature 54 is simple. Just go to your dashboard, click on the '54' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_55",
                title: "How to manage feature 55 for your small business",
                content: "Managing feature 55 is simple. Just go to your dashboard, click on the '55' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_56",
                title: "How to manage feature 56 for your small business",
                content: "Managing feature 56 is simple. Just go to your dashboard, click on the '56' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_57",
                title: "How to manage feature 57 for your small business",
                content: "Managing feature 57 is simple. Just go to your dashboard, click on the '57' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_58",
                title: "How to manage feature 58 for your small business",
                content: "Managing feature 58 is simple. Just go to your dashboard, click on the '58' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_59",
                title: "How to manage feature 59 for your small business",
                content: "Managing feature 59 is simple. Just go to your dashboard, click on the '59' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_60",
                title: "How to manage feature 60 for your small business",
                content: "Managing feature 60 is simple. Just go to your dashboard, click on the '60' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_61",
                title: "How to manage feature 61 for your small business",
                content: "Managing feature 61 is simple. Just go to your dashboard, click on the '61' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_62",
                title: "How to manage feature 62 for your small business",
                content: "Managing feature 62 is simple. Just go to your dashboard, click on the '62' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_63",
                title: "How to manage feature 63 for your small business",
                content: "Managing feature 63 is simple. Just go to your dashboard, click on the '63' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_64",
                title: "How to manage feature 64 for your small business",
                content: "Managing feature 64 is simple. Just go to your dashboard, click on the '64' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_65",
                title: "How to manage feature 65 for your small business",
                content: "Managing feature 65 is simple. Just go to your dashboard, click on the '65' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_66",
                title: "How to manage feature 66 for your small business",
                content: "Managing feature 66 is simple. Just go to your dashboard, click on the '66' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_67",
                title: "How to manage feature 67 for your small business",
                content: "Managing feature 67 is simple. Just go to your dashboard, click on the '67' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_68",
                title: "How to manage feature 68 for your small business",
                content: "Managing feature 68 is simple. Just go to your dashboard, click on the '68' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_69",
                title: "How to manage feature 69 for your small business",
                content: "Managing feature 69 is simple. Just go to your dashboard, click on the '69' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_70",
                title: "How to manage feature 70 for your small business",
                content: "Managing feature 70 is simple. Just go to your dashboard, click on the '70' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_71",
                title: "How to manage feature 71 for your small business",
                content: "Managing feature 71 is simple. Just go to your dashboard, click on the '71' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_72",
                title: "How to manage feature 72 for your small business",
                content: "Managing feature 72 is simple. Just go to your dashboard, click on the '72' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_73",
                title: "How to manage feature 73 for your small business",
                content: "Managing feature 73 is simple. Just go to your dashboard, click on the '73' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_74",
                title: "How to manage feature 74 for your small business",
                content: "Managing feature 74 is simple. Just go to your dashboard, click on the '74' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_75",
                title: "How to manage feature 75 for your small business",
                content: "Managing feature 75 is simple. Just go to your dashboard, click on the '75' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_76",
                title: "How to manage feature 76 for your small business",
                content: "Managing feature 76 is simple. Just go to your dashboard, click on the '76' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_77",
                title: "How to manage feature 77 for your small business",
                content: "Managing feature 77 is simple. Just go to your dashboard, click on the '77' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_78",
                title: "How to manage feature 78 for your small business",
                content: "Managing feature 78 is simple. Just go to your dashboard, click on the '78' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_79",
                title: "How to manage feature 79 for your small business",
                content: "Managing feature 79 is simple. Just go to your dashboard, click on the '79' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_80",
                title: "How to manage feature 80 for your small business",
                content: "Managing feature 80 is simple. Just go to your dashboard, click on the '80' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_81",
                title: "How to manage feature 81 for your small business",
                content: "Managing feature 81 is simple. Just go to your dashboard, click on the '81' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_82",
                title: "How to manage feature 82 for your small business",
                content: "Managing feature 82 is simple. Just go to your dashboard, click on the '82' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_83",
                title: "How to manage feature 83 for your small business",
                content: "Managing feature 83 is simple. Just go to your dashboard, click on the '83' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_84",
                title: "How to manage feature 84 for your small business",
                content: "Managing feature 84 is simple. Just go to your dashboard, click on the '84' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_85",
                title: "How to manage feature 85 for your small business",
                content: "Managing feature 85 is simple. Just go to your dashboard, click on the '85' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_86",
                title: "How to manage feature 86 for your small business",
                content: "Managing feature 86 is simple. Just go to your dashboard, click on the '86' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_87",
                title: "How to manage feature 87 for your small business",
                content: "Managing feature 87 is simple. Just go to your dashboard, click on the '87' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_88",
                title: "How to manage feature 88 for your small business",
                content: "Managing feature 88 is simple. Just go to your dashboard, click on the '88' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_89",
                title: "How to manage feature 89 for your small business",
                content: "Managing feature 89 is simple. Just go to your dashboard, click on the '89' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_90",
                title: "How to manage feature 90 for your small business",
                content: "Managing feature 90 is simple. Just go to your dashboard, click on the '90' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_91",
                title: "How to manage feature 91 for your small business",
                content: "Managing feature 91 is simple. Just go to your dashboard, click on the '91' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_92",
                title: "How to manage feature 92 for your small business",
                content: "Managing feature 92 is simple. Just go to your dashboard, click on the '92' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_93",
                title: "How to manage feature 93 for your small business",
                content: "Managing feature 93 is simple. Just go to your dashboard, click on the '93' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_94",
                title: "How to manage feature 94 for your small business",
                content: "Managing feature 94 is simple. Just go to your dashboard, click on the '94' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_95",
                title: "How to manage feature 95 for your small business",
                content: "Managing feature 95 is simple. Just go to your dashboard, click on the '95' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_96",
                title: "How to manage feature 96 for your small business",
                content: "Managing feature 96 is simple. Just go to your dashboard, click on the '96' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_97",
                title: "How to manage feature 97 for your small business",
                content: "Managing feature 97 is simple. Just go to your dashboard, click on the '97' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_98",
                title: "How to manage feature 98 for your small business",
                content: "Managing feature 98 is simple. Just go to your dashboard, click on the '98' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_99",
                title: "How to manage feature 99 for your small business",
                content: "Managing feature 99 is simple. Just go to your dashboard, click on the '99' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_100",
                title: "How to manage feature 100 for your small business",
                content: "Managing feature 100 is simple. Just go to your dashboard, click on the '100' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_101",
                title: "How to manage feature 101 for your small business",
                content: "Managing feature 101 is simple. Just go to your dashboard, click on the '101' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_102",
                title: "How to manage feature 102 for your small business",
                content: "Managing feature 102 is simple. Just go to your dashboard, click on the '102' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_103",
                title: "How to manage feature 103 for your small business",
                content: "Managing feature 103 is simple. Just go to your dashboard, click on the '103' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_104",
                title: "How to manage feature 104 for your small business",
                content: "Managing feature 104 is simple. Just go to your dashboard, click on the '104' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_105",
                title: "How to manage feature 105 for your small business",
                content: "Managing feature 105 is simple. Just go to your dashboard, click on the '105' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_106",
                title: "How to manage feature 106 for your small business",
                content: "Managing feature 106 is simple. Just go to your dashboard, click on the '106' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_107",
                title: "How to manage feature 107 for your small business",
                content: "Managing feature 107 is simple. Just go to your dashboard, click on the '107' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_108",
                title: "How to manage feature 108 for your small business",
                content: "Managing feature 108 is simple. Just go to your dashboard, click on the '108' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_109",
                title: "How to manage feature 109 for your small business",
                content: "Managing feature 109 is simple. Just go to your dashboard, click on the '109' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_110",
                title: "How to manage feature 110 for your small business",
                content: "Managing feature 110 is simple. Just go to your dashboard, click on the '110' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_111",
                title: "How to manage feature 111 for your small business",
                content: "Managing feature 111 is simple. Just go to your dashboard, click on the '111' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_112",
                title: "How to manage feature 112 for your small business",
                content: "Managing feature 112 is simple. Just go to your dashboard, click on the '112' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_113",
                title: "How to manage feature 113 for your small business",
                content: "Managing feature 113 is simple. Just go to your dashboard, click on the '113' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_114",
                title: "How to manage feature 114 for your small business",
                content: "Managing feature 114 is simple. Just go to your dashboard, click on the '114' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_115",
                title: "How to manage feature 115 for your small business",
                content: "Managing feature 115 is simple. Just go to your dashboard, click on the '115' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_116",
                title: "How to manage feature 116 for your small business",
                content: "Managing feature 116 is simple. Just go to your dashboard, click on the '116' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_117",
                title: "How to manage feature 117 for your small business",
                content: "Managing feature 117 is simple. Just go to your dashboard, click on the '117' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_118",
                title: "How to manage feature 118 for your small business",
                content: "Managing feature 118 is simple. Just go to your dashboard, click on the '118' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_119",
                title: "How to manage feature 119 for your small business",
                content: "Managing feature 119 is simple. Just go to your dashboard, click on the '119' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_120",
                title: "How to manage feature 120 for your small business",
                content: "Managing feature 120 is simple. Just go to your dashboard, click on the '120' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_121",
                title: "How to manage feature 121 for your small business",
                content: "Managing feature 121 is simple. Just go to your dashboard, click on the '121' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_122",
                title: "How to manage feature 122 for your small business",
                content: "Managing feature 122 is simple. Just go to your dashboard, click on the '122' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_123",
                title: "How to manage feature 123 for your small business",
                content: "Managing feature 123 is simple. Just go to your dashboard, click on the '123' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_124",
                title: "How to manage feature 124 for your small business",
                content: "Managing feature 124 is simple. Just go to your dashboard, click on the '124' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_125",
                title: "How to manage feature 125 for your small business",
                content: "Managing feature 125 is simple. Just go to your dashboard, click on the '125' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_126",
                title: "How to manage feature 126 for your small business",
                content: "Managing feature 126 is simple. Just go to your dashboard, click on the '126' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_127",
                title: "How to manage feature 127 for your small business",
                content: "Managing feature 127 is simple. Just go to your dashboard, click on the '127' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_128",
                title: "How to manage feature 128 for your small business",
                content: "Managing feature 128 is simple. Just go to your dashboard, click on the '128' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_129",
                title: "How to manage feature 129 for your small business",
                content: "Managing feature 129 is simple. Just go to your dashboard, click on the '129' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_130",
                title: "How to manage feature 130 for your small business",
                content: "Managing feature 130 is simple. Just go to your dashboard, click on the '130' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_131",
                title: "How to manage feature 131 for your small business",
                content: "Managing feature 131 is simple. Just go to your dashboard, click on the '131' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_132",
                title: "How to manage feature 132 for your small business",
                content: "Managing feature 132 is simple. Just go to your dashboard, click on the '132' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_133",
                title: "How to manage feature 133 for your small business",
                content: "Managing feature 133 is simple. Just go to your dashboard, click on the '133' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_134",
                title: "How to manage feature 134 for your small business",
                content: "Managing feature 134 is simple. Just go to your dashboard, click on the '134' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_135",
                title: "How to manage feature 135 for your small business",
                content: "Managing feature 135 is simple. Just go to your dashboard, click on the '135' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_136",
                title: "How to manage feature 136 for your small business",
                content: "Managing feature 136 is simple. Just go to your dashboard, click on the '136' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_137",
                title: "How to manage feature 137 for your small business",
                content: "Managing feature 137 is simple. Just go to your dashboard, click on the '137' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_138",
                title: "How to manage feature 138 for your small business",
                content: "Managing feature 138 is simple. Just go to your dashboard, click on the '138' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_139",
                title: "How to manage feature 139 for your small business",
                content: "Managing feature 139 is simple. Just go to your dashboard, click on the '139' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_140",
                title: "How to manage feature 140 for your small business",
                content: "Managing feature 140 is simple. Just go to your dashboard, click on the '140' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_141",
                title: "How to manage feature 141 for your small business",
                content: "Managing feature 141 is simple. Just go to your dashboard, click on the '141' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_142",
                title: "How to manage feature 142 for your small business",
                content: "Managing feature 142 is simple. Just go to your dashboard, click on the '142' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_143",
                title: "How to manage feature 143 for your small business",
                content: "Managing feature 143 is simple. Just go to your dashboard, click on the '143' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_144",
                title: "How to manage feature 144 for your small business",
                content: "Managing feature 144 is simple. Just go to your dashboard, click on the '144' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_145",
                title: "How to manage feature 145 for your small business",
                content: "Managing feature 145 is simple. Just go to your dashboard, click on the '145' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_146",
                title: "How to manage feature 146 for your small business",
                content: "Managing feature 146 is simple. Just go to your dashboard, click on the '146' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_147",
                title: "How to manage feature 147 for your small business",
                content: "Managing feature 147 is simple. Just go to your dashboard, click on the '147' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_148",
                title: "How to manage feature 148 for your small business",
                content: "Managing feature 148 is simple. Just go to your dashboard, click on the '148' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
            HelpArticle {
                id: "article_149",
                title: "How to manage feature 149 for your small business",
                content: "Managing feature 149 is simple. Just go to your dashboard, click on the '149' tab, and follow the plain language instructions. This will help your business grow and keep everything organized.",
                category: "Getting Started",
            },
        ]
    });

    TOOLTIPS.get_or_init(|| {
        vec![
            Tooltip {
                element_id: "tooltip_1",
                text: "Click here to adjust settings for 1. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_2",
                text: "Click here to adjust settings for 2. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_3",
                text: "Click here to adjust settings for 3. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_4",
                text: "Click here to adjust settings for 4. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_5",
                text: "Click here to adjust settings for 5. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_6",
                text: "Click here to adjust settings for 6. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_7",
                text: "Click here to adjust settings for 7. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_8",
                text: "Click here to adjust settings for 8. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_9",
                text: "Click here to adjust settings for 9. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_10",
                text: "Click here to adjust settings for 10. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_11",
                text: "Click here to adjust settings for 11. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_12",
                text: "Click here to adjust settings for 12. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_13",
                text: "Click here to adjust settings for 13. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_14",
                text: "Click here to adjust settings for 14. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_15",
                text: "Click here to adjust settings for 15. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_16",
                text: "Click here to adjust settings for 16. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_17",
                text: "Click here to adjust settings for 17. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_18",
                text: "Click here to adjust settings for 18. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_19",
                text: "Click here to adjust settings for 19. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_20",
                text: "Click here to adjust settings for 20. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_21",
                text: "Click here to adjust settings for 21. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_22",
                text: "Click here to adjust settings for 22. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_23",
                text: "Click here to adjust settings for 23. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_24",
                text: "Click here to adjust settings for 24. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_25",
                text: "Click here to adjust settings for 25. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_26",
                text: "Click here to adjust settings for 26. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_27",
                text: "Click here to adjust settings for 27. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_28",
                text: "Click here to adjust settings for 28. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_29",
                text: "Click here to adjust settings for 29. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_30",
                text: "Click here to adjust settings for 30. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_31",
                text: "Click here to adjust settings for 31. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_32",
                text: "Click here to adjust settings for 32. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_33",
                text: "Click here to adjust settings for 33. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_34",
                text: "Click here to adjust settings for 34. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_35",
                text: "Click here to adjust settings for 35. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_36",
                text: "Click here to adjust settings for 36. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_37",
                text: "Click here to adjust settings for 37. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_38",
                text: "Click here to adjust settings for 38. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_39",
                text: "Click here to adjust settings for 39. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_40",
                text: "Click here to adjust settings for 40. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_41",
                text: "Click here to adjust settings for 41. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_42",
                text: "Click here to adjust settings for 42. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_43",
                text: "Click here to adjust settings for 43. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_44",
                text: "Click here to adjust settings for 44. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_45",
                text: "Click here to adjust settings for 45. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_46",
                text: "Click here to adjust settings for 46. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_47",
                text: "Click here to adjust settings for 47. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_48",
                text: "Click here to adjust settings for 48. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_49",
                text: "Click here to adjust settings for 49. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_50",
                text: "Click here to adjust settings for 50. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_51",
                text: "Click here to adjust settings for 51. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_52",
                text: "Click here to adjust settings for 52. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_53",
                text: "Click here to adjust settings for 53. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_54",
                text: "Click here to adjust settings for 54. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_55",
                text: "Click here to adjust settings for 55. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_56",
                text: "Click here to adjust settings for 56. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_57",
                text: "Click here to adjust settings for 57. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_58",
                text: "Click here to adjust settings for 58. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_59",
                text: "Click here to adjust settings for 59. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_60",
                text: "Click here to adjust settings for 60. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_61",
                text: "Click here to adjust settings for 61. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_62",
                text: "Click here to adjust settings for 62. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_63",
                text: "Click here to adjust settings for 63. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_64",
                text: "Click here to adjust settings for 64. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_65",
                text: "Click here to adjust settings for 65. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_66",
                text: "Click here to adjust settings for 66. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_67",
                text: "Click here to adjust settings for 67. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_68",
                text: "Click here to adjust settings for 68. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_69",
                text: "Click here to adjust settings for 69. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_70",
                text: "Click here to adjust settings for 70. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_71",
                text: "Click here to adjust settings for 71. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_72",
                text: "Click here to adjust settings for 72. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_73",
                text: "Click here to adjust settings for 73. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_74",
                text: "Click here to adjust settings for 74. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_75",
                text: "Click here to adjust settings for 75. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_76",
                text: "Click here to adjust settings for 76. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_77",
                text: "Click here to adjust settings for 77. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_78",
                text: "Click here to adjust settings for 78. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_79",
                text: "Click here to adjust settings for 79. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_80",
                text: "Click here to adjust settings for 80. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_81",
                text: "Click here to adjust settings for 81. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_82",
                text: "Click here to adjust settings for 82. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_83",
                text: "Click here to adjust settings for 83. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_84",
                text: "Click here to adjust settings for 84. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_85",
                text: "Click here to adjust settings for 85. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_86",
                text: "Click here to adjust settings for 86. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_87",
                text: "Click here to adjust settings for 87. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_88",
                text: "Click here to adjust settings for 88. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_89",
                text: "Click here to adjust settings for 89. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_90",
                text: "Click here to adjust settings for 90. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_91",
                text: "Click here to adjust settings for 91. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_92",
                text: "Click here to adjust settings for 92. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_93",
                text: "Click here to adjust settings for 93. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_94",
                text: "Click here to adjust settings for 94. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_95",
                text: "Click here to adjust settings for 95. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_96",
                text: "Click here to adjust settings for 96. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_97",
                text: "Click here to adjust settings for 97. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_98",
                text: "Click here to adjust settings for 98. It's easy!",
            },
            Tooltip {
                element_id: "tooltip_99",
                text: "Click here to adjust settings for 99. It's easy!",
            },
        ]
    });
}

pub async fn get_help_articles() -> Json<Vec<HelpArticle>> {
    let articles = HELP_ARTICLES.get().unwrap_or(&vec![]).clone();
    Json(articles)
}

pub async fn get_tooltips() -> Json<Vec<Tooltip>> {
    let tooltips = TOOLTIPS.get().unwrap_or(&vec![]).clone();
    Json(tooltips)
}

pub fn router<S: Clone + Send + Sync + 'static>() -> Router<S> {
    init_docs();
    Router::new()
        .route("/articles", get(get_help_articles))
        .route("/tooltips", get(get_tooltips))
}
