
use std::collections::HashMap;

pub struct HelpArticle {
    pub id: String,
    pub title: String,
    pub content: String,
    pub category: String,
    pub tags: Vec<String>,
}

pub struct Tooltip {
    pub id: String,
    pub text: String,
    pub target_element: String,
}

pub struct Guide {
    pub id: String,
    pub title: String,
    pub steps: Vec<GuideStep>,
}

pub struct GuideStep {
    pub id: String,
    pub title: String,
    pub content: String,
    pub target_element: String,
}

pub fn populate_registry(registry: &mut super::registry::HelpRegistry) {

    registry.articles.insert(
        "article_1".to_string(),
        HelpArticle {
            id: "article_1".to_string(),
            title: "How to use feature 1".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 1. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_1".to_string(),
        Tooltip {
            id: "tooltip_1".to_string(),
            text: "Click here to activate feature 1. It is very simple.".to_string(),
            target_element: "button_1".to_string(),
        }
    );

    registry.articles.insert(
        "article_2".to_string(),
        HelpArticle {
            id: "article_2".to_string(),
            title: "How to use feature 2".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 2. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_2".to_string(),
        Tooltip {
            id: "tooltip_2".to_string(),
            text: "Click here to activate feature 2. It is very simple.".to_string(),
            target_element: "button_2".to_string(),
        }
    );

    registry.articles.insert(
        "article_3".to_string(),
        HelpArticle {
            id: "article_3".to_string(),
            title: "How to use feature 3".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 3. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_3".to_string(),
        Tooltip {
            id: "tooltip_3".to_string(),
            text: "Click here to activate feature 3. It is very simple.".to_string(),
            target_element: "button_3".to_string(),
        }
    );

    registry.articles.insert(
        "article_4".to_string(),
        HelpArticle {
            id: "article_4".to_string(),
            title: "How to use feature 4".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 4. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_4".to_string(),
        Tooltip {
            id: "tooltip_4".to_string(),
            text: "Click here to activate feature 4. It is very simple.".to_string(),
            target_element: "button_4".to_string(),
        }
    );

    registry.articles.insert(
        "article_5".to_string(),
        HelpArticle {
            id: "article_5".to_string(),
            title: "How to use feature 5".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 5. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_5".to_string(),
        Tooltip {
            id: "tooltip_5".to_string(),
            text: "Click here to activate feature 5. It is very simple.".to_string(),
            target_element: "button_5".to_string(),
        }
    );

    registry.articles.insert(
        "article_6".to_string(),
        HelpArticle {
            id: "article_6".to_string(),
            title: "How to use feature 6".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 6. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_6".to_string(),
        Tooltip {
            id: "tooltip_6".to_string(),
            text: "Click here to activate feature 6. It is very simple.".to_string(),
            target_element: "button_6".to_string(),
        }
    );

    registry.articles.insert(
        "article_7".to_string(),
        HelpArticle {
            id: "article_7".to_string(),
            title: "How to use feature 7".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 7. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_7".to_string(),
        Tooltip {
            id: "tooltip_7".to_string(),
            text: "Click here to activate feature 7. It is very simple.".to_string(),
            target_element: "button_7".to_string(),
        }
    );

    registry.articles.insert(
        "article_8".to_string(),
        HelpArticle {
            id: "article_8".to_string(),
            title: "How to use feature 8".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 8. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_8".to_string(),
        Tooltip {
            id: "tooltip_8".to_string(),
            text: "Click here to activate feature 8. It is very simple.".to_string(),
            target_element: "button_8".to_string(),
        }
    );

    registry.articles.insert(
        "article_9".to_string(),
        HelpArticle {
            id: "article_9".to_string(),
            title: "How to use feature 9".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 9. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_9".to_string(),
        Tooltip {
            id: "tooltip_9".to_string(),
            text: "Click here to activate feature 9. It is very simple.".to_string(),
            target_element: "button_9".to_string(),
        }
    );

    registry.articles.insert(
        "article_10".to_string(),
        HelpArticle {
            id: "article_10".to_string(),
            title: "How to use feature 10".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 10. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_10".to_string(),
        Tooltip {
            id: "tooltip_10".to_string(),
            text: "Click here to activate feature 10. It is very simple.".to_string(),
            target_element: "button_10".to_string(),
        }
    );

    registry.articles.insert(
        "article_11".to_string(),
        HelpArticle {
            id: "article_11".to_string(),
            title: "How to use feature 11".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 11. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_11".to_string(),
        Tooltip {
            id: "tooltip_11".to_string(),
            text: "Click here to activate feature 11. It is very simple.".to_string(),
            target_element: "button_11".to_string(),
        }
    );

    registry.articles.insert(
        "article_12".to_string(),
        HelpArticle {
            id: "article_12".to_string(),
            title: "How to use feature 12".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 12. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_12".to_string(),
        Tooltip {
            id: "tooltip_12".to_string(),
            text: "Click here to activate feature 12. It is very simple.".to_string(),
            target_element: "button_12".to_string(),
        }
    );

    registry.articles.insert(
        "article_13".to_string(),
        HelpArticle {
            id: "article_13".to_string(),
            title: "How to use feature 13".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 13. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_13".to_string(),
        Tooltip {
            id: "tooltip_13".to_string(),
            text: "Click here to activate feature 13. It is very simple.".to_string(),
            target_element: "button_13".to_string(),
        }
    );

    registry.articles.insert(
        "article_14".to_string(),
        HelpArticle {
            id: "article_14".to_string(),
            title: "How to use feature 14".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 14. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_14".to_string(),
        Tooltip {
            id: "tooltip_14".to_string(),
            text: "Click here to activate feature 14. It is very simple.".to_string(),
            target_element: "button_14".to_string(),
        }
    );

    registry.articles.insert(
        "article_15".to_string(),
        HelpArticle {
            id: "article_15".to_string(),
            title: "How to use feature 15".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 15. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_15".to_string(),
        Tooltip {
            id: "tooltip_15".to_string(),
            text: "Click here to activate feature 15. It is very simple.".to_string(),
            target_element: "button_15".to_string(),
        }
    );

    registry.articles.insert(
        "article_16".to_string(),
        HelpArticle {
            id: "article_16".to_string(),
            title: "How to use feature 16".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 16. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_16".to_string(),
        Tooltip {
            id: "tooltip_16".to_string(),
            text: "Click here to activate feature 16. It is very simple.".to_string(),
            target_element: "button_16".to_string(),
        }
    );

    registry.articles.insert(
        "article_17".to_string(),
        HelpArticle {
            id: "article_17".to_string(),
            title: "How to use feature 17".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 17. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_17".to_string(),
        Tooltip {
            id: "tooltip_17".to_string(),
            text: "Click here to activate feature 17. It is very simple.".to_string(),
            target_element: "button_17".to_string(),
        }
    );

    registry.articles.insert(
        "article_18".to_string(),
        HelpArticle {
            id: "article_18".to_string(),
            title: "How to use feature 18".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 18. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_18".to_string(),
        Tooltip {
            id: "tooltip_18".to_string(),
            text: "Click here to activate feature 18. It is very simple.".to_string(),
            target_element: "button_18".to_string(),
        }
    );

    registry.articles.insert(
        "article_19".to_string(),
        HelpArticle {
            id: "article_19".to_string(),
            title: "How to use feature 19".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 19. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_19".to_string(),
        Tooltip {
            id: "tooltip_19".to_string(),
            text: "Click here to activate feature 19. It is very simple.".to_string(),
            target_element: "button_19".to_string(),
        }
    );

    registry.articles.insert(
        "article_20".to_string(),
        HelpArticle {
            id: "article_20".to_string(),
            title: "How to use feature 20".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 20. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_20".to_string(),
        Tooltip {
            id: "tooltip_20".to_string(),
            text: "Click here to activate feature 20. It is very simple.".to_string(),
            target_element: "button_20".to_string(),
        }
    );

    registry.articles.insert(
        "article_21".to_string(),
        HelpArticle {
            id: "article_21".to_string(),
            title: "How to use feature 21".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 21. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_21".to_string(),
        Tooltip {
            id: "tooltip_21".to_string(),
            text: "Click here to activate feature 21. It is very simple.".to_string(),
            target_element: "button_21".to_string(),
        }
    );

    registry.articles.insert(
        "article_22".to_string(),
        HelpArticle {
            id: "article_22".to_string(),
            title: "How to use feature 22".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 22. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_22".to_string(),
        Tooltip {
            id: "tooltip_22".to_string(),
            text: "Click here to activate feature 22. It is very simple.".to_string(),
            target_element: "button_22".to_string(),
        }
    );

    registry.articles.insert(
        "article_23".to_string(),
        HelpArticle {
            id: "article_23".to_string(),
            title: "How to use feature 23".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 23. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_23".to_string(),
        Tooltip {
            id: "tooltip_23".to_string(),
            text: "Click here to activate feature 23. It is very simple.".to_string(),
            target_element: "button_23".to_string(),
        }
    );

    registry.articles.insert(
        "article_24".to_string(),
        HelpArticle {
            id: "article_24".to_string(),
            title: "How to use feature 24".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 24. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_24".to_string(),
        Tooltip {
            id: "tooltip_24".to_string(),
            text: "Click here to activate feature 24. It is very simple.".to_string(),
            target_element: "button_24".to_string(),
        }
    );

    registry.articles.insert(
        "article_25".to_string(),
        HelpArticle {
            id: "article_25".to_string(),
            title: "How to use feature 25".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 25. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_25".to_string(),
        Tooltip {
            id: "tooltip_25".to_string(),
            text: "Click here to activate feature 25. It is very simple.".to_string(),
            target_element: "button_25".to_string(),
        }
    );

    registry.articles.insert(
        "article_26".to_string(),
        HelpArticle {
            id: "article_26".to_string(),
            title: "How to use feature 26".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 26. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_26".to_string(),
        Tooltip {
            id: "tooltip_26".to_string(),
            text: "Click here to activate feature 26. It is very simple.".to_string(),
            target_element: "button_26".to_string(),
        }
    );

    registry.articles.insert(
        "article_27".to_string(),
        HelpArticle {
            id: "article_27".to_string(),
            title: "How to use feature 27".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 27. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_27".to_string(),
        Tooltip {
            id: "tooltip_27".to_string(),
            text: "Click here to activate feature 27. It is very simple.".to_string(),
            target_element: "button_27".to_string(),
        }
    );

    registry.articles.insert(
        "article_28".to_string(),
        HelpArticle {
            id: "article_28".to_string(),
            title: "How to use feature 28".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 28. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_28".to_string(),
        Tooltip {
            id: "tooltip_28".to_string(),
            text: "Click here to activate feature 28. It is very simple.".to_string(),
            target_element: "button_28".to_string(),
        }
    );

    registry.articles.insert(
        "article_29".to_string(),
        HelpArticle {
            id: "article_29".to_string(),
            title: "How to use feature 29".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 29. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_29".to_string(),
        Tooltip {
            id: "tooltip_29".to_string(),
            text: "Click here to activate feature 29. It is very simple.".to_string(),
            target_element: "button_29".to_string(),
        }
    );

    registry.articles.insert(
        "article_30".to_string(),
        HelpArticle {
            id: "article_30".to_string(),
            title: "How to use feature 30".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 30. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_30".to_string(),
        Tooltip {
            id: "tooltip_30".to_string(),
            text: "Click here to activate feature 30. It is very simple.".to_string(),
            target_element: "button_30".to_string(),
        }
    );

    registry.articles.insert(
        "article_31".to_string(),
        HelpArticle {
            id: "article_31".to_string(),
            title: "How to use feature 31".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 31. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_31".to_string(),
        Tooltip {
            id: "tooltip_31".to_string(),
            text: "Click here to activate feature 31. It is very simple.".to_string(),
            target_element: "button_31".to_string(),
        }
    );

    registry.articles.insert(
        "article_32".to_string(),
        HelpArticle {
            id: "article_32".to_string(),
            title: "How to use feature 32".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 32. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_32".to_string(),
        Tooltip {
            id: "tooltip_32".to_string(),
            text: "Click here to activate feature 32. It is very simple.".to_string(),
            target_element: "button_32".to_string(),
        }
    );

    registry.articles.insert(
        "article_33".to_string(),
        HelpArticle {
            id: "article_33".to_string(),
            title: "How to use feature 33".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 33. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_33".to_string(),
        Tooltip {
            id: "tooltip_33".to_string(),
            text: "Click here to activate feature 33. It is very simple.".to_string(),
            target_element: "button_33".to_string(),
        }
    );

    registry.articles.insert(
        "article_34".to_string(),
        HelpArticle {
            id: "article_34".to_string(),
            title: "How to use feature 34".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 34. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_34".to_string(),
        Tooltip {
            id: "tooltip_34".to_string(),
            text: "Click here to activate feature 34. It is very simple.".to_string(),
            target_element: "button_34".to_string(),
        }
    );

    registry.articles.insert(
        "article_35".to_string(),
        HelpArticle {
            id: "article_35".to_string(),
            title: "How to use feature 35".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 35. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_35".to_string(),
        Tooltip {
            id: "tooltip_35".to_string(),
            text: "Click here to activate feature 35. It is very simple.".to_string(),
            target_element: "button_35".to_string(),
        }
    );

    registry.articles.insert(
        "article_36".to_string(),
        HelpArticle {
            id: "article_36".to_string(),
            title: "How to use feature 36".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 36. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_36".to_string(),
        Tooltip {
            id: "tooltip_36".to_string(),
            text: "Click here to activate feature 36. It is very simple.".to_string(),
            target_element: "button_36".to_string(),
        }
    );

    registry.articles.insert(
        "article_37".to_string(),
        HelpArticle {
            id: "article_37".to_string(),
            title: "How to use feature 37".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 37. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_37".to_string(),
        Tooltip {
            id: "tooltip_37".to_string(),
            text: "Click here to activate feature 37. It is very simple.".to_string(),
            target_element: "button_37".to_string(),
        }
    );

    registry.articles.insert(
        "article_38".to_string(),
        HelpArticle {
            id: "article_38".to_string(),
            title: "How to use feature 38".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 38. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_38".to_string(),
        Tooltip {
            id: "tooltip_38".to_string(),
            text: "Click here to activate feature 38. It is very simple.".to_string(),
            target_element: "button_38".to_string(),
        }
    );

    registry.articles.insert(
        "article_39".to_string(),
        HelpArticle {
            id: "article_39".to_string(),
            title: "How to use feature 39".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 39. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_39".to_string(),
        Tooltip {
            id: "tooltip_39".to_string(),
            text: "Click here to activate feature 39. It is very simple.".to_string(),
            target_element: "button_39".to_string(),
        }
    );

    registry.articles.insert(
        "article_40".to_string(),
        HelpArticle {
            id: "article_40".to_string(),
            title: "How to use feature 40".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 40. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_40".to_string(),
        Tooltip {
            id: "tooltip_40".to_string(),
            text: "Click here to activate feature 40. It is very simple.".to_string(),
            target_element: "button_40".to_string(),
        }
    );

    registry.articles.insert(
        "article_41".to_string(),
        HelpArticle {
            id: "article_41".to_string(),
            title: "How to use feature 41".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 41. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_41".to_string(),
        Tooltip {
            id: "tooltip_41".to_string(),
            text: "Click here to activate feature 41. It is very simple.".to_string(),
            target_element: "button_41".to_string(),
        }
    );

    registry.articles.insert(
        "article_42".to_string(),
        HelpArticle {
            id: "article_42".to_string(),
            title: "How to use feature 42".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 42. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_42".to_string(),
        Tooltip {
            id: "tooltip_42".to_string(),
            text: "Click here to activate feature 42. It is very simple.".to_string(),
            target_element: "button_42".to_string(),
        }
    );

    registry.articles.insert(
        "article_43".to_string(),
        HelpArticle {
            id: "article_43".to_string(),
            title: "How to use feature 43".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 43. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_43".to_string(),
        Tooltip {
            id: "tooltip_43".to_string(),
            text: "Click here to activate feature 43. It is very simple.".to_string(),
            target_element: "button_43".to_string(),
        }
    );

    registry.articles.insert(
        "article_44".to_string(),
        HelpArticle {
            id: "article_44".to_string(),
            title: "How to use feature 44".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 44. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_44".to_string(),
        Tooltip {
            id: "tooltip_44".to_string(),
            text: "Click here to activate feature 44. It is very simple.".to_string(),
            target_element: "button_44".to_string(),
        }
    );

    registry.articles.insert(
        "article_45".to_string(),
        HelpArticle {
            id: "article_45".to_string(),
            title: "How to use feature 45".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 45. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_45".to_string(),
        Tooltip {
            id: "tooltip_45".to_string(),
            text: "Click here to activate feature 45. It is very simple.".to_string(),
            target_element: "button_45".to_string(),
        }
    );

    registry.articles.insert(
        "article_46".to_string(),
        HelpArticle {
            id: "article_46".to_string(),
            title: "How to use feature 46".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 46. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_46".to_string(),
        Tooltip {
            id: "tooltip_46".to_string(),
            text: "Click here to activate feature 46. It is very simple.".to_string(),
            target_element: "button_46".to_string(),
        }
    );

    registry.articles.insert(
        "article_47".to_string(),
        HelpArticle {
            id: "article_47".to_string(),
            title: "How to use feature 47".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 47. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_47".to_string(),
        Tooltip {
            id: "tooltip_47".to_string(),
            text: "Click here to activate feature 47. It is very simple.".to_string(),
            target_element: "button_47".to_string(),
        }
    );

    registry.articles.insert(
        "article_48".to_string(),
        HelpArticle {
            id: "article_48".to_string(),
            title: "How to use feature 48".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 48. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_48".to_string(),
        Tooltip {
            id: "tooltip_48".to_string(),
            text: "Click here to activate feature 48. It is very simple.".to_string(),
            target_element: "button_48".to_string(),
        }
    );

    registry.articles.insert(
        "article_49".to_string(),
        HelpArticle {
            id: "article_49".to_string(),
            title: "How to use feature 49".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 49. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_49".to_string(),
        Tooltip {
            id: "tooltip_49".to_string(),
            text: "Click here to activate feature 49. It is very simple.".to_string(),
            target_element: "button_49".to_string(),
        }
    );

    registry.articles.insert(
        "article_50".to_string(),
        HelpArticle {
            id: "article_50".to_string(),
            title: "How to use feature 50".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 50. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_50".to_string(),
        Tooltip {
            id: "tooltip_50".to_string(),
            text: "Click here to activate feature 50. It is very simple.".to_string(),
            target_element: "button_50".to_string(),
        }
    );

    registry.articles.insert(
        "article_51".to_string(),
        HelpArticle {
            id: "article_51".to_string(),
            title: "How to use feature 51".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 51. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_51".to_string(),
        Tooltip {
            id: "tooltip_51".to_string(),
            text: "Click here to activate feature 51. It is very simple.".to_string(),
            target_element: "button_51".to_string(),
        }
    );

    registry.articles.insert(
        "article_52".to_string(),
        HelpArticle {
            id: "article_52".to_string(),
            title: "How to use feature 52".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 52. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_52".to_string(),
        Tooltip {
            id: "tooltip_52".to_string(),
            text: "Click here to activate feature 52. It is very simple.".to_string(),
            target_element: "button_52".to_string(),
        }
    );

    registry.articles.insert(
        "article_53".to_string(),
        HelpArticle {
            id: "article_53".to_string(),
            title: "How to use feature 53".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 53. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_53".to_string(),
        Tooltip {
            id: "tooltip_53".to_string(),
            text: "Click here to activate feature 53. It is very simple.".to_string(),
            target_element: "button_53".to_string(),
        }
    );

    registry.articles.insert(
        "article_54".to_string(),
        HelpArticle {
            id: "article_54".to_string(),
            title: "How to use feature 54".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 54. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_54".to_string(),
        Tooltip {
            id: "tooltip_54".to_string(),
            text: "Click here to activate feature 54. It is very simple.".to_string(),
            target_element: "button_54".to_string(),
        }
    );

    registry.articles.insert(
        "article_55".to_string(),
        HelpArticle {
            id: "article_55".to_string(),
            title: "How to use feature 55".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 55. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_55".to_string(),
        Tooltip {
            id: "tooltip_55".to_string(),
            text: "Click here to activate feature 55. It is very simple.".to_string(),
            target_element: "button_55".to_string(),
        }
    );

    registry.articles.insert(
        "article_56".to_string(),
        HelpArticle {
            id: "article_56".to_string(),
            title: "How to use feature 56".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 56. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_56".to_string(),
        Tooltip {
            id: "tooltip_56".to_string(),
            text: "Click here to activate feature 56. It is very simple.".to_string(),
            target_element: "button_56".to_string(),
        }
    );

    registry.articles.insert(
        "article_57".to_string(),
        HelpArticle {
            id: "article_57".to_string(),
            title: "How to use feature 57".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 57. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_57".to_string(),
        Tooltip {
            id: "tooltip_57".to_string(),
            text: "Click here to activate feature 57. It is very simple.".to_string(),
            target_element: "button_57".to_string(),
        }
    );

    registry.articles.insert(
        "article_58".to_string(),
        HelpArticle {
            id: "article_58".to_string(),
            title: "How to use feature 58".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 58. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_58".to_string(),
        Tooltip {
            id: "tooltip_58".to_string(),
            text: "Click here to activate feature 58. It is very simple.".to_string(),
            target_element: "button_58".to_string(),
        }
    );

    registry.articles.insert(
        "article_59".to_string(),
        HelpArticle {
            id: "article_59".to_string(),
            title: "How to use feature 59".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 59. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_59".to_string(),
        Tooltip {
            id: "tooltip_59".to_string(),
            text: "Click here to activate feature 59. It is very simple.".to_string(),
            target_element: "button_59".to_string(),
        }
    );

    registry.articles.insert(
        "article_60".to_string(),
        HelpArticle {
            id: "article_60".to_string(),
            title: "How to use feature 60".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 60. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_60".to_string(),
        Tooltip {
            id: "tooltip_60".to_string(),
            text: "Click here to activate feature 60. It is very simple.".to_string(),
            target_element: "button_60".to_string(),
        }
    );

    registry.articles.insert(
        "article_61".to_string(),
        HelpArticle {
            id: "article_61".to_string(),
            title: "How to use feature 61".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 61. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_61".to_string(),
        Tooltip {
            id: "tooltip_61".to_string(),
            text: "Click here to activate feature 61. It is very simple.".to_string(),
            target_element: "button_61".to_string(),
        }
    );

    registry.articles.insert(
        "article_62".to_string(),
        HelpArticle {
            id: "article_62".to_string(),
            title: "How to use feature 62".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 62. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_62".to_string(),
        Tooltip {
            id: "tooltip_62".to_string(),
            text: "Click here to activate feature 62. It is very simple.".to_string(),
            target_element: "button_62".to_string(),
        }
    );

    registry.articles.insert(
        "article_63".to_string(),
        HelpArticle {
            id: "article_63".to_string(),
            title: "How to use feature 63".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 63. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_63".to_string(),
        Tooltip {
            id: "tooltip_63".to_string(),
            text: "Click here to activate feature 63. It is very simple.".to_string(),
            target_element: "button_63".to_string(),
        }
    );

    registry.articles.insert(
        "article_64".to_string(),
        HelpArticle {
            id: "article_64".to_string(),
            title: "How to use feature 64".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 64. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_64".to_string(),
        Tooltip {
            id: "tooltip_64".to_string(),
            text: "Click here to activate feature 64. It is very simple.".to_string(),
            target_element: "button_64".to_string(),
        }
    );

    registry.articles.insert(
        "article_65".to_string(),
        HelpArticle {
            id: "article_65".to_string(),
            title: "How to use feature 65".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 65. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_65".to_string(),
        Tooltip {
            id: "tooltip_65".to_string(),
            text: "Click here to activate feature 65. It is very simple.".to_string(),
            target_element: "button_65".to_string(),
        }
    );

    registry.articles.insert(
        "article_66".to_string(),
        HelpArticle {
            id: "article_66".to_string(),
            title: "How to use feature 66".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 66. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_66".to_string(),
        Tooltip {
            id: "tooltip_66".to_string(),
            text: "Click here to activate feature 66. It is very simple.".to_string(),
            target_element: "button_66".to_string(),
        }
    );

    registry.articles.insert(
        "article_67".to_string(),
        HelpArticle {
            id: "article_67".to_string(),
            title: "How to use feature 67".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 67. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_67".to_string(),
        Tooltip {
            id: "tooltip_67".to_string(),
            text: "Click here to activate feature 67. It is very simple.".to_string(),
            target_element: "button_67".to_string(),
        }
    );

    registry.articles.insert(
        "article_68".to_string(),
        HelpArticle {
            id: "article_68".to_string(),
            title: "How to use feature 68".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 68. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_68".to_string(),
        Tooltip {
            id: "tooltip_68".to_string(),
            text: "Click here to activate feature 68. It is very simple.".to_string(),
            target_element: "button_68".to_string(),
        }
    );

    registry.articles.insert(
        "article_69".to_string(),
        HelpArticle {
            id: "article_69".to_string(),
            title: "How to use feature 69".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 69. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_69".to_string(),
        Tooltip {
            id: "tooltip_69".to_string(),
            text: "Click here to activate feature 69. It is very simple.".to_string(),
            target_element: "button_69".to_string(),
        }
    );

    registry.articles.insert(
        "article_70".to_string(),
        HelpArticle {
            id: "article_70".to_string(),
            title: "How to use feature 70".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 70. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_70".to_string(),
        Tooltip {
            id: "tooltip_70".to_string(),
            text: "Click here to activate feature 70. It is very simple.".to_string(),
            target_element: "button_70".to_string(),
        }
    );

    registry.articles.insert(
        "article_71".to_string(),
        HelpArticle {
            id: "article_71".to_string(),
            title: "How to use feature 71".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 71. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_71".to_string(),
        Tooltip {
            id: "tooltip_71".to_string(),
            text: "Click here to activate feature 71. It is very simple.".to_string(),
            target_element: "button_71".to_string(),
        }
    );

    registry.articles.insert(
        "article_72".to_string(),
        HelpArticle {
            id: "article_72".to_string(),
            title: "How to use feature 72".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 72. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_72".to_string(),
        Tooltip {
            id: "tooltip_72".to_string(),
            text: "Click here to activate feature 72. It is very simple.".to_string(),
            target_element: "button_72".to_string(),
        }
    );

    registry.articles.insert(
        "article_73".to_string(),
        HelpArticle {
            id: "article_73".to_string(),
            title: "How to use feature 73".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 73. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_73".to_string(),
        Tooltip {
            id: "tooltip_73".to_string(),
            text: "Click here to activate feature 73. It is very simple.".to_string(),
            target_element: "button_73".to_string(),
        }
    );

    registry.articles.insert(
        "article_74".to_string(),
        HelpArticle {
            id: "article_74".to_string(),
            title: "How to use feature 74".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 74. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_74".to_string(),
        Tooltip {
            id: "tooltip_74".to_string(),
            text: "Click here to activate feature 74. It is very simple.".to_string(),
            target_element: "button_74".to_string(),
        }
    );

    registry.articles.insert(
        "article_75".to_string(),
        HelpArticle {
            id: "article_75".to_string(),
            title: "How to use feature 75".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 75. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_75".to_string(),
        Tooltip {
            id: "tooltip_75".to_string(),
            text: "Click here to activate feature 75. It is very simple.".to_string(),
            target_element: "button_75".to_string(),
        }
    );

    registry.articles.insert(
        "article_76".to_string(),
        HelpArticle {
            id: "article_76".to_string(),
            title: "How to use feature 76".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 76. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_76".to_string(),
        Tooltip {
            id: "tooltip_76".to_string(),
            text: "Click here to activate feature 76. It is very simple.".to_string(),
            target_element: "button_76".to_string(),
        }
    );

    registry.articles.insert(
        "article_77".to_string(),
        HelpArticle {
            id: "article_77".to_string(),
            title: "How to use feature 77".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 77. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_77".to_string(),
        Tooltip {
            id: "tooltip_77".to_string(),
            text: "Click here to activate feature 77. It is very simple.".to_string(),
            target_element: "button_77".to_string(),
        }
    );

    registry.articles.insert(
        "article_78".to_string(),
        HelpArticle {
            id: "article_78".to_string(),
            title: "How to use feature 78".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 78. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_78".to_string(),
        Tooltip {
            id: "tooltip_78".to_string(),
            text: "Click here to activate feature 78. It is very simple.".to_string(),
            target_element: "button_78".to_string(),
        }
    );

    registry.articles.insert(
        "article_79".to_string(),
        HelpArticle {
            id: "article_79".to_string(),
            title: "How to use feature 79".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 79. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_79".to_string(),
        Tooltip {
            id: "tooltip_79".to_string(),
            text: "Click here to activate feature 79. It is very simple.".to_string(),
            target_element: "button_79".to_string(),
        }
    );

    registry.articles.insert(
        "article_80".to_string(),
        HelpArticle {
            id: "article_80".to_string(),
            title: "How to use feature 80".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 80. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_80".to_string(),
        Tooltip {
            id: "tooltip_80".to_string(),
            text: "Click here to activate feature 80. It is very simple.".to_string(),
            target_element: "button_80".to_string(),
        }
    );

    registry.articles.insert(
        "article_81".to_string(),
        HelpArticle {
            id: "article_81".to_string(),
            title: "How to use feature 81".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 81. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_81".to_string(),
        Tooltip {
            id: "tooltip_81".to_string(),
            text: "Click here to activate feature 81. It is very simple.".to_string(),
            target_element: "button_81".to_string(),
        }
    );

    registry.articles.insert(
        "article_82".to_string(),
        HelpArticle {
            id: "article_82".to_string(),
            title: "How to use feature 82".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 82. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_82".to_string(),
        Tooltip {
            id: "tooltip_82".to_string(),
            text: "Click here to activate feature 82. It is very simple.".to_string(),
            target_element: "button_82".to_string(),
        }
    );

    registry.articles.insert(
        "article_83".to_string(),
        HelpArticle {
            id: "article_83".to_string(),
            title: "How to use feature 83".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 83. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_83".to_string(),
        Tooltip {
            id: "tooltip_83".to_string(),
            text: "Click here to activate feature 83. It is very simple.".to_string(),
            target_element: "button_83".to_string(),
        }
    );

    registry.articles.insert(
        "article_84".to_string(),
        HelpArticle {
            id: "article_84".to_string(),
            title: "How to use feature 84".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 84. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_84".to_string(),
        Tooltip {
            id: "tooltip_84".to_string(),
            text: "Click here to activate feature 84. It is very simple.".to_string(),
            target_element: "button_84".to_string(),
        }
    );

    registry.articles.insert(
        "article_85".to_string(),
        HelpArticle {
            id: "article_85".to_string(),
            title: "How to use feature 85".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 85. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_85".to_string(),
        Tooltip {
            id: "tooltip_85".to_string(),
            text: "Click here to activate feature 85. It is very simple.".to_string(),
            target_element: "button_85".to_string(),
        }
    );

    registry.articles.insert(
        "article_86".to_string(),
        HelpArticle {
            id: "article_86".to_string(),
            title: "How to use feature 86".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 86. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_86".to_string(),
        Tooltip {
            id: "tooltip_86".to_string(),
            text: "Click here to activate feature 86. It is very simple.".to_string(),
            target_element: "button_86".to_string(),
        }
    );

    registry.articles.insert(
        "article_87".to_string(),
        HelpArticle {
            id: "article_87".to_string(),
            title: "How to use feature 87".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 87. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_87".to_string(),
        Tooltip {
            id: "tooltip_87".to_string(),
            text: "Click here to activate feature 87. It is very simple.".to_string(),
            target_element: "button_87".to_string(),
        }
    );

    registry.articles.insert(
        "article_88".to_string(),
        HelpArticle {
            id: "article_88".to_string(),
            title: "How to use feature 88".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 88. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_88".to_string(),
        Tooltip {
            id: "tooltip_88".to_string(),
            text: "Click here to activate feature 88. It is very simple.".to_string(),
            target_element: "button_88".to_string(),
        }
    );

    registry.articles.insert(
        "article_89".to_string(),
        HelpArticle {
            id: "article_89".to_string(),
            title: "How to use feature 89".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 89. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_89".to_string(),
        Tooltip {
            id: "tooltip_89".to_string(),
            text: "Click here to activate feature 89. It is very simple.".to_string(),
            target_element: "button_89".to_string(),
        }
    );

    registry.articles.insert(
        "article_90".to_string(),
        HelpArticle {
            id: "article_90".to_string(),
            title: "How to use feature 90".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 90. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_90".to_string(),
        Tooltip {
            id: "tooltip_90".to_string(),
            text: "Click here to activate feature 90. It is very simple.".to_string(),
            target_element: "button_90".to_string(),
        }
    );

    registry.articles.insert(
        "article_91".to_string(),
        HelpArticle {
            id: "article_91".to_string(),
            title: "How to use feature 91".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 91. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_91".to_string(),
        Tooltip {
            id: "tooltip_91".to_string(),
            text: "Click here to activate feature 91. It is very simple.".to_string(),
            target_element: "button_91".to_string(),
        }
    );

    registry.articles.insert(
        "article_92".to_string(),
        HelpArticle {
            id: "article_92".to_string(),
            title: "How to use feature 92".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 92. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_92".to_string(),
        Tooltip {
            id: "tooltip_92".to_string(),
            text: "Click here to activate feature 92. It is very simple.".to_string(),
            target_element: "button_92".to_string(),
        }
    );

    registry.articles.insert(
        "article_93".to_string(),
        HelpArticle {
            id: "article_93".to_string(),
            title: "How to use feature 93".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 93. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_93".to_string(),
        Tooltip {
            id: "tooltip_93".to_string(),
            text: "Click here to activate feature 93. It is very simple.".to_string(),
            target_element: "button_93".to_string(),
        }
    );

    registry.articles.insert(
        "article_94".to_string(),
        HelpArticle {
            id: "article_94".to_string(),
            title: "How to use feature 94".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 94. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_94".to_string(),
        Tooltip {
            id: "tooltip_94".to_string(),
            text: "Click here to activate feature 94. It is very simple.".to_string(),
            target_element: "button_94".to_string(),
        }
    );

    registry.articles.insert(
        "article_95".to_string(),
        HelpArticle {
            id: "article_95".to_string(),
            title: "How to use feature 95".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 95. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_95".to_string(),
        Tooltip {
            id: "tooltip_95".to_string(),
            text: "Click here to activate feature 95. It is very simple.".to_string(),
            target_element: "button_95".to_string(),
        }
    );

    registry.articles.insert(
        "article_96".to_string(),
        HelpArticle {
            id: "article_96".to_string(),
            title: "How to use feature 96".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 96. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_96".to_string(),
        Tooltip {
            id: "tooltip_96".to_string(),
            text: "Click here to activate feature 96. It is very simple.".to_string(),
            target_element: "button_96".to_string(),
        }
    );

    registry.articles.insert(
        "article_97".to_string(),
        HelpArticle {
            id: "article_97".to_string(),
            title: "How to use feature 97".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 97. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_97".to_string(),
        Tooltip {
            id: "tooltip_97".to_string(),
            text: "Click here to activate feature 97. It is very simple.".to_string(),
            target_element: "button_97".to_string(),
        }
    );

    registry.articles.insert(
        "article_98".to_string(),
        HelpArticle {
            id: "article_98".to_string(),
            title: "How to use feature 98".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 98. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_98".to_string(),
        Tooltip {
            id: "tooltip_98".to_string(),
            text: "Click here to activate feature 98. It is very simple.".to_string(),
            target_element: "button_98".to_string(),
        }
    );

    registry.articles.insert(
        "article_99".to_string(),
        HelpArticle {
            id: "article_99".to_string(),
            title: "How to use feature 99".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 99. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_99".to_string(),
        Tooltip {
            id: "tooltip_99".to_string(),
            text: "Click here to activate feature 99. It is very simple.".to_string(),
            target_element: "button_99".to_string(),
        }
    );

    registry.articles.insert(
        "article_100".to_string(),
        HelpArticle {
            id: "article_100".to_string(),
            title: "How to use feature 100".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 100. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_100".to_string(),
        Tooltip {
            id: "tooltip_100".to_string(),
            text: "Click here to activate feature 100. It is very simple.".to_string(),
            target_element: "button_100".to_string(),
        }
    );

    registry.articles.insert(
        "article_101".to_string(),
        HelpArticle {
            id: "article_101".to_string(),
            title: "How to use feature 101".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 101. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_101".to_string(),
        Tooltip {
            id: "tooltip_101".to_string(),
            text: "Click here to activate feature 101. It is very simple.".to_string(),
            target_element: "button_101".to_string(),
        }
    );

    registry.articles.insert(
        "article_102".to_string(),
        HelpArticle {
            id: "article_102".to_string(),
            title: "How to use feature 102".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 102. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_102".to_string(),
        Tooltip {
            id: "tooltip_102".to_string(),
            text: "Click here to activate feature 102. It is very simple.".to_string(),
            target_element: "button_102".to_string(),
        }
    );

    registry.articles.insert(
        "article_103".to_string(),
        HelpArticle {
            id: "article_103".to_string(),
            title: "How to use feature 103".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 103. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_103".to_string(),
        Tooltip {
            id: "tooltip_103".to_string(),
            text: "Click here to activate feature 103. It is very simple.".to_string(),
            target_element: "button_103".to_string(),
        }
    );

    registry.articles.insert(
        "article_104".to_string(),
        HelpArticle {
            id: "article_104".to_string(),
            title: "How to use feature 104".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 104. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_104".to_string(),
        Tooltip {
            id: "tooltip_104".to_string(),
            text: "Click here to activate feature 104. It is very simple.".to_string(),
            target_element: "button_104".to_string(),
        }
    );

    registry.articles.insert(
        "article_105".to_string(),
        HelpArticle {
            id: "article_105".to_string(),
            title: "How to use feature 105".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 105. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_105".to_string(),
        Tooltip {
            id: "tooltip_105".to_string(),
            text: "Click here to activate feature 105. It is very simple.".to_string(),
            target_element: "button_105".to_string(),
        }
    );

    registry.articles.insert(
        "article_106".to_string(),
        HelpArticle {
            id: "article_106".to_string(),
            title: "How to use feature 106".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 106. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_106".to_string(),
        Tooltip {
            id: "tooltip_106".to_string(),
            text: "Click here to activate feature 106. It is very simple.".to_string(),
            target_element: "button_106".to_string(),
        }
    );

    registry.articles.insert(
        "article_107".to_string(),
        HelpArticle {
            id: "article_107".to_string(),
            title: "How to use feature 107".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 107. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_107".to_string(),
        Tooltip {
            id: "tooltip_107".to_string(),
            text: "Click here to activate feature 107. It is very simple.".to_string(),
            target_element: "button_107".to_string(),
        }
    );

    registry.articles.insert(
        "article_108".to_string(),
        HelpArticle {
            id: "article_108".to_string(),
            title: "How to use feature 108".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 108. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_108".to_string(),
        Tooltip {
            id: "tooltip_108".to_string(),
            text: "Click here to activate feature 108. It is very simple.".to_string(),
            target_element: "button_108".to_string(),
        }
    );

    registry.articles.insert(
        "article_109".to_string(),
        HelpArticle {
            id: "article_109".to_string(),
            title: "How to use feature 109".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 109. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_109".to_string(),
        Tooltip {
            id: "tooltip_109".to_string(),
            text: "Click here to activate feature 109. It is very simple.".to_string(),
            target_element: "button_109".to_string(),
        }
    );

    registry.articles.insert(
        "article_110".to_string(),
        HelpArticle {
            id: "article_110".to_string(),
            title: "How to use feature 110".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 110. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_110".to_string(),
        Tooltip {
            id: "tooltip_110".to_string(),
            text: "Click here to activate feature 110. It is very simple.".to_string(),
            target_element: "button_110".to_string(),
        }
    );

    registry.articles.insert(
        "article_111".to_string(),
        HelpArticle {
            id: "article_111".to_string(),
            title: "How to use feature 111".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 111. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_111".to_string(),
        Tooltip {
            id: "tooltip_111".to_string(),
            text: "Click here to activate feature 111. It is very simple.".to_string(),
            target_element: "button_111".to_string(),
        }
    );

    registry.articles.insert(
        "article_112".to_string(),
        HelpArticle {
            id: "article_112".to_string(),
            title: "How to use feature 112".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 112. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_112".to_string(),
        Tooltip {
            id: "tooltip_112".to_string(),
            text: "Click here to activate feature 112. It is very simple.".to_string(),
            target_element: "button_112".to_string(),
        }
    );

    registry.articles.insert(
        "article_113".to_string(),
        HelpArticle {
            id: "article_113".to_string(),
            title: "How to use feature 113".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 113. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_113".to_string(),
        Tooltip {
            id: "tooltip_113".to_string(),
            text: "Click here to activate feature 113. It is very simple.".to_string(),
            target_element: "button_113".to_string(),
        }
    );

    registry.articles.insert(
        "article_114".to_string(),
        HelpArticle {
            id: "article_114".to_string(),
            title: "How to use feature 114".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 114. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_114".to_string(),
        Tooltip {
            id: "tooltip_114".to_string(),
            text: "Click here to activate feature 114. It is very simple.".to_string(),
            target_element: "button_114".to_string(),
        }
    );

    registry.articles.insert(
        "article_115".to_string(),
        HelpArticle {
            id: "article_115".to_string(),
            title: "How to use feature 115".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 115. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_115".to_string(),
        Tooltip {
            id: "tooltip_115".to_string(),
            text: "Click here to activate feature 115. It is very simple.".to_string(),
            target_element: "button_115".to_string(),
        }
    );

    registry.articles.insert(
        "article_116".to_string(),
        HelpArticle {
            id: "article_116".to_string(),
            title: "How to use feature 116".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 116. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_116".to_string(),
        Tooltip {
            id: "tooltip_116".to_string(),
            text: "Click here to activate feature 116. It is very simple.".to_string(),
            target_element: "button_116".to_string(),
        }
    );

    registry.articles.insert(
        "article_117".to_string(),
        HelpArticle {
            id: "article_117".to_string(),
            title: "How to use feature 117".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 117. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_117".to_string(),
        Tooltip {
            id: "tooltip_117".to_string(),
            text: "Click here to activate feature 117. It is very simple.".to_string(),
            target_element: "button_117".to_string(),
        }
    );

    registry.articles.insert(
        "article_118".to_string(),
        HelpArticle {
            id: "article_118".to_string(),
            title: "How to use feature 118".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 118. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_118".to_string(),
        Tooltip {
            id: "tooltip_118".to_string(),
            text: "Click here to activate feature 118. It is very simple.".to_string(),
            target_element: "button_118".to_string(),
        }
    );

    registry.articles.insert(
        "article_119".to_string(),
        HelpArticle {
            id: "article_119".to_string(),
            title: "How to use feature 119".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 119. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_119".to_string(),
        Tooltip {
            id: "tooltip_119".to_string(),
            text: "Click here to activate feature 119. It is very simple.".to_string(),
            target_element: "button_119".to_string(),
        }
    );

    registry.articles.insert(
        "article_120".to_string(),
        HelpArticle {
            id: "article_120".to_string(),
            title: "How to use feature 120".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 120. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_120".to_string(),
        Tooltip {
            id: "tooltip_120".to_string(),
            text: "Click here to activate feature 120. It is very simple.".to_string(),
            target_element: "button_120".to_string(),
        }
    );

    registry.articles.insert(
        "article_121".to_string(),
        HelpArticle {
            id: "article_121".to_string(),
            title: "How to use feature 121".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 121. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_121".to_string(),
        Tooltip {
            id: "tooltip_121".to_string(),
            text: "Click here to activate feature 121. It is very simple.".to_string(),
            target_element: "button_121".to_string(),
        }
    );

    registry.articles.insert(
        "article_122".to_string(),
        HelpArticle {
            id: "article_122".to_string(),
            title: "How to use feature 122".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 122. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_122".to_string(),
        Tooltip {
            id: "tooltip_122".to_string(),
            text: "Click here to activate feature 122. It is very simple.".to_string(),
            target_element: "button_122".to_string(),
        }
    );

    registry.articles.insert(
        "article_123".to_string(),
        HelpArticle {
            id: "article_123".to_string(),
            title: "How to use feature 123".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 123. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_123".to_string(),
        Tooltip {
            id: "tooltip_123".to_string(),
            text: "Click here to activate feature 123. It is very simple.".to_string(),
            target_element: "button_123".to_string(),
        }
    );

    registry.articles.insert(
        "article_124".to_string(),
        HelpArticle {
            id: "article_124".to_string(),
            title: "How to use feature 124".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 124. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_124".to_string(),
        Tooltip {
            id: "tooltip_124".to_string(),
            text: "Click here to activate feature 124. It is very simple.".to_string(),
            target_element: "button_124".to_string(),
        }
    );

    registry.articles.insert(
        "article_125".to_string(),
        HelpArticle {
            id: "article_125".to_string(),
            title: "How to use feature 125".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 125. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_125".to_string(),
        Tooltip {
            id: "tooltip_125".to_string(),
            text: "Click here to activate feature 125. It is very simple.".to_string(),
            target_element: "button_125".to_string(),
        }
    );

    registry.articles.insert(
        "article_126".to_string(),
        HelpArticle {
            id: "article_126".to_string(),
            title: "How to use feature 126".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 126. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_126".to_string(),
        Tooltip {
            id: "tooltip_126".to_string(),
            text: "Click here to activate feature 126. It is very simple.".to_string(),
            target_element: "button_126".to_string(),
        }
    );

    registry.articles.insert(
        "article_127".to_string(),
        HelpArticle {
            id: "article_127".to_string(),
            title: "How to use feature 127".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 127. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_127".to_string(),
        Tooltip {
            id: "tooltip_127".to_string(),
            text: "Click here to activate feature 127. It is very simple.".to_string(),
            target_element: "button_127".to_string(),
        }
    );

    registry.articles.insert(
        "article_128".to_string(),
        HelpArticle {
            id: "article_128".to_string(),
            title: "How to use feature 128".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 128. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_128".to_string(),
        Tooltip {
            id: "tooltip_128".to_string(),
            text: "Click here to activate feature 128. It is very simple.".to_string(),
            target_element: "button_128".to_string(),
        }
    );

    registry.articles.insert(
        "article_129".to_string(),
        HelpArticle {
            id: "article_129".to_string(),
            title: "How to use feature 129".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 129. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_129".to_string(),
        Tooltip {
            id: "tooltip_129".to_string(),
            text: "Click here to activate feature 129. It is very simple.".to_string(),
            target_element: "button_129".to_string(),
        }
    );

    registry.articles.insert(
        "article_130".to_string(),
        HelpArticle {
            id: "article_130".to_string(),
            title: "How to use feature 130".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 130. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_130".to_string(),
        Tooltip {
            id: "tooltip_130".to_string(),
            text: "Click here to activate feature 130. It is very simple.".to_string(),
            target_element: "button_130".to_string(),
        }
    );

    registry.articles.insert(
        "article_131".to_string(),
        HelpArticle {
            id: "article_131".to_string(),
            title: "How to use feature 131".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 131. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_131".to_string(),
        Tooltip {
            id: "tooltip_131".to_string(),
            text: "Click here to activate feature 131. It is very simple.".to_string(),
            target_element: "button_131".to_string(),
        }
    );

    registry.articles.insert(
        "article_132".to_string(),
        HelpArticle {
            id: "article_132".to_string(),
            title: "How to use feature 132".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 132. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_132".to_string(),
        Tooltip {
            id: "tooltip_132".to_string(),
            text: "Click here to activate feature 132. It is very simple.".to_string(),
            target_element: "button_132".to_string(),
        }
    );

    registry.articles.insert(
        "article_133".to_string(),
        HelpArticle {
            id: "article_133".to_string(),
            title: "How to use feature 133".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 133. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_133".to_string(),
        Tooltip {
            id: "tooltip_133".to_string(),
            text: "Click here to activate feature 133. It is very simple.".to_string(),
            target_element: "button_133".to_string(),
        }
    );

    registry.articles.insert(
        "article_134".to_string(),
        HelpArticle {
            id: "article_134".to_string(),
            title: "How to use feature 134".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 134. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_134".to_string(),
        Tooltip {
            id: "tooltip_134".to_string(),
            text: "Click here to activate feature 134. It is very simple.".to_string(),
            target_element: "button_134".to_string(),
        }
    );

    registry.articles.insert(
        "article_135".to_string(),
        HelpArticle {
            id: "article_135".to_string(),
            title: "How to use feature 135".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 135. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_135".to_string(),
        Tooltip {
            id: "tooltip_135".to_string(),
            text: "Click here to activate feature 135. It is very simple.".to_string(),
            target_element: "button_135".to_string(),
        }
    );

    registry.articles.insert(
        "article_136".to_string(),
        HelpArticle {
            id: "article_136".to_string(),
            title: "How to use feature 136".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 136. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_136".to_string(),
        Tooltip {
            id: "tooltip_136".to_string(),
            text: "Click here to activate feature 136. It is very simple.".to_string(),
            target_element: "button_136".to_string(),
        }
    );

    registry.articles.insert(
        "article_137".to_string(),
        HelpArticle {
            id: "article_137".to_string(),
            title: "How to use feature 137".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 137. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_137".to_string(),
        Tooltip {
            id: "tooltip_137".to_string(),
            text: "Click here to activate feature 137. It is very simple.".to_string(),
            target_element: "button_137".to_string(),
        }
    );

    registry.articles.insert(
        "article_138".to_string(),
        HelpArticle {
            id: "article_138".to_string(),
            title: "How to use feature 138".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 138. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_138".to_string(),
        Tooltip {
            id: "tooltip_138".to_string(),
            text: "Click here to activate feature 138. It is very simple.".to_string(),
            target_element: "button_138".to_string(),
        }
    );

    registry.articles.insert(
        "article_139".to_string(),
        HelpArticle {
            id: "article_139".to_string(),
            title: "How to use feature 139".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 139. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_139".to_string(),
        Tooltip {
            id: "tooltip_139".to_string(),
            text: "Click here to activate feature 139. It is very simple.".to_string(),
            target_element: "button_139".to_string(),
        }
    );

    registry.articles.insert(
        "article_140".to_string(),
        HelpArticle {
            id: "article_140".to_string(),
            title: "How to use feature 140".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 140. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_140".to_string(),
        Tooltip {
            id: "tooltip_140".to_string(),
            text: "Click here to activate feature 140. It is very simple.".to_string(),
            target_element: "button_140".to_string(),
        }
    );

    registry.articles.insert(
        "article_141".to_string(),
        HelpArticle {
            id: "article_141".to_string(),
            title: "How to use feature 141".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 141. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_141".to_string(),
        Tooltip {
            id: "tooltip_141".to_string(),
            text: "Click here to activate feature 141. It is very simple.".to_string(),
            target_element: "button_141".to_string(),
        }
    );

    registry.articles.insert(
        "article_142".to_string(),
        HelpArticle {
            id: "article_142".to_string(),
            title: "How to use feature 142".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 142. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_142".to_string(),
        Tooltip {
            id: "tooltip_142".to_string(),
            text: "Click here to activate feature 142. It is very simple.".to_string(),
            target_element: "button_142".to_string(),
        }
    );

    registry.articles.insert(
        "article_143".to_string(),
        HelpArticle {
            id: "article_143".to_string(),
            title: "How to use feature 143".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 143. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_143".to_string(),
        Tooltip {
            id: "tooltip_143".to_string(),
            text: "Click here to activate feature 143. It is very simple.".to_string(),
            target_element: "button_143".to_string(),
        }
    );

    registry.articles.insert(
        "article_144".to_string(),
        HelpArticle {
            id: "article_144".to_string(),
            title: "How to use feature 144".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 144. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_144".to_string(),
        Tooltip {
            id: "tooltip_144".to_string(),
            text: "Click here to activate feature 144. It is very simple.".to_string(),
            target_element: "button_144".to_string(),
        }
    );

    registry.articles.insert(
        "article_145".to_string(),
        HelpArticle {
            id: "article_145".to_string(),
            title: "How to use feature 145".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 145. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_145".to_string(),
        Tooltip {
            id: "tooltip_145".to_string(),
            text: "Click here to activate feature 145. It is very simple.".to_string(),
            target_element: "button_145".to_string(),
        }
    );

    registry.articles.insert(
        "article_146".to_string(),
        HelpArticle {
            id: "article_146".to_string(),
            title: "How to use feature 146".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 146. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_146".to_string(),
        Tooltip {
            id: "tooltip_146".to_string(),
            text: "Click here to activate feature 146. It is very simple.".to_string(),
            target_element: "button_146".to_string(),
        }
    );

    registry.articles.insert(
        "article_147".to_string(),
        HelpArticle {
            id: "article_147".to_string(),
            title: "How to use feature 147".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 147. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_147".to_string(),
        Tooltip {
            id: "tooltip_147".to_string(),
            text: "Click here to activate feature 147. It is very simple.".to_string(),
            target_element: "button_147".to_string(),
        }
    );

    registry.articles.insert(
        "article_148".to_string(),
        HelpArticle {
            id: "article_148".to_string(),
            title: "How to use feature 148".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 148. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_148".to_string(),
        Tooltip {
            id: "tooltip_148".to_string(),
            text: "Click here to activate feature 148. It is very simple.".to_string(),
            target_element: "button_148".to_string(),
        }
    );

    registry.articles.insert(
        "article_149".to_string(),
        HelpArticle {
            id: "article_149".to_string(),
            title: "How to use feature 149".to_string(),
            content: "This is a plain language guide for small business owners on how to use feature 149. It is very easy and straightforward. Just click the button and follow the instructions. If you need more help, please contact support. You will find it very helpful for your daily operations. This ensures you can focus on growing your business instead of worrying about technical details. Thank you for using OneHumanCorp.".to_string(),
            category: "Getting Started".to_string(),
            tags: vec!["feature".to_string(), "guide".to_string()],
        }
    );

    registry.tooltips.insert(
        "tooltip_149".to_string(),
        Tooltip {
            id: "tooltip_149".to_string(),
            text: "Click here to activate feature 149. It is very simple.".to_string(),
            target_element: "button_149".to_string(),
        }
    );
}
