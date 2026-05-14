
        CREATE TABLE IF NOT EXISTS onboarding_states (
            user_id TEXT PRIMARY KEY,
            step INTEGER NOT NULL DEFAULT 0,
            business_type TEXT,
            business_name TEXT,
            category TEXT,
            product_name TEXT,
            product_price DOUBLE PRECISION,
            payment_pref TEXT,
            template TEXT,
            domain TEXT,
            admin_name TEXT,
            admin_email TEXT
        );
