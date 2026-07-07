-- Wait for tables to be available before seeding
DO $$
BEGIN
    -- Seed data for testing staff management
    INSERT INTO shift_summaries (id, tenant_id, shift_date, summary_text, escalations, supply_needs)
    VALUES ('sum_test_1', 'test_tenant', '2024-05-20', 'Shift Performance: 15 tasks completed, 2 escalations. Overall smooth operation.', 'Low on Cups', 'Cups, Napkins')
    ON CONFLICT DO NOTHING;

    INSERT INTO ohc_location_escalation (id, tenant_id, staff_id, escalation_text, status)
    VALUES ('esc_test_1', 'test_tenant', 'staff_1', 'Bathroom needs cleaning urgently', 'pending')
    ON CONFLICT DO NOTHING;
END $$;
