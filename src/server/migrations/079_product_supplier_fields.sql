ALTER TABLE products ADD COLUMN IF NOT EXISTS supplier_name TEXT;
ALTER TABLE products ADD COLUMN IF NOT EXISTS supplier_contact TEXT;
ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS proposed_content TEXT;
