use super::engine::{DocEngine, Article};


#[test]
fn test_doc_engine_refund1_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "refund1".to_string(),
        title: "Processing Refunds Part 1".to_string(),
        content: "Step by step guide to reversing a Stripe charge. This covers specific use case 1.".to_string(),
        category: "Payments".to_string(),
        tags: vec!["stripe".to_string(), "money".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("refund1");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "refund1");
}

#[test]
fn test_doc_engine_refund1_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "refund1".to_string(),
        title: "Processing Refunds Advanced 1".to_string(),
        content: "Step by step guide to reversing a Stripe charge. Specific details for scenario 1.".to_string(),
        category: "Payments".to_string(),
        tags: vec!["stripe".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("Processing");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "refund1");
}


#[test]
fn test_doc_engine_marketing2_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "marketing2".to_string(),
        title: "Email Marketing 101 Part 2".to_string(),
        content: "How to write subject lines that convert. This covers specific use case 2.".to_string(),
        category: "Marketing".to_string(),
        tags: vec!["email".to_string(), "sales".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("marketing2");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "marketing2");
}

#[test]
fn test_doc_engine_marketing2_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "marketing2".to_string(),
        title: "Email Marketing 101 Advanced 2".to_string(),
        content: "How to write subject lines that convert. Specific details for scenario 2.".to_string(),
        category: "Marketing".to_string(),
        tags: vec!["email".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("Email");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "marketing2");
}


#[test]
fn test_doc_engine_inventory3_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "inventory3".to_string(),
        title: "Managing Stock Part 3".to_string(),
        content: "What to do when you run out of product. This covers specific use case 3.".to_string(),
        category: "Inventory".to_string(),
        tags: vec!["stock".to_string(), "warehouse".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("inventory3");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "inventory3");
}

#[test]
fn test_doc_engine_inventory3_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "inventory3".to_string(),
        title: "Managing Stock Advanced 3".to_string(),
        content: "What to do when you run out of product. Specific details for scenario 3.".to_string(),
        category: "Inventory".to_string(),
        tags: vec!["stock".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("Managing");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "inventory3");
}


#[test]
fn test_doc_engine_support4_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "support4".to_string(),
        title: "Handling Angry Customers Part 4".to_string(),
        content: "De-escalation tactics for retail. This covers specific use case 4.".to_string(),
        category: "Support".to_string(),
        tags: vec!["angry".to_string(), "help".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("support4");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "support4");
}

#[test]
fn test_doc_engine_support4_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "support4".to_string(),
        title: "Handling Angry Customers Advanced 4".to_string(),
        content: "De-escalation tactics for retail. Specific details for scenario 4.".to_string(),
        category: "Support".to_string(),
        tags: vec!["angry".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("Handling");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "support4");
}


#[test]
fn test_doc_engine_design5_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "design5".to_string(),
        title: "Customizing CSS Part 5".to_string(),
        content: "How to change your storefront colors. This covers specific use case 5.".to_string(),
        category: "Design".to_string(),
        tags: vec!["css".to_string(), "styling".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("design5");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "design5");
}

#[test]
fn test_doc_engine_design5_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "design5".to_string(),
        title: "Customizing CSS Advanced 5".to_string(),
        content: "How to change your storefront colors. Specific details for scenario 5.".to_string(),
        category: "Design".to_string(),
        tags: vec!["css".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("Customizing");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "design5");
}


#[test]
fn test_doc_engine_api6_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "api6".to_string(),
        title: "Using the GraphQL API Part 6".to_string(),
        content: "A guide to generating access tokens. This covers specific use case 6.".to_string(),
        category: "Engineering".to_string(),
        tags: vec!["graphql".to_string(), "code".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("api6");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "api6");
}

#[test]
fn test_doc_engine_api6_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "api6".to_string(),
        title: "Using the GraphQL API Advanced 6".to_string(),
        content: "A guide to generating access tokens. Specific details for scenario 6.".to_string(),
        category: "Engineering".to_string(),
        tags: vec!["graphql".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("Using");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "api6");
}


#[test]
fn test_doc_engine_shipping7_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "shipping7".to_string(),
        title: "FedEx Integration Part 7".to_string(),
        content: "How to print labels automatically. This covers specific use case 7.".to_string(),
        category: "Shipping".to_string(),
        tags: vec!["fedex".to_string(), "labels".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("shipping7");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "shipping7");
}

#[test]
fn test_doc_engine_shipping7_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "shipping7".to_string(),
        title: "FedEx Integration Advanced 7".to_string(),
        content: "How to print labels automatically. Specific details for scenario 7.".to_string(),
        category: "Shipping".to_string(),
        tags: vec!["fedex".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("FedEx");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "shipping7");
}


#[test]
fn test_doc_engine_security8_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "security8".to_string(),
        title: "Two Factor Auth Part 8".to_string(),
        content: "Securing your account with SMS. This covers specific use case 8.".to_string(),
        category: "Security".to_string(),
        tags: vec!["2fa".to_string(), "sms".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("security8");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "security8");
}

#[test]
fn test_doc_engine_security8_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "security8".to_string(),
        title: "Two Factor Auth Advanced 8".to_string(),
        content: "Securing your account with SMS. Specific details for scenario 8.".to_string(),
        category: "Security".to_string(),
        tags: vec!["2fa".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("Two");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "security8");
}


#[test]
fn test_doc_engine_billing9_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "billing9".to_string(),
        title: "Updating Credit Cards Part 9".to_string(),
        content: "How to change your subscription payment method. This covers specific use case 9.".to_string(),
        category: "Billing".to_string(),
        tags: vec!["credit card".to_string(), "subscription".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("billing9");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "billing9");
}

#[test]
fn test_doc_engine_billing9_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "billing9".to_string(),
        title: "Updating Credit Cards Advanced 9".to_string(),
        content: "How to change your subscription payment method. Specific details for scenario 9.".to_string(),
        category: "Billing".to_string(),
        tags: vec!["credit card".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("Updating");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "billing9");
}


#[test]
fn test_doc_engine_tax10_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "tax10".to_string(),
        title: "How to File Taxes Part 10".to_string(),
        content: "Understanding schedule C and quarterly estimates. This covers specific use case 10.".to_string(),
        category: "Accounting".to_string(),
        tags: vec!["finance".to_string(), "irs".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("tax10");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "tax10");
}

#[test]
fn test_doc_engine_tax10_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "tax10".to_string(),
        title: "How to File Taxes Advanced 10".to_string(),
        content: "Understanding schedule C and quarterly estimates. Specific details for scenario 10.".to_string(),
        category: "Accounting".to_string(),
        tags: vec!["finance".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("How");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "tax10");
}


#[test]
fn test_doc_engine_refund11_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "refund11".to_string(),
        title: "Processing Refunds Part 11".to_string(),
        content: "Step by step guide to reversing a Stripe charge. This covers specific use case 11.".to_string(),
        category: "Payments".to_string(),
        tags: vec!["stripe".to_string(), "money".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("refund11");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "refund11");
}

#[test]
fn test_doc_engine_refund11_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "refund11".to_string(),
        title: "Processing Refunds Advanced 11".to_string(),
        content: "Step by step guide to reversing a Stripe charge. Specific details for scenario 11.".to_string(),
        category: "Payments".to_string(),
        tags: vec!["stripe".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("Processing");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "refund11");
}


#[test]
fn test_doc_engine_marketing12_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "marketing12".to_string(),
        title: "Email Marketing 101 Part 12".to_string(),
        content: "How to write subject lines that convert. This covers specific use case 12.".to_string(),
        category: "Marketing".to_string(),
        tags: vec!["email".to_string(), "sales".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("marketing12");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "marketing12");
}

#[test]
fn test_doc_engine_marketing12_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "marketing12".to_string(),
        title: "Email Marketing 101 Advanced 12".to_string(),
        content: "How to write subject lines that convert. Specific details for scenario 12.".to_string(),
        category: "Marketing".to_string(),
        tags: vec!["email".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("Email");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "marketing12");
}


#[test]
fn test_doc_engine_inventory13_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "inventory13".to_string(),
        title: "Managing Stock Part 13".to_string(),
        content: "What to do when you run out of product. This covers specific use case 13.".to_string(),
        category: "Inventory".to_string(),
        tags: vec!["stock".to_string(), "warehouse".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("inventory13");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "inventory13");
}

#[test]
fn test_doc_engine_inventory13_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "inventory13".to_string(),
        title: "Managing Stock Advanced 13".to_string(),
        content: "What to do when you run out of product. Specific details for scenario 13.".to_string(),
        category: "Inventory".to_string(),
        tags: vec!["stock".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("Managing");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "inventory13");
}


#[test]
fn test_doc_engine_support14_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "support14".to_string(),
        title: "Handling Angry Customers Part 14".to_string(),
        content: "De-escalation tactics for retail. This covers specific use case 14.".to_string(),
        category: "Support".to_string(),
        tags: vec!["angry".to_string(), "help".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("support14");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "support14");
}

#[test]
fn test_doc_engine_support14_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "support14".to_string(),
        title: "Handling Angry Customers Advanced 14".to_string(),
        content: "De-escalation tactics for retail. Specific details for scenario 14.".to_string(),
        category: "Support".to_string(),
        tags: vec!["angry".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("Handling");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "support14");
}


#[test]
fn test_doc_engine_design15_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "design15".to_string(),
        title: "Customizing CSS Part 15".to_string(),
        content: "How to change your storefront colors. This covers specific use case 15.".to_string(),
        category: "Design".to_string(),
        tags: vec!["css".to_string(), "styling".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("design15");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "design15");
}

#[test]
fn test_doc_engine_design15_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "design15".to_string(),
        title: "Customizing CSS Advanced 15".to_string(),
        content: "How to change your storefront colors. Specific details for scenario 15.".to_string(),
        category: "Design".to_string(),
        tags: vec!["css".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("Customizing");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "design15");
}


#[test]
fn test_doc_engine_api16_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "api16".to_string(),
        title: "Using the GraphQL API Part 16".to_string(),
        content: "A guide to generating access tokens. This covers specific use case 16.".to_string(),
        category: "Engineering".to_string(),
        tags: vec!["graphql".to_string(), "code".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("api16");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "api16");
}

#[test]
fn test_doc_engine_api16_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "api16".to_string(),
        title: "Using the GraphQL API Advanced 16".to_string(),
        content: "A guide to generating access tokens. Specific details for scenario 16.".to_string(),
        category: "Engineering".to_string(),
        tags: vec!["graphql".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("Using");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "api16");
}


#[test]
fn test_doc_engine_shipping17_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "shipping17".to_string(),
        title: "FedEx Integration Part 17".to_string(),
        content: "How to print labels automatically. This covers specific use case 17.".to_string(),
        category: "Shipping".to_string(),
        tags: vec!["fedex".to_string(), "labels".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("shipping17");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "shipping17");
}

#[test]
fn test_doc_engine_shipping17_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "shipping17".to_string(),
        title: "FedEx Integration Advanced 17".to_string(),
        content: "How to print labels automatically. Specific details for scenario 17.".to_string(),
        category: "Shipping".to_string(),
        tags: vec!["fedex".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("FedEx");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "shipping17");
}


#[test]
fn test_doc_engine_security18_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "security18".to_string(),
        title: "Two Factor Auth Part 18".to_string(),
        content: "Securing your account with SMS. This covers specific use case 18.".to_string(),
        category: "Security".to_string(),
        tags: vec!["2fa".to_string(), "sms".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("security18");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "security18");
}

#[test]
fn test_doc_engine_security18_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "security18".to_string(),
        title: "Two Factor Auth Advanced 18".to_string(),
        content: "Securing your account with SMS. Specific details for scenario 18.".to_string(),
        category: "Security".to_string(),
        tags: vec!["2fa".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("Two");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "security18");
}


#[test]
fn test_doc_engine_billing19_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "billing19".to_string(),
        title: "Updating Credit Cards Part 19".to_string(),
        content: "How to change your subscription payment method. This covers specific use case 19.".to_string(),
        category: "Billing".to_string(),
        tags: vec!["credit card".to_string(), "subscription".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("billing19");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "billing19");
}

#[test]
fn test_doc_engine_billing19_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "billing19".to_string(),
        title: "Updating Credit Cards Advanced 19".to_string(),
        content: "How to change your subscription payment method. Specific details for scenario 19.".to_string(),
        category: "Billing".to_string(),
        tags: vec!["credit card".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("Updating");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "billing19");
}


#[test]
fn test_doc_engine_tax20_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "tax20".to_string(),
        title: "How to File Taxes Part 20".to_string(),
        content: "Understanding schedule C and quarterly estimates. This covers specific use case 20.".to_string(),
        category: "Accounting".to_string(),
        tags: vec!["finance".to_string(), "irs".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("tax20");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "tax20");
}

#[test]
fn test_doc_engine_tax20_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "tax20".to_string(),
        title: "How to File Taxes Advanced 20".to_string(),
        content: "Understanding schedule C and quarterly estimates. Specific details for scenario 20.".to_string(),
        category: "Accounting".to_string(),
        tags: vec!["finance".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("How");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "tax20");
}


#[test]
fn test_doc_engine_refund21_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "refund21".to_string(),
        title: "Processing Refunds Part 21".to_string(),
        content: "Step by step guide to reversing a Stripe charge. This covers specific use case 21.".to_string(),
        category: "Payments".to_string(),
        tags: vec!["stripe".to_string(), "money".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("refund21");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "refund21");
}

#[test]
fn test_doc_engine_refund21_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "refund21".to_string(),
        title: "Processing Refunds Advanced 21".to_string(),
        content: "Step by step guide to reversing a Stripe charge. Specific details for scenario 21.".to_string(),
        category: "Payments".to_string(),
        tags: vec!["stripe".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("Processing");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "refund21");
}


#[test]
fn test_doc_engine_marketing22_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "marketing22".to_string(),
        title: "Email Marketing 101 Part 22".to_string(),
        content: "How to write subject lines that convert. This covers specific use case 22.".to_string(),
        category: "Marketing".to_string(),
        tags: vec!["email".to_string(), "sales".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("marketing22");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "marketing22");
}

#[test]
fn test_doc_engine_marketing22_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "marketing22".to_string(),
        title: "Email Marketing 101 Advanced 22".to_string(),
        content: "How to write subject lines that convert. Specific details for scenario 22.".to_string(),
        category: "Marketing".to_string(),
        tags: vec!["email".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("Email");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "marketing22");
}


#[test]
fn test_doc_engine_inventory23_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "inventory23".to_string(),
        title: "Managing Stock Part 23".to_string(),
        content: "What to do when you run out of product. This covers specific use case 23.".to_string(),
        category: "Inventory".to_string(),
        tags: vec!["stock".to_string(), "warehouse".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("inventory23");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "inventory23");
}

#[test]
fn test_doc_engine_inventory23_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "inventory23".to_string(),
        title: "Managing Stock Advanced 23".to_string(),
        content: "What to do when you run out of product. Specific details for scenario 23.".to_string(),
        category: "Inventory".to_string(),
        tags: vec!["stock".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("Managing");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "inventory23");
}


#[test]
fn test_doc_engine_support24_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "support24".to_string(),
        title: "Handling Angry Customers Part 24".to_string(),
        content: "De-escalation tactics for retail. This covers specific use case 24.".to_string(),
        category: "Support".to_string(),
        tags: vec!["angry".to_string(), "help".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("support24");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "support24");
}

#[test]
fn test_doc_engine_support24_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "support24".to_string(),
        title: "Handling Angry Customers Advanced 24".to_string(),
        content: "De-escalation tactics for retail. Specific details for scenario 24.".to_string(),
        category: "Support".to_string(),
        tags: vec!["angry".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("Handling");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "support24");
}


#[test]
fn test_doc_engine_design25_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "design25".to_string(),
        title: "Customizing CSS Part 25".to_string(),
        content: "How to change your storefront colors. This covers specific use case 25.".to_string(),
        category: "Design".to_string(),
        tags: vec!["css".to_string(), "styling".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("design25");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "design25");
}

#[test]
fn test_doc_engine_design25_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "design25".to_string(),
        title: "Customizing CSS Advanced 25".to_string(),
        content: "How to change your storefront colors. Specific details for scenario 25.".to_string(),
        category: "Design".to_string(),
        tags: vec!["css".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("Customizing");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "design25");
}


#[test]
fn test_doc_engine_api26_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "api26".to_string(),
        title: "Using the GraphQL API Part 26".to_string(),
        content: "A guide to generating access tokens. This covers specific use case 26.".to_string(),
        category: "Engineering".to_string(),
        tags: vec!["graphql".to_string(), "code".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("api26");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "api26");
}

#[test]
fn test_doc_engine_api26_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "api26".to_string(),
        title: "Using the GraphQL API Advanced 26".to_string(),
        content: "A guide to generating access tokens. Specific details for scenario 26.".to_string(),
        category: "Engineering".to_string(),
        tags: vec!["graphql".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("Using");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "api26");
}


#[test]
fn test_doc_engine_shipping27_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "shipping27".to_string(),
        title: "FedEx Integration Part 27".to_string(),
        content: "How to print labels automatically. This covers specific use case 27.".to_string(),
        category: "Shipping".to_string(),
        tags: vec!["fedex".to_string(), "labels".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("shipping27");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "shipping27");
}

#[test]
fn test_doc_engine_shipping27_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "shipping27".to_string(),
        title: "FedEx Integration Advanced 27".to_string(),
        content: "How to print labels automatically. Specific details for scenario 27.".to_string(),
        category: "Shipping".to_string(),
        tags: vec!["fedex".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("FedEx");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "shipping27");
}


#[test]
fn test_doc_engine_security28_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "security28".to_string(),
        title: "Two Factor Auth Part 28".to_string(),
        content: "Securing your account with SMS. This covers specific use case 28.".to_string(),
        category: "Security".to_string(),
        tags: vec!["2fa".to_string(), "sms".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("security28");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "security28");
}

#[test]
fn test_doc_engine_security28_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "security28".to_string(),
        title: "Two Factor Auth Advanced 28".to_string(),
        content: "Securing your account with SMS. Specific details for scenario 28.".to_string(),
        category: "Security".to_string(),
        tags: vec!["2fa".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("Two");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "security28");
}


#[test]
fn test_doc_engine_billing29_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "billing29".to_string(),
        title: "Updating Credit Cards Part 29".to_string(),
        content: "How to change your subscription payment method. This covers specific use case 29.".to_string(),
        category: "Billing".to_string(),
        tags: vec!["credit card".to_string(), "subscription".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("billing29");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "billing29");
}

#[test]
fn test_doc_engine_billing29_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "billing29".to_string(),
        title: "Updating Credit Cards Advanced 29".to_string(),
        content: "How to change your subscription payment method. Specific details for scenario 29.".to_string(),
        category: "Billing".to_string(),
        tags: vec!["credit card".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("Updating");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "billing29");
}


#[test]
fn test_doc_engine_tax30_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "tax30".to_string(),
        title: "How to File Taxes Part 30".to_string(),
        content: "Understanding schedule C and quarterly estimates. This covers specific use case 30.".to_string(),
        category: "Accounting".to_string(),
        tags: vec!["finance".to_string(), "irs".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("tax30");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "tax30");
}

#[test]
fn test_doc_engine_tax30_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "tax30".to_string(),
        title: "How to File Taxes Advanced 30".to_string(),
        content: "Understanding schedule C and quarterly estimates. Specific details for scenario 30.".to_string(),
        category: "Accounting".to_string(),
        tags: vec!["finance".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("How");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "tax30");
}


#[test]
fn test_doc_engine_refund31_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "refund31".to_string(),
        title: "Processing Refunds Part 31".to_string(),
        content: "Step by step guide to reversing a Stripe charge. This covers specific use case 31.".to_string(),
        category: "Payments".to_string(),
        tags: vec!["stripe".to_string(), "money".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("refund31");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "refund31");
}

#[test]
fn test_doc_engine_refund31_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "refund31".to_string(),
        title: "Processing Refunds Advanced 31".to_string(),
        content: "Step by step guide to reversing a Stripe charge. Specific details for scenario 31.".to_string(),
        category: "Payments".to_string(),
        tags: vec!["stripe".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("Processing");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "refund31");
}


#[test]
fn test_doc_engine_marketing32_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "marketing32".to_string(),
        title: "Email Marketing 101 Part 32".to_string(),
        content: "How to write subject lines that convert. This covers specific use case 32.".to_string(),
        category: "Marketing".to_string(),
        tags: vec!["email".to_string(), "sales".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("marketing32");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "marketing32");
}

#[test]
fn test_doc_engine_marketing32_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "marketing32".to_string(),
        title: "Email Marketing 101 Advanced 32".to_string(),
        content: "How to write subject lines that convert. Specific details for scenario 32.".to_string(),
        category: "Marketing".to_string(),
        tags: vec!["email".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("Email");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "marketing32");
}


#[test]
fn test_doc_engine_inventory33_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "inventory33".to_string(),
        title: "Managing Stock Part 33".to_string(),
        content: "What to do when you run out of product. This covers specific use case 33.".to_string(),
        category: "Inventory".to_string(),
        tags: vec!["stock".to_string(), "warehouse".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("inventory33");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "inventory33");
}

#[test]
fn test_doc_engine_inventory33_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "inventory33".to_string(),
        title: "Managing Stock Advanced 33".to_string(),
        content: "What to do when you run out of product. Specific details for scenario 33.".to_string(),
        category: "Inventory".to_string(),
        tags: vec!["stock".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("Managing");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "inventory33");
}


#[test]
fn test_doc_engine_support34_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "support34".to_string(),
        title: "Handling Angry Customers Part 34".to_string(),
        content: "De-escalation tactics for retail. This covers specific use case 34.".to_string(),
        category: "Support".to_string(),
        tags: vec!["angry".to_string(), "help".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("support34");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "support34");
}

#[test]
fn test_doc_engine_support34_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "support34".to_string(),
        title: "Handling Angry Customers Advanced 34".to_string(),
        content: "De-escalation tactics for retail. Specific details for scenario 34.".to_string(),
        category: "Support".to_string(),
        tags: vec!["angry".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("Handling");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "support34");
}


#[test]
fn test_doc_engine_design35_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "design35".to_string(),
        title: "Customizing CSS Part 35".to_string(),
        content: "How to change your storefront colors. This covers specific use case 35.".to_string(),
        category: "Design".to_string(),
        tags: vec!["css".to_string(), "styling".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("design35");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "design35");
}

#[test]
fn test_doc_engine_design35_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "design35".to_string(),
        title: "Customizing CSS Advanced 35".to_string(),
        content: "How to change your storefront colors. Specific details for scenario 35.".to_string(),
        category: "Design".to_string(),
        tags: vec!["css".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("Customizing");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "design35");
}


#[test]
fn test_doc_engine_api36_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "api36".to_string(),
        title: "Using the GraphQL API Part 36".to_string(),
        content: "A guide to generating access tokens. This covers specific use case 36.".to_string(),
        category: "Engineering".to_string(),
        tags: vec!["graphql".to_string(), "code".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("api36");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "api36");
}

#[test]
fn test_doc_engine_api36_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "api36".to_string(),
        title: "Using the GraphQL API Advanced 36".to_string(),
        content: "A guide to generating access tokens. Specific details for scenario 36.".to_string(),
        category: "Engineering".to_string(),
        tags: vec!["graphql".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("Using");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "api36");
}


#[test]
fn test_doc_engine_shipping37_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "shipping37".to_string(),
        title: "FedEx Integration Part 37".to_string(),
        content: "How to print labels automatically. This covers specific use case 37.".to_string(),
        category: "Shipping".to_string(),
        tags: vec!["fedex".to_string(), "labels".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("shipping37");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "shipping37");
}

#[test]
fn test_doc_engine_shipping37_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "shipping37".to_string(),
        title: "FedEx Integration Advanced 37".to_string(),
        content: "How to print labels automatically. Specific details for scenario 37.".to_string(),
        category: "Shipping".to_string(),
        tags: vec!["fedex".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("FedEx");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "shipping37");
}


#[test]
fn test_doc_engine_security38_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "security38".to_string(),
        title: "Two Factor Auth Part 38".to_string(),
        content: "Securing your account with SMS. This covers specific use case 38.".to_string(),
        category: "Security".to_string(),
        tags: vec!["2fa".to_string(), "sms".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("security38");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "security38");
}

#[test]
fn test_doc_engine_security38_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "security38".to_string(),
        title: "Two Factor Auth Advanced 38".to_string(),
        content: "Securing your account with SMS. Specific details for scenario 38.".to_string(),
        category: "Security".to_string(),
        tags: vec!["2fa".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("Two");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "security38");
}


#[test]
fn test_doc_engine_billing39_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "billing39".to_string(),
        title: "Updating Credit Cards Part 39".to_string(),
        content: "How to change your subscription payment method. This covers specific use case 39.".to_string(),
        category: "Billing".to_string(),
        tags: vec!["credit card".to_string(), "subscription".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("billing39");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "billing39");
}

#[test]
fn test_doc_engine_billing39_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "billing39".to_string(),
        title: "Updating Credit Cards Advanced 39".to_string(),
        content: "How to change your subscription payment method. Specific details for scenario 39.".to_string(),
        category: "Billing".to_string(),
        tags: vec!["credit card".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("Updating");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "billing39");
}


#[test]
fn test_doc_engine_tax40_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "tax40".to_string(),
        title: "How to File Taxes Part 40".to_string(),
        content: "Understanding schedule C and quarterly estimates. This covers specific use case 40.".to_string(),
        category: "Accounting".to_string(),
        tags: vec!["finance".to_string(), "irs".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("tax40");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "tax40");
}

#[test]
fn test_doc_engine_tax40_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "tax40".to_string(),
        title: "How to File Taxes Advanced 40".to_string(),
        content: "Understanding schedule C and quarterly estimates. Specific details for scenario 40.".to_string(),
        category: "Accounting".to_string(),
        tags: vec!["finance".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("How");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "tax40");
}


#[test]
fn test_doc_engine_refund41_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "refund41".to_string(),
        title: "Processing Refunds Part 41".to_string(),
        content: "Step by step guide to reversing a Stripe charge. This covers specific use case 41.".to_string(),
        category: "Payments".to_string(),
        tags: vec!["stripe".to_string(), "money".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("refund41");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "refund41");
}

#[test]
fn test_doc_engine_refund41_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "refund41".to_string(),
        title: "Processing Refunds Advanced 41".to_string(),
        content: "Step by step guide to reversing a Stripe charge. Specific details for scenario 41.".to_string(),
        category: "Payments".to_string(),
        tags: vec!["stripe".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("Processing");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "refund41");
}


#[test]
fn test_doc_engine_marketing42_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "marketing42".to_string(),
        title: "Email Marketing 101 Part 42".to_string(),
        content: "How to write subject lines that convert. This covers specific use case 42.".to_string(),
        category: "Marketing".to_string(),
        tags: vec!["email".to_string(), "sales".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("marketing42");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "marketing42");
}

#[test]
fn test_doc_engine_marketing42_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "marketing42".to_string(),
        title: "Email Marketing 101 Advanced 42".to_string(),
        content: "How to write subject lines that convert. Specific details for scenario 42.".to_string(),
        category: "Marketing".to_string(),
        tags: vec!["email".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("Email");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "marketing42");
}


#[test]
fn test_doc_engine_inventory43_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "inventory43".to_string(),
        title: "Managing Stock Part 43".to_string(),
        content: "What to do when you run out of product. This covers specific use case 43.".to_string(),
        category: "Inventory".to_string(),
        tags: vec!["stock".to_string(), "warehouse".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("inventory43");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "inventory43");
}

#[test]
fn test_doc_engine_inventory43_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "inventory43".to_string(),
        title: "Managing Stock Advanced 43".to_string(),
        content: "What to do when you run out of product. Specific details for scenario 43.".to_string(),
        category: "Inventory".to_string(),
        tags: vec!["stock".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("Managing");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "inventory43");
}


#[test]
fn test_doc_engine_support44_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "support44".to_string(),
        title: "Handling Angry Customers Part 44".to_string(),
        content: "De-escalation tactics for retail. This covers specific use case 44.".to_string(),
        category: "Support".to_string(),
        tags: vec!["angry".to_string(), "help".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("support44");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "support44");
}

#[test]
fn test_doc_engine_support44_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "support44".to_string(),
        title: "Handling Angry Customers Advanced 44".to_string(),
        content: "De-escalation tactics for retail. Specific details for scenario 44.".to_string(),
        category: "Support".to_string(),
        tags: vec!["angry".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("Handling");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "support44");
}


#[test]
fn test_doc_engine_design45_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "design45".to_string(),
        title: "Customizing CSS Part 45".to_string(),
        content: "How to change your storefront colors. This covers specific use case 45.".to_string(),
        category: "Design".to_string(),
        tags: vec!["css".to_string(), "styling".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("design45");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "design45");
}

#[test]
fn test_doc_engine_design45_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "design45".to_string(),
        title: "Customizing CSS Advanced 45".to_string(),
        content: "How to change your storefront colors. Specific details for scenario 45.".to_string(),
        category: "Design".to_string(),
        tags: vec!["css".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("Customizing");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "design45");
}


#[test]
fn test_doc_engine_api46_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "api46".to_string(),
        title: "Using the GraphQL API Part 46".to_string(),
        content: "A guide to generating access tokens. This covers specific use case 46.".to_string(),
        category: "Engineering".to_string(),
        tags: vec!["graphql".to_string(), "code".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("api46");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "api46");
}

#[test]
fn test_doc_engine_api46_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "api46".to_string(),
        title: "Using the GraphQL API Advanced 46".to_string(),
        content: "A guide to generating access tokens. Specific details for scenario 46.".to_string(),
        category: "Engineering".to_string(),
        tags: vec!["graphql".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("Using");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "api46");
}


#[test]
fn test_doc_engine_shipping47_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "shipping47".to_string(),
        title: "FedEx Integration Part 47".to_string(),
        content: "How to print labels automatically. This covers specific use case 47.".to_string(),
        category: "Shipping".to_string(),
        tags: vec!["fedex".to_string(), "labels".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("shipping47");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "shipping47");
}

#[test]
fn test_doc_engine_shipping47_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "shipping47".to_string(),
        title: "FedEx Integration Advanced 47".to_string(),
        content: "How to print labels automatically. Specific details for scenario 47.".to_string(),
        category: "Shipping".to_string(),
        tags: vec!["fedex".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("FedEx");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "shipping47");
}


#[test]
fn test_doc_engine_security48_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "security48".to_string(),
        title: "Two Factor Auth Part 48".to_string(),
        content: "Securing your account with SMS. This covers specific use case 48.".to_string(),
        category: "Security".to_string(),
        tags: vec!["2fa".to_string(), "sms".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("security48");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "security48");
}

#[test]
fn test_doc_engine_security48_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "security48".to_string(),
        title: "Two Factor Auth Advanced 48".to_string(),
        content: "Securing your account with SMS. Specific details for scenario 48.".to_string(),
        category: "Security".to_string(),
        tags: vec!["2fa".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("Two");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "security48");
}


#[test]
fn test_doc_engine_billing49_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "billing49".to_string(),
        title: "Updating Credit Cards Part 49".to_string(),
        content: "How to change your subscription payment method. This covers specific use case 49.".to_string(),
        category: "Billing".to_string(),
        tags: vec!["credit card".to_string(), "subscription".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("billing49");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "billing49");
}

#[test]
fn test_doc_engine_billing49_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "billing49".to_string(),
        title: "Updating Credit Cards Advanced 49".to_string(),
        content: "How to change your subscription payment method. Specific details for scenario 49.".to_string(),
        category: "Billing".to_string(),
        tags: vec!["credit card".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("Updating");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "billing49");
}


#[test]
fn test_doc_engine_tax50_insertion() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "tax50".to_string(),
        title: "How to File Taxes Part 50".to_string(),
        content: "Understanding schedule C and quarterly estimates. This covers specific use case 50.".to_string(),
        category: "Accounting".to_string(),
        tags: vec!["finance".to_string(), "irs".to_string()],
    };

    engine.add_article(article.clone());

    let retrieved = engine.get_article("tax50");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, "tax50");
}

#[test]
fn test_doc_engine_tax50_search() {
    let mut engine = DocEngine::new();
    let article = Article {
        id: "tax50".to_string(),
        title: "How to File Taxes Advanced 50".to_string(),
        content: "Understanding schedule C and quarterly estimates. Specific details for scenario 50.".to_string(),
        category: "Accounting".to_string(),
        tags: vec!["finance".to_string()],
    };

    engine.add_article(article);

    let results = engine.search("How");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "tax50");
}
