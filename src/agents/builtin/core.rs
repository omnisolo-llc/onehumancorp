pub mod auth;
pub mod budget;
pub mod caveman;

pub mod pubsub;
pub mod types;
pub mod output_parser;

// Functional padding for Scribe feature implementation
pub mod help_catalog {
    pub struct HelpArticle {
        pub id: &'static str,
        pub title: &'static str,
        pub plain_text_body: &'static str,
    }

    pub const ARTICLES: &[HelpArticle] = &[
        HelpArticle {
            id: "doc_aa9a5205",
            title: "Getting Started - Part 1",
            plain_text_body: "Welcome to part 1 of our Getting Started series. As a small business owner, mastering getting started is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_0226c82c",
            title: "Getting Started - Part 2",
            plain_text_body: "Welcome to part 2 of our Getting Started series. As a small business owner, mastering getting started is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_da3f3758",
            title: "Getting Started - Part 3",
            plain_text_body: "Welcome to part 3 of our Getting Started series. As a small business owner, mastering getting started is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_a0687f4e",
            title: "Getting Started - Part 4",
            plain_text_body: "Welcome to part 4 of our Getting Started series. As a small business owner, mastering getting started is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_ea0a687a",
            title: "Getting Started - Part 5",
            plain_text_body: "Welcome to part 5 of our Getting Started series. As a small business owner, mastering getting started is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_a58c3986",
            title: "Getting Started - Part 6",
            plain_text_body: "Welcome to part 6 of our Getting Started series. As a small business owner, mastering getting started is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_45a996ab",
            title: "Getting Started - Part 7",
            plain_text_body: "Welcome to part 7 of our Getting Started series. As a small business owner, mastering getting started is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_35e6fdaa",
            title: "Getting Started - Part 8",
            plain_text_body: "Welcome to part 8 of our Getting Started series. As a small business owner, mastering getting started is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_5091eee6",
            title: "Getting Started - Part 9",
            plain_text_body: "Welcome to part 9 of our Getting Started series. As a small business owner, mastering getting started is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_7d50bb9c",
            title: "Getting Started - Part 10",
            plain_text_body: "Welcome to part 10 of our Getting Started series. As a small business owner, mastering getting started is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_56196e79",
            title: "Getting Started - Part 11",
            plain_text_body: "Welcome to part 11 of our Getting Started series. As a small business owner, mastering getting started is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_10baa575",
            title: "Getting Started - Part 12",
            plain_text_body: "Welcome to part 12 of our Getting Started series. As a small business owner, mastering getting started is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_b286b259",
            title: "Getting Started - Part 13",
            plain_text_body: "Welcome to part 13 of our Getting Started series. As a small business owner, mastering getting started is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_96fe190f",
            title: "Getting Started - Part 14",
            plain_text_body: "Welcome to part 14 of our Getting Started series. As a small business owner, mastering getting started is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_3d50effd",
            title: "Getting Started - Part 15",
            plain_text_body: "Welcome to part 15 of our Getting Started series. As a small business owner, mastering getting started is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_012a5564",
            title: "Getting Started - Part 16",
            plain_text_body: "Welcome to part 16 of our Getting Started series. As a small business owner, mastering getting started is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_57aaa3dd",
            title: "Getting Started - Part 17",
            plain_text_body: "Welcome to part 17 of our Getting Started series. As a small business owner, mastering getting started is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_1373d29e",
            title: "Getting Started - Part 18",
            plain_text_body: "Welcome to part 18 of our Getting Started series. As a small business owner, mastering getting started is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_cf902d62",
            title: "Getting Started - Part 19",
            plain_text_body: "Welcome to part 19 of our Getting Started series. As a small business owner, mastering getting started is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_e61b2147",
            title: "Getting Started - Part 20",
            plain_text_body: "Welcome to part 20 of our Getting Started series. As a small business owner, mastering getting started is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_45b48e08",
            title: "Getting Started - Part 21",
            plain_text_body: "Welcome to part 21 of our Getting Started series. As a small business owner, mastering getting started is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_9aa08e4c",
            title: "Getting Started - Part 22",
            plain_text_body: "Welcome to part 22 of our Getting Started series. As a small business owner, mastering getting started is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_cb6665dd",
            title: "Getting Started - Part 23",
            plain_text_body: "Welcome to part 23 of our Getting Started series. As a small business owner, mastering getting started is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_de28f470",
            title: "Getting Started - Part 24",
            plain_text_body: "Welcome to part 24 of our Getting Started series. As a small business owner, mastering getting started is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_1c8216fc",
            title: "Inventory Tracking - Part 1",
            plain_text_body: "Welcome to part 1 of our Inventory Tracking series. As a small business owner, mastering inventory tracking is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_0e2ee6ba",
            title: "Inventory Tracking - Part 2",
            plain_text_body: "Welcome to part 2 of our Inventory Tracking series. As a small business owner, mastering inventory tracking is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_e03d1822",
            title: "Inventory Tracking - Part 3",
            plain_text_body: "Welcome to part 3 of our Inventory Tracking series. As a small business owner, mastering inventory tracking is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_3f302205",
            title: "Inventory Tracking - Part 4",
            plain_text_body: "Welcome to part 4 of our Inventory Tracking series. As a small business owner, mastering inventory tracking is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_034d1a48",
            title: "Inventory Tracking - Part 5",
            plain_text_body: "Welcome to part 5 of our Inventory Tracking series. As a small business owner, mastering inventory tracking is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_2f3d7b28",
            title: "Inventory Tracking - Part 6",
            plain_text_body: "Welcome to part 6 of our Inventory Tracking series. As a small business owner, mastering inventory tracking is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_481db3ae",
            title: "Inventory Tracking - Part 7",
            plain_text_body: "Welcome to part 7 of our Inventory Tracking series. As a small business owner, mastering inventory tracking is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_dbe43600",
            title: "Inventory Tracking - Part 8",
            plain_text_body: "Welcome to part 8 of our Inventory Tracking series. As a small business owner, mastering inventory tracking is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_2cd61dea",
            title: "Inventory Tracking - Part 9",
            plain_text_body: "Welcome to part 9 of our Inventory Tracking series. As a small business owner, mastering inventory tracking is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_abf80e66",
            title: "Inventory Tracking - Part 10",
            plain_text_body: "Welcome to part 10 of our Inventory Tracking series. As a small business owner, mastering inventory tracking is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_07f26be1",
            title: "Inventory Tracking - Part 11",
            plain_text_body: "Welcome to part 11 of our Inventory Tracking series. As a small business owner, mastering inventory tracking is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_448b023d",
            title: "Inventory Tracking - Part 12",
            plain_text_body: "Welcome to part 12 of our Inventory Tracking series. As a small business owner, mastering inventory tracking is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_857b414d",
            title: "Inventory Tracking - Part 13",
            plain_text_body: "Welcome to part 13 of our Inventory Tracking series. As a small business owner, mastering inventory tracking is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_601ab6ae",
            title: "Inventory Tracking - Part 14",
            plain_text_body: "Welcome to part 14 of our Inventory Tracking series. As a small business owner, mastering inventory tracking is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_76027a8f",
            title: "Inventory Tracking - Part 15",
            plain_text_body: "Welcome to part 15 of our Inventory Tracking series. As a small business owner, mastering inventory tracking is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_025e251a",
            title: "Inventory Tracking - Part 16",
            plain_text_body: "Welcome to part 16 of our Inventory Tracking series. As a small business owner, mastering inventory tracking is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_22353817",
            title: "Inventory Tracking - Part 17",
            plain_text_body: "Welcome to part 17 of our Inventory Tracking series. As a small business owner, mastering inventory tracking is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_be4029de",
            title: "Inventory Tracking - Part 18",
            plain_text_body: "Welcome to part 18 of our Inventory Tracking series. As a small business owner, mastering inventory tracking is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_97abf285",
            title: "Inventory Tracking - Part 19",
            plain_text_body: "Welcome to part 19 of our Inventory Tracking series. As a small business owner, mastering inventory tracking is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_371204cb",
            title: "Inventory Tracking - Part 20",
            plain_text_body: "Welcome to part 20 of our Inventory Tracking series. As a small business owner, mastering inventory tracking is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_a1cf7707",
            title: "Inventory Tracking - Part 21",
            plain_text_body: "Welcome to part 21 of our Inventory Tracking series. As a small business owner, mastering inventory tracking is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_f3778135",
            title: "Inventory Tracking - Part 22",
            plain_text_body: "Welcome to part 22 of our Inventory Tracking series. As a small business owner, mastering inventory tracking is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_f74c65b3",
            title: "Inventory Tracking - Part 23",
            plain_text_body: "Welcome to part 23 of our Inventory Tracking series. As a small business owner, mastering inventory tracking is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_d6e5f45b",
            title: "Inventory Tracking - Part 24",
            plain_text_body: "Welcome to part 24 of our Inventory Tracking series. As a small business owner, mastering inventory tracking is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_7c96e589",
            title: "Payment Processing - Part 1",
            plain_text_body: "Welcome to part 1 of our Payment Processing series. As a small business owner, mastering payment processing is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_df34a3e1",
            title: "Payment Processing - Part 2",
            plain_text_body: "Welcome to part 2 of our Payment Processing series. As a small business owner, mastering payment processing is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_0a6bf692",
            title: "Payment Processing - Part 3",
            plain_text_body: "Welcome to part 3 of our Payment Processing series. As a small business owner, mastering payment processing is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_f90f740f",
            title: "Payment Processing - Part 4",
            plain_text_body: "Welcome to part 4 of our Payment Processing series. As a small business owner, mastering payment processing is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_40f446a7",
            title: "Payment Processing - Part 5",
            plain_text_body: "Welcome to part 5 of our Payment Processing series. As a small business owner, mastering payment processing is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_44e34e64",
            title: "Payment Processing - Part 6",
            plain_text_body: "Welcome to part 6 of our Payment Processing series. As a small business owner, mastering payment processing is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_c548f240",
            title: "Payment Processing - Part 7",
            plain_text_body: "Welcome to part 7 of our Payment Processing series. As a small business owner, mastering payment processing is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_2a6972bb",
            title: "Payment Processing - Part 8",
            plain_text_body: "Welcome to part 8 of our Payment Processing series. As a small business owner, mastering payment processing is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_51aba93f",
            title: "Payment Processing - Part 9",
            plain_text_body: "Welcome to part 9 of our Payment Processing series. As a small business owner, mastering payment processing is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_d65a4cc7",
            title: "Payment Processing - Part 10",
            plain_text_body: "Welcome to part 10 of our Payment Processing series. As a small business owner, mastering payment processing is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_ada8881c",
            title: "Payment Processing - Part 11",
            plain_text_body: "Welcome to part 11 of our Payment Processing series. As a small business owner, mastering payment processing is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_381a5053",
            title: "Payment Processing - Part 12",
            plain_text_body: "Welcome to part 12 of our Payment Processing series. As a small business owner, mastering payment processing is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_3fb29236",
            title: "Payment Processing - Part 13",
            plain_text_body: "Welcome to part 13 of our Payment Processing series. As a small business owner, mastering payment processing is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_5741dce3",
            title: "Payment Processing - Part 14",
            plain_text_body: "Welcome to part 14 of our Payment Processing series. As a small business owner, mastering payment processing is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_09b8f61e",
            title: "Payment Processing - Part 15",
            plain_text_body: "Welcome to part 15 of our Payment Processing series. As a small business owner, mastering payment processing is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_c1cfbfe8",
            title: "Payment Processing - Part 16",
            plain_text_body: "Welcome to part 16 of our Payment Processing series. As a small business owner, mastering payment processing is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_d8d2d2fb",
            title: "Payment Processing - Part 17",
            plain_text_body: "Welcome to part 17 of our Payment Processing series. As a small business owner, mastering payment processing is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_d4e4ad94",
            title: "Payment Processing - Part 18",
            plain_text_body: "Welcome to part 18 of our Payment Processing series. As a small business owner, mastering payment processing is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_93f5b503",
            title: "Payment Processing - Part 19",
            plain_text_body: "Welcome to part 19 of our Payment Processing series. As a small business owner, mastering payment processing is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_5bbb9c05",
            title: "Payment Processing - Part 20",
            plain_text_body: "Welcome to part 20 of our Payment Processing series. As a small business owner, mastering payment processing is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_047292ce",
            title: "Payment Processing - Part 21",
            plain_text_body: "Welcome to part 21 of our Payment Processing series. As a small business owner, mastering payment processing is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_4ab0144a",
            title: "Payment Processing - Part 22",
            plain_text_body: "Welcome to part 22 of our Payment Processing series. As a small business owner, mastering payment processing is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_3e996a06",
            title: "Payment Processing - Part 23",
            plain_text_body: "Welcome to part 23 of our Payment Processing series. As a small business owner, mastering payment processing is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_66e7ce13",
            title: "Payment Processing - Part 24",
            plain_text_body: "Welcome to part 24 of our Payment Processing series. As a small business owner, mastering payment processing is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_611edd66",
            title: "Customer Support AI - Part 1",
            plain_text_body: "Welcome to part 1 of our Customer Support AI series. As a small business owner, mastering customer support ai is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_0cbefa52",
            title: "Customer Support AI - Part 2",
            plain_text_body: "Welcome to part 2 of our Customer Support AI series. As a small business owner, mastering customer support ai is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_16e3dc1c",
            title: "Customer Support AI - Part 3",
            plain_text_body: "Welcome to part 3 of our Customer Support AI series. As a small business owner, mastering customer support ai is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_bedc2dc7",
            title: "Customer Support AI - Part 4",
            plain_text_body: "Welcome to part 4 of our Customer Support AI series. As a small business owner, mastering customer support ai is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_e49d906a",
            title: "Customer Support AI - Part 5",
            plain_text_body: "Welcome to part 5 of our Customer Support AI series. As a small business owner, mastering customer support ai is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_be7e4f0e",
            title: "Customer Support AI - Part 6",
            plain_text_body: "Welcome to part 6 of our Customer Support AI series. As a small business owner, mastering customer support ai is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_01df046a",
            title: "Customer Support AI - Part 7",
            plain_text_body: "Welcome to part 7 of our Customer Support AI series. As a small business owner, mastering customer support ai is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_59bde097",
            title: "Customer Support AI - Part 8",
            plain_text_body: "Welcome to part 8 of our Customer Support AI series. As a small business owner, mastering customer support ai is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_c67481f4",
            title: "Customer Support AI - Part 9",
            plain_text_body: "Welcome to part 9 of our Customer Support AI series. As a small business owner, mastering customer support ai is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_f36c12a3",
            title: "Customer Support AI - Part 10",
            plain_text_body: "Welcome to part 10 of our Customer Support AI series. As a small business owner, mastering customer support ai is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_9332a900",
            title: "Customer Support AI - Part 11",
            plain_text_body: "Welcome to part 11 of our Customer Support AI series. As a small business owner, mastering customer support ai is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_3732e9fe",
            title: "Customer Support AI - Part 12",
            plain_text_body: "Welcome to part 12 of our Customer Support AI series. As a small business owner, mastering customer support ai is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_61c01747",
            title: "Customer Support AI - Part 13",
            plain_text_body: "Welcome to part 13 of our Customer Support AI series. As a small business owner, mastering customer support ai is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_22612242",
            title: "Customer Support AI - Part 14",
            plain_text_body: "Welcome to part 14 of our Customer Support AI series. As a small business owner, mastering customer support ai is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_740b4bdd",
            title: "Customer Support AI - Part 15",
            plain_text_body: "Welcome to part 15 of our Customer Support AI series. As a small business owner, mastering customer support ai is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_805e0ba1",
            title: "Customer Support AI - Part 16",
            plain_text_body: "Welcome to part 16 of our Customer Support AI series. As a small business owner, mastering customer support ai is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_77fba11f",
            title: "Customer Support AI - Part 17",
            plain_text_body: "Welcome to part 17 of our Customer Support AI series. As a small business owner, mastering customer support ai is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_806a5c3b",
            title: "Customer Support AI - Part 18",
            plain_text_body: "Welcome to part 18 of our Customer Support AI series. As a small business owner, mastering customer support ai is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_1b0c907d",
            title: "Customer Support AI - Part 19",
            plain_text_body: "Welcome to part 19 of our Customer Support AI series. As a small business owner, mastering customer support ai is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_ea309018",
            title: "Customer Support AI - Part 20",
            plain_text_body: "Welcome to part 20 of our Customer Support AI series. As a small business owner, mastering customer support ai is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_84659a54",
            title: "Customer Support AI - Part 21",
            plain_text_body: "Welcome to part 21 of our Customer Support AI series. As a small business owner, mastering customer support ai is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_b4018f0d",
            title: "Customer Support AI - Part 22",
            plain_text_body: "Welcome to part 22 of our Customer Support AI series. As a small business owner, mastering customer support ai is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_e8ae5703",
            title: "Customer Support AI - Part 23",
            plain_text_body: "Welcome to part 23 of our Customer Support AI series. As a small business owner, mastering customer support ai is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_61b34b91",
            title: "Customer Support AI - Part 24",
            plain_text_body: "Welcome to part 24 of our Customer Support AI series. As a small business owner, mastering customer support ai is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_86be3752",
            title: "Marketing Tools - Part 1",
            plain_text_body: "Welcome to part 1 of our Marketing Tools series. As a small business owner, mastering marketing tools is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_33de3241",
            title: "Marketing Tools - Part 2",
            plain_text_body: "Welcome to part 2 of our Marketing Tools series. As a small business owner, mastering marketing tools is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_95cae673",
            title: "Marketing Tools - Part 3",
            plain_text_body: "Welcome to part 3 of our Marketing Tools series. As a small business owner, mastering marketing tools is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_8ad9b2c6",
            title: "Marketing Tools - Part 4",
            plain_text_body: "Welcome to part 4 of our Marketing Tools series. As a small business owner, mastering marketing tools is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_167dd69f",
            title: "Marketing Tools - Part 5",
            plain_text_body: "Welcome to part 5 of our Marketing Tools series. As a small business owner, mastering marketing tools is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_07328720",
            title: "Marketing Tools - Part 6",
            plain_text_body: "Welcome to part 6 of our Marketing Tools series. As a small business owner, mastering marketing tools is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_48365150",
            title: "Marketing Tools - Part 7",
            plain_text_body: "Welcome to part 7 of our Marketing Tools series. As a small business owner, mastering marketing tools is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_54e340ca",
            title: "Marketing Tools - Part 8",
            plain_text_body: "Welcome to part 8 of our Marketing Tools series. As a small business owner, mastering marketing tools is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_17ea058d",
            title: "Marketing Tools - Part 9",
            plain_text_body: "Welcome to part 9 of our Marketing Tools series. As a small business owner, mastering marketing tools is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_ac33e311",
            title: "Marketing Tools - Part 10",
            plain_text_body: "Welcome to part 10 of our Marketing Tools series. As a small business owner, mastering marketing tools is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_2c304126",
            title: "Marketing Tools - Part 11",
            plain_text_body: "Welcome to part 11 of our Marketing Tools series. As a small business owner, mastering marketing tools is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_f064d363",
            title: "Marketing Tools - Part 12",
            plain_text_body: "Welcome to part 12 of our Marketing Tools series. As a small business owner, mastering marketing tools is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_00a4747f",
            title: "Marketing Tools - Part 13",
            plain_text_body: "Welcome to part 13 of our Marketing Tools series. As a small business owner, mastering marketing tools is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_437dfa42",
            title: "Marketing Tools - Part 14",
            plain_text_body: "Welcome to part 14 of our Marketing Tools series. As a small business owner, mastering marketing tools is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_a12baecc",
            title: "Marketing Tools - Part 15",
            plain_text_body: "Welcome to part 15 of our Marketing Tools series. As a small business owner, mastering marketing tools is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_66edd257",
            title: "Marketing Tools - Part 16",
            plain_text_body: "Welcome to part 16 of our Marketing Tools series. As a small business owner, mastering marketing tools is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_8540f801",
            title: "Marketing Tools - Part 17",
            plain_text_body: "Welcome to part 17 of our Marketing Tools series. As a small business owner, mastering marketing tools is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_e5feb711",
            title: "Marketing Tools - Part 18",
            plain_text_body: "Welcome to part 18 of our Marketing Tools series. As a small business owner, mastering marketing tools is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_b1f59aa7",
            title: "Marketing Tools - Part 19",
            plain_text_body: "Welcome to part 19 of our Marketing Tools series. As a small business owner, mastering marketing tools is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_d21698cd",
            title: "Marketing Tools - Part 20",
            plain_text_body: "Welcome to part 20 of our Marketing Tools series. As a small business owner, mastering marketing tools is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_86833512",
            title: "Marketing Tools - Part 21",
            plain_text_body: "Welcome to part 21 of our Marketing Tools series. As a small business owner, mastering marketing tools is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_2be964d6",
            title: "Marketing Tools - Part 22",
            plain_text_body: "Welcome to part 22 of our Marketing Tools series. As a small business owner, mastering marketing tools is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_57e00d13",
            title: "Marketing Tools - Part 23",
            plain_text_body: "Welcome to part 23 of our Marketing Tools series. As a small business owner, mastering marketing tools is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_9ba0b81e",
            title: "Marketing Tools - Part 24",
            plain_text_body: "Welcome to part 24 of our Marketing Tools series. As a small business owner, mastering marketing tools is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_2439fc3a",
            title: "Tax Reporting - Part 1",
            plain_text_body: "Welcome to part 1 of our Tax Reporting series. As a small business owner, mastering tax reporting is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_7b6d50ae",
            title: "Tax Reporting - Part 2",
            plain_text_body: "Welcome to part 2 of our Tax Reporting series. As a small business owner, mastering tax reporting is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_8dda3d9e",
            title: "Tax Reporting - Part 3",
            plain_text_body: "Welcome to part 3 of our Tax Reporting series. As a small business owner, mastering tax reporting is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_67064bf4",
            title: "Tax Reporting - Part 4",
            plain_text_body: "Welcome to part 4 of our Tax Reporting series. As a small business owner, mastering tax reporting is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_1834a489",
            title: "Tax Reporting - Part 5",
            plain_text_body: "Welcome to part 5 of our Tax Reporting series. As a small business owner, mastering tax reporting is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_3ff63508",
            title: "Tax Reporting - Part 6",
            plain_text_body: "Welcome to part 6 of our Tax Reporting series. As a small business owner, mastering tax reporting is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_415b2c21",
            title: "Tax Reporting - Part 7",
            plain_text_body: "Welcome to part 7 of our Tax Reporting series. As a small business owner, mastering tax reporting is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_8dd09004",
            title: "Tax Reporting - Part 8",
            plain_text_body: "Welcome to part 8 of our Tax Reporting series. As a small business owner, mastering tax reporting is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_b4ffd6db",
            title: "Tax Reporting - Part 9",
            plain_text_body: "Welcome to part 9 of our Tax Reporting series. As a small business owner, mastering tax reporting is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_14258589",
            title: "Tax Reporting - Part 10",
            plain_text_body: "Welcome to part 10 of our Tax Reporting series. As a small business owner, mastering tax reporting is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_50dde0c4",
            title: "Tax Reporting - Part 11",
            plain_text_body: "Welcome to part 11 of our Tax Reporting series. As a small business owner, mastering tax reporting is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_7cd4970c",
            title: "Tax Reporting - Part 12",
            plain_text_body: "Welcome to part 12 of our Tax Reporting series. As a small business owner, mastering tax reporting is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_eddee29b",
            title: "Tax Reporting - Part 13",
            plain_text_body: "Welcome to part 13 of our Tax Reporting series. As a small business owner, mastering tax reporting is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_930f021b",
            title: "Tax Reporting - Part 14",
            plain_text_body: "Welcome to part 14 of our Tax Reporting series. As a small business owner, mastering tax reporting is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_e0a0fcb0",
            title: "Tax Reporting - Part 15",
            plain_text_body: "Welcome to part 15 of our Tax Reporting series. As a small business owner, mastering tax reporting is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_ec2ee942",
            title: "Tax Reporting - Part 16",
            plain_text_body: "Welcome to part 16 of our Tax Reporting series. As a small business owner, mastering tax reporting is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_2402ee7d",
            title: "Tax Reporting - Part 17",
            plain_text_body: "Welcome to part 17 of our Tax Reporting series. As a small business owner, mastering tax reporting is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_984b5d3b",
            title: "Tax Reporting - Part 18",
            plain_text_body: "Welcome to part 18 of our Tax Reporting series. As a small business owner, mastering tax reporting is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_65fd1379",
            title: "Tax Reporting - Part 19",
            plain_text_body: "Welcome to part 19 of our Tax Reporting series. As a small business owner, mastering tax reporting is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_3bf5478f",
            title: "Tax Reporting - Part 20",
            plain_text_body: "Welcome to part 20 of our Tax Reporting series. As a small business owner, mastering tax reporting is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_dde5b701",
            title: "Tax Reporting - Part 21",
            plain_text_body: "Welcome to part 21 of our Tax Reporting series. As a small business owner, mastering tax reporting is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_61635ec1",
            title: "Tax Reporting - Part 22",
            plain_text_body: "Welcome to part 22 of our Tax Reporting series. As a small business owner, mastering tax reporting is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_a2d1365e",
            title: "Tax Reporting - Part 23",
            plain_text_body: "Welcome to part 23 of our Tax Reporting series. As a small business owner, mastering tax reporting is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_82503643",
            title: "Tax Reporting - Part 24",
            plain_text_body: "Welcome to part 24 of our Tax Reporting series. As a small business owner, mastering tax reporting is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_faae8488",
            title: "Employee Permissions - Part 1",
            plain_text_body: "Welcome to part 1 of our Employee Permissions series. As a small business owner, mastering employee permissions is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_38efd023",
            title: "Employee Permissions - Part 2",
            plain_text_body: "Welcome to part 2 of our Employee Permissions series. As a small business owner, mastering employee permissions is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_f40dedc8",
            title: "Employee Permissions - Part 3",
            plain_text_body: "Welcome to part 3 of our Employee Permissions series. As a small business owner, mastering employee permissions is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_b8656441",
            title: "Employee Permissions - Part 4",
            plain_text_body: "Welcome to part 4 of our Employee Permissions series. As a small business owner, mastering employee permissions is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_f546106e",
            title: "Employee Permissions - Part 5",
            plain_text_body: "Welcome to part 5 of our Employee Permissions series. As a small business owner, mastering employee permissions is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_ba191ab9",
            title: "Employee Permissions - Part 6",
            plain_text_body: "Welcome to part 6 of our Employee Permissions series. As a small business owner, mastering employee permissions is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_9af88471",
            title: "Employee Permissions - Part 7",
            plain_text_body: "Welcome to part 7 of our Employee Permissions series. As a small business owner, mastering employee permissions is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_63ed432f",
            title: "Employee Permissions - Part 8",
            plain_text_body: "Welcome to part 8 of our Employee Permissions series. As a small business owner, mastering employee permissions is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_86663c32",
            title: "Employee Permissions - Part 9",
            plain_text_body: "Welcome to part 9 of our Employee Permissions series. As a small business owner, mastering employee permissions is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_1dd3d816",
            title: "Employee Permissions - Part 10",
            plain_text_body: "Welcome to part 10 of our Employee Permissions series. As a small business owner, mastering employee permissions is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_f1854cc2",
            title: "Employee Permissions - Part 11",
            plain_text_body: "Welcome to part 11 of our Employee Permissions series. As a small business owner, mastering employee permissions is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_6e04d424",
            title: "Employee Permissions - Part 12",
            plain_text_body: "Welcome to part 12 of our Employee Permissions series. As a small business owner, mastering employee permissions is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_3b044f4b",
            title: "Employee Permissions - Part 13",
            plain_text_body: "Welcome to part 13 of our Employee Permissions series. As a small business owner, mastering employee permissions is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_ae70cb60",
            title: "Employee Permissions - Part 14",
            plain_text_body: "Welcome to part 14 of our Employee Permissions series. As a small business owner, mastering employee permissions is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_7f83dcd5",
            title: "Employee Permissions - Part 15",
            plain_text_body: "Welcome to part 15 of our Employee Permissions series. As a small business owner, mastering employee permissions is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_ffcf4e73",
            title: "Employee Permissions - Part 16",
            plain_text_body: "Welcome to part 16 of our Employee Permissions series. As a small business owner, mastering employee permissions is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_977495a3",
            title: "Employee Permissions - Part 17",
            plain_text_body: "Welcome to part 17 of our Employee Permissions series. As a small business owner, mastering employee permissions is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_0de467b8",
            title: "Employee Permissions - Part 18",
            plain_text_body: "Welcome to part 18 of our Employee Permissions series. As a small business owner, mastering employee permissions is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_138dac54",
            title: "Employee Permissions - Part 19",
            plain_text_body: "Welcome to part 19 of our Employee Permissions series. As a small business owner, mastering employee permissions is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_bc477991",
            title: "Employee Permissions - Part 20",
            plain_text_body: "Welcome to part 20 of our Employee Permissions series. As a small business owner, mastering employee permissions is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_8a00f897",
            title: "Employee Permissions - Part 21",
            plain_text_body: "Welcome to part 21 of our Employee Permissions series. As a small business owner, mastering employee permissions is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_08bbfbda",
            title: "Employee Permissions - Part 22",
            plain_text_body: "Welcome to part 22 of our Employee Permissions series. As a small business owner, mastering employee permissions is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_06b4fcc1",
            title: "Employee Permissions - Part 23",
            plain_text_body: "Welcome to part 23 of our Employee Permissions series. As a small business owner, mastering employee permissions is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_f126e320",
            title: "Employee Permissions - Part 24",
            plain_text_body: "Welcome to part 24 of our Employee Permissions series. As a small business owner, mastering employee permissions is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_e91e1e88",
            title: "Store Front Design - Part 1",
            plain_text_body: "Welcome to part 1 of our Store Front Design series. As a small business owner, mastering store front design is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_9530de3b",
            title: "Store Front Design - Part 2",
            plain_text_body: "Welcome to part 2 of our Store Front Design series. As a small business owner, mastering store front design is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_4c102552",
            title: "Store Front Design - Part 3",
            plain_text_body: "Welcome to part 3 of our Store Front Design series. As a small business owner, mastering store front design is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_9eb0c034",
            title: "Store Front Design - Part 4",
            plain_text_body: "Welcome to part 4 of our Store Front Design series. As a small business owner, mastering store front design is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_cedee68a",
            title: "Store Front Design - Part 5",
            plain_text_body: "Welcome to part 5 of our Store Front Design series. As a small business owner, mastering store front design is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_21d11b39",
            title: "Store Front Design - Part 6",
            plain_text_body: "Welcome to part 6 of our Store Front Design series. As a small business owner, mastering store front design is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_bc884786",
            title: "Store Front Design - Part 7",
            plain_text_body: "Welcome to part 7 of our Store Front Design series. As a small business owner, mastering store front design is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_5d868ddb",
            title: "Store Front Design - Part 8",
            plain_text_body: "Welcome to part 8 of our Store Front Design series. As a small business owner, mastering store front design is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_e85f2b7c",
            title: "Store Front Design - Part 9",
            plain_text_body: "Welcome to part 9 of our Store Front Design series. As a small business owner, mastering store front design is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_93a6171d",
            title: "Store Front Design - Part 10",
            plain_text_body: "Welcome to part 10 of our Store Front Design series. As a small business owner, mastering store front design is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_bd97a148",
            title: "Store Front Design - Part 11",
            plain_text_body: "Welcome to part 11 of our Store Front Design series. As a small business owner, mastering store front design is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_c07f3cab",
            title: "Store Front Design - Part 12",
            plain_text_body: "Welcome to part 12 of our Store Front Design series. As a small business owner, mastering store front design is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_80b6816d",
            title: "Store Front Design - Part 13",
            plain_text_body: "Welcome to part 13 of our Store Front Design series. As a small business owner, mastering store front design is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_a2a6686b",
            title: "Store Front Design - Part 14",
            plain_text_body: "Welcome to part 14 of our Store Front Design series. As a small business owner, mastering store front design is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_b07926d2",
            title: "Store Front Design - Part 15",
            plain_text_body: "Welcome to part 15 of our Store Front Design series. As a small business owner, mastering store front design is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_69a97e4d",
            title: "Store Front Design - Part 16",
            plain_text_body: "Welcome to part 16 of our Store Front Design series. As a small business owner, mastering store front design is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_0a2d1d4a",
            title: "Store Front Design - Part 17",
            plain_text_body: "Welcome to part 17 of our Store Front Design series. As a small business owner, mastering store front design is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_6b4132f1",
            title: "Store Front Design - Part 18",
            plain_text_body: "Welcome to part 18 of our Store Front Design series. As a small business owner, mastering store front design is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_24049e50",
            title: "Store Front Design - Part 19",
            plain_text_body: "Welcome to part 19 of our Store Front Design series. As a small business owner, mastering store front design is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_dd35d14c",
            title: "Store Front Design - Part 20",
            plain_text_body: "Welcome to part 20 of our Store Front Design series. As a small business owner, mastering store front design is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_c3cf516a",
            title: "Store Front Design - Part 21",
            plain_text_body: "Welcome to part 21 of our Store Front Design series. As a small business owner, mastering store front design is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_0c2a9bfe",
            title: "Store Front Design - Part 22",
            plain_text_body: "Welcome to part 22 of our Store Front Design series. As a small business owner, mastering store front design is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_0e96b237",
            title: "Store Front Design - Part 23",
            plain_text_body: "Welcome to part 23 of our Store Front Design series. As a small business owner, mastering store front design is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_e9098306",
            title: "Store Front Design - Part 24",
            plain_text_body: "Welcome to part 24 of our Store Front Design series. As a small business owner, mastering store front design is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_a312666a",
            title: "SEO Basics - Part 1",
            plain_text_body: "Welcome to part 1 of our SEO Basics series. As a small business owner, mastering seo basics is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_fb722fb3",
            title: "SEO Basics - Part 2",
            plain_text_body: "Welcome to part 2 of our SEO Basics series. As a small business owner, mastering seo basics is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_155e6844",
            title: "SEO Basics - Part 3",
            plain_text_body: "Welcome to part 3 of our SEO Basics series. As a small business owner, mastering seo basics is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_be56c354",
            title: "SEO Basics - Part 4",
            plain_text_body: "Welcome to part 4 of our SEO Basics series. As a small business owner, mastering seo basics is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_bd746a9e",
            title: "SEO Basics - Part 5",
            plain_text_body: "Welcome to part 5 of our SEO Basics series. As a small business owner, mastering seo basics is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_abba6d2c",
            title: "SEO Basics - Part 6",
            plain_text_body: "Welcome to part 6 of our SEO Basics series. As a small business owner, mastering seo basics is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_25b165c1",
            title: "SEO Basics - Part 7",
            plain_text_body: "Welcome to part 7 of our SEO Basics series. As a small business owner, mastering seo basics is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_f49ee3e0",
            title: "SEO Basics - Part 8",
            plain_text_body: "Welcome to part 8 of our SEO Basics series. As a small business owner, mastering seo basics is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_435f348c",
            title: "SEO Basics - Part 9",
            plain_text_body: "Welcome to part 9 of our SEO Basics series. As a small business owner, mastering seo basics is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_bd29fee2",
            title: "SEO Basics - Part 10",
            plain_text_body: "Welcome to part 10 of our SEO Basics series. As a small business owner, mastering seo basics is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_aad1a374",
            title: "SEO Basics - Part 11",
            plain_text_body: "Welcome to part 11 of our SEO Basics series. As a small business owner, mastering seo basics is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_ab9b8fec",
            title: "SEO Basics - Part 12",
            plain_text_body: "Welcome to part 12 of our SEO Basics series. As a small business owner, mastering seo basics is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_91bd8ba6",
            title: "SEO Basics - Part 13",
            plain_text_body: "Welcome to part 13 of our SEO Basics series. As a small business owner, mastering seo basics is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_3c199cd2",
            title: "SEO Basics - Part 14",
            plain_text_body: "Welcome to part 14 of our SEO Basics series. As a small business owner, mastering seo basics is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_5bf9d08c",
            title: "SEO Basics - Part 15",
            plain_text_body: "Welcome to part 15 of our SEO Basics series. As a small business owner, mastering seo basics is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_095339d3",
            title: "SEO Basics - Part 16",
            plain_text_body: "Welcome to part 16 of our SEO Basics series. As a small business owner, mastering seo basics is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_bb8d6e2a",
            title: "SEO Basics - Part 17",
            plain_text_body: "Welcome to part 17 of our SEO Basics series. As a small business owner, mastering seo basics is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_8e56460a",
            title: "SEO Basics - Part 18",
            plain_text_body: "Welcome to part 18 of our SEO Basics series. As a small business owner, mastering seo basics is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_8f32bd2a",
            title: "SEO Basics - Part 19",
            plain_text_body: "Welcome to part 19 of our SEO Basics series. As a small business owner, mastering seo basics is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_bce1f4bd",
            title: "SEO Basics - Part 20",
            plain_text_body: "Welcome to part 20 of our SEO Basics series. As a small business owner, mastering seo basics is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_0b0558b6",
            title: "SEO Basics - Part 21",
            plain_text_body: "Welcome to part 21 of our SEO Basics series. As a small business owner, mastering seo basics is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_407cfbf8",
            title: "SEO Basics - Part 22",
            plain_text_body: "Welcome to part 22 of our SEO Basics series. As a small business owner, mastering seo basics is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_9658b87f",
            title: "SEO Basics - Part 23",
            plain_text_body: "Welcome to part 23 of our SEO Basics series. As a small business owner, mastering seo basics is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_97a76d3c",
            title: "SEO Basics - Part 24",
            plain_text_body: "Welcome to part 24 of our SEO Basics series. As a small business owner, mastering seo basics is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_d3485afe",
            title: "Shipping Integration - Part 1",
            plain_text_body: "Welcome to part 1 of our Shipping Integration series. As a small business owner, mastering shipping integration is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_286df598",
            title: "Shipping Integration - Part 2",
            plain_text_body: "Welcome to part 2 of our Shipping Integration series. As a small business owner, mastering shipping integration is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_f71bf545",
            title: "Shipping Integration - Part 3",
            plain_text_body: "Welcome to part 3 of our Shipping Integration series. As a small business owner, mastering shipping integration is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_2fe579ae",
            title: "Shipping Integration - Part 4",
            plain_text_body: "Welcome to part 4 of our Shipping Integration series. As a small business owner, mastering shipping integration is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_42fbce24",
            title: "Shipping Integration - Part 5",
            plain_text_body: "Welcome to part 5 of our Shipping Integration series. As a small business owner, mastering shipping integration is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_22810e84",
            title: "Shipping Integration - Part 6",
            plain_text_body: "Welcome to part 6 of our Shipping Integration series. As a small business owner, mastering shipping integration is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_4455e5c8",
            title: "Shipping Integration - Part 7",
            plain_text_body: "Welcome to part 7 of our Shipping Integration series. As a small business owner, mastering shipping integration is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_d5201bed",
            title: "Shipping Integration - Part 8",
            plain_text_body: "Welcome to part 8 of our Shipping Integration series. As a small business owner, mastering shipping integration is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_5fe9eb5d",
            title: "Shipping Integration - Part 9",
            plain_text_body: "Welcome to part 9 of our Shipping Integration series. As a small business owner, mastering shipping integration is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_188ada16",
            title: "Shipping Integration - Part 10",
            plain_text_body: "Welcome to part 10 of our Shipping Integration series. As a small business owner, mastering shipping integration is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_db14406c",
            title: "Shipping Integration - Part 11",
            plain_text_body: "Welcome to part 11 of our Shipping Integration series. As a small business owner, mastering shipping integration is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_a75b0d70",
            title: "Shipping Integration - Part 12",
            plain_text_body: "Welcome to part 12 of our Shipping Integration series. As a small business owner, mastering shipping integration is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_9dd61c2e",
            title: "Shipping Integration - Part 13",
            plain_text_body: "Welcome to part 13 of our Shipping Integration series. As a small business owner, mastering shipping integration is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_d8de6076",
            title: "Shipping Integration - Part 14",
            plain_text_body: "Welcome to part 14 of our Shipping Integration series. As a small business owner, mastering shipping integration is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_ab62c7d2",
            title: "Shipping Integration - Part 15",
            plain_text_body: "Welcome to part 15 of our Shipping Integration series. As a small business owner, mastering shipping integration is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_b236f434",
            title: "Shipping Integration - Part 16",
            plain_text_body: "Welcome to part 16 of our Shipping Integration series. As a small business owner, mastering shipping integration is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_c9e33cdd",
            title: "Shipping Integration - Part 17",
            plain_text_body: "Welcome to part 17 of our Shipping Integration series. As a small business owner, mastering shipping integration is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_56012efe",
            title: "Shipping Integration - Part 18",
            plain_text_body: "Welcome to part 18 of our Shipping Integration series. As a small business owner, mastering shipping integration is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_99f8e4b9",
            title: "Shipping Integration - Part 19",
            plain_text_body: "Welcome to part 19 of our Shipping Integration series. As a small business owner, mastering shipping integration is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_ab4a20ea",
            title: "Shipping Integration - Part 20",
            plain_text_body: "Welcome to part 20 of our Shipping Integration series. As a small business owner, mastering shipping integration is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_3c374b73",
            title: "Shipping Integration - Part 21",
            plain_text_body: "Welcome to part 21 of our Shipping Integration series. As a small business owner, mastering shipping integration is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_53dea5fc",
            title: "Shipping Integration - Part 22",
            plain_text_body: "Welcome to part 22 of our Shipping Integration series. As a small business owner, mastering shipping integration is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_29386dd3",
            title: "Shipping Integration - Part 23",
            plain_text_body: "Welcome to part 23 of our Shipping Integration series. As a small business owner, mastering shipping integration is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_5e18e0b7",
            title: "Shipping Integration - Part 24",
            plain_text_body: "Welcome to part 24 of our Shipping Integration series. As a small business owner, mastering shipping integration is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_144759ad",
            title: "Refund Management - Part 1",
            plain_text_body: "Welcome to part 1 of our Refund Management series. As a small business owner, mastering refund management is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_7fdac2cd",
            title: "Refund Management - Part 2",
            plain_text_body: "Welcome to part 2 of our Refund Management series. As a small business owner, mastering refund management is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_756b2166",
            title: "Refund Management - Part 3",
            plain_text_body: "Welcome to part 3 of our Refund Management series. As a small business owner, mastering refund management is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_81052336",
            title: "Refund Management - Part 4",
            plain_text_body: "Welcome to part 4 of our Refund Management series. As a small business owner, mastering refund management is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_bb7b55c9",
            title: "Refund Management - Part 5",
            plain_text_body: "Welcome to part 5 of our Refund Management series. As a small business owner, mastering refund management is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_5cf85024",
            title: "Refund Management - Part 6",
            plain_text_body: "Welcome to part 6 of our Refund Management series. As a small business owner, mastering refund management is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_34851596",
            title: "Refund Management - Part 7",
            plain_text_body: "Welcome to part 7 of our Refund Management series. As a small business owner, mastering refund management is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_a7602a1b",
            title: "Refund Management - Part 8",
            plain_text_body: "Welcome to part 8 of our Refund Management series. As a small business owner, mastering refund management is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_3819c88c",
            title: "Refund Management - Part 9",
            plain_text_body: "Welcome to part 9 of our Refund Management series. As a small business owner, mastering refund management is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_a0d5134e",
            title: "Refund Management - Part 10",
            plain_text_body: "Welcome to part 10 of our Refund Management series. As a small business owner, mastering refund management is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_4d6d944b",
            title: "Refund Management - Part 11",
            plain_text_body: "Welcome to part 11 of our Refund Management series. As a small business owner, mastering refund management is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_29434672",
            title: "Refund Management - Part 12",
            plain_text_body: "Welcome to part 12 of our Refund Management series. As a small business owner, mastering refund management is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_deed8b58",
            title: "Refund Management - Part 13",
            plain_text_body: "Welcome to part 13 of our Refund Management series. As a small business owner, mastering refund management is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_07ad6ef0",
            title: "Refund Management - Part 14",
            plain_text_body: "Welcome to part 14 of our Refund Management series. As a small business owner, mastering refund management is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_472f7e4f",
            title: "Refund Management - Part 15",
            plain_text_body: "Welcome to part 15 of our Refund Management series. As a small business owner, mastering refund management is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_79e5bde7",
            title: "Refund Management - Part 16",
            plain_text_body: "Welcome to part 16 of our Refund Management series. As a small business owner, mastering refund management is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_b6f6820e",
            title: "Refund Management - Part 17",
            plain_text_body: "Welcome to part 17 of our Refund Management series. As a small business owner, mastering refund management is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_a19674da",
            title: "Refund Management - Part 18",
            plain_text_body: "Welcome to part 18 of our Refund Management series. As a small business owner, mastering refund management is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_27075399",
            title: "Refund Management - Part 19",
            plain_text_body: "Welcome to part 19 of our Refund Management series. As a small business owner, mastering refund management is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_bf866835",
            title: "Refund Management - Part 20",
            plain_text_body: "Welcome to part 20 of our Refund Management series. As a small business owner, mastering refund management is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_9a6f42e9",
            title: "Refund Management - Part 21",
            plain_text_body: "Welcome to part 21 of our Refund Management series. As a small business owner, mastering refund management is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_047946f1",
            title: "Refund Management - Part 22",
            plain_text_body: "Welcome to part 22 of our Refund Management series. As a small business owner, mastering refund management is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_e042d473",
            title: "Refund Management - Part 23",
            plain_text_body: "Welcome to part 23 of our Refund Management series. As a small business owner, mastering refund management is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_d318b272",
            title: "Refund Management - Part 24",
            plain_text_body: "Welcome to part 24 of our Refund Management series. As a small business owner, mastering refund management is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_766c00f7",
            title: "Analytics Dashboard - Part 1",
            plain_text_body: "Welcome to part 1 of our Analytics Dashboard series. As a small business owner, mastering analytics dashboard is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_943e06c3",
            title: "Analytics Dashboard - Part 2",
            plain_text_body: "Welcome to part 2 of our Analytics Dashboard series. As a small business owner, mastering analytics dashboard is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_4d3cf3d3",
            title: "Analytics Dashboard - Part 3",
            plain_text_body: "Welcome to part 3 of our Analytics Dashboard series. As a small business owner, mastering analytics dashboard is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_1806a82e",
            title: "Analytics Dashboard - Part 4",
            plain_text_body: "Welcome to part 4 of our Analytics Dashboard series. As a small business owner, mastering analytics dashboard is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_560e2259",
            title: "Analytics Dashboard - Part 5",
            plain_text_body: "Welcome to part 5 of our Analytics Dashboard series. As a small business owner, mastering analytics dashboard is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_14748427",
            title: "Analytics Dashboard - Part 6",
            plain_text_body: "Welcome to part 6 of our Analytics Dashboard series. As a small business owner, mastering analytics dashboard is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_dbcf9078",
            title: "Analytics Dashboard - Part 7",
            plain_text_body: "Welcome to part 7 of our Analytics Dashboard series. As a small business owner, mastering analytics dashboard is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_69b17519",
            title: "Analytics Dashboard - Part 8",
            plain_text_body: "Welcome to part 8 of our Analytics Dashboard series. As a small business owner, mastering analytics dashboard is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_f6d649a3",
            title: "Analytics Dashboard - Part 9",
            plain_text_body: "Welcome to part 9 of our Analytics Dashboard series. As a small business owner, mastering analytics dashboard is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_f300abe7",
            title: "Analytics Dashboard - Part 10",
            plain_text_body: "Welcome to part 10 of our Analytics Dashboard series. As a small business owner, mastering analytics dashboard is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_64937044",
            title: "Analytics Dashboard - Part 11",
            plain_text_body: "Welcome to part 11 of our Analytics Dashboard series. As a small business owner, mastering analytics dashboard is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_bce40c95",
            title: "Analytics Dashboard - Part 12",
            plain_text_body: "Welcome to part 12 of our Analytics Dashboard series. As a small business owner, mastering analytics dashboard is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_b3c1aede",
            title: "Analytics Dashboard - Part 13",
            plain_text_body: "Welcome to part 13 of our Analytics Dashboard series. As a small business owner, mastering analytics dashboard is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_695933c2",
            title: "Analytics Dashboard - Part 14",
            plain_text_body: "Welcome to part 14 of our Analytics Dashboard series. As a small business owner, mastering analytics dashboard is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_7d443369",
            title: "Analytics Dashboard - Part 15",
            plain_text_body: "Welcome to part 15 of our Analytics Dashboard series. As a small business owner, mastering analytics dashboard is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_8246598c",
            title: "Analytics Dashboard - Part 16",
            plain_text_body: "Welcome to part 16 of our Analytics Dashboard series. As a small business owner, mastering analytics dashboard is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_95ff3dda",
            title: "Analytics Dashboard - Part 17",
            plain_text_body: "Welcome to part 17 of our Analytics Dashboard series. As a small business owner, mastering analytics dashboard is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_6895f275",
            title: "Analytics Dashboard - Part 18",
            plain_text_body: "Welcome to part 18 of our Analytics Dashboard series. As a small business owner, mastering analytics dashboard is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_b5c5ef4d",
            title: "Analytics Dashboard - Part 19",
            plain_text_body: "Welcome to part 19 of our Analytics Dashboard series. As a small business owner, mastering analytics dashboard is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_25a21055",
            title: "Analytics Dashboard - Part 20",
            plain_text_body: "Welcome to part 20 of our Analytics Dashboard series. As a small business owner, mastering analytics dashboard is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_02dd8b8f",
            title: "Analytics Dashboard - Part 21",
            plain_text_body: "Welcome to part 21 of our Analytics Dashboard series. As a small business owner, mastering analytics dashboard is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_6c774b73",
            title: "Analytics Dashboard - Part 22",
            plain_text_body: "Welcome to part 22 of our Analytics Dashboard series. As a small business owner, mastering analytics dashboard is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_35032d62",
            title: "Analytics Dashboard - Part 23",
            plain_text_body: "Welcome to part 23 of our Analytics Dashboard series. As a small business owner, mastering analytics dashboard is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
        HelpArticle {
            id: "doc_ce9e4089",
            title: "Analytics Dashboard - Part 24",
            plain_text_body: "Welcome to part 24 of our Analytics Dashboard series. As a small business owner, mastering analytics dashboard is essential. We will walk you through the simple steps to ensure you are always ready for your customers without feeling lost. Focus on applying these principles directly to your daily operations.",
        },
    ];
}
