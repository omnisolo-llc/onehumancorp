## Missing RLS Policies Found in Database Migrations

During a full repository sweep for unused code and database isolation hygiene, we identified several tables created in migrations that were missing explicit `ENABLE ROW LEVEL SECURITY` statements or `CREATE POLICY` statements to enforce tenant isolation.

### Tables missing RLS:
- `agent_actions` (Created in `074_missing_tables.sql`)
- `ai_memories` (Created in `008_data_model_architecture.sql` - index created, but RLS commented out)
- `bom_items` (Created in `022_supply_chain.sql`)
- `customer_timeline` (Created in `074_missing_tables.sql`)
- `depletion_logs` (Created in `022_supply_chain.sql`)
- `interactions` (Created in `074_missing_tables.sql`)
- `order_line_items` (Created in `008_data_model_architecture.sql`)
- `po_line_items` (Created in `022_supply_chain.sql`)
- `raw_materials` (Created in `022_supply_chain.sql`)
- `services` (Created in `008_data_model_architecture.sql`)

### Actions Taken:
We have created a new migration (`077_enforce_remaining_rls.sql`) to explicitly enable RLS and add standard `tenant_id` policies for these tables.
