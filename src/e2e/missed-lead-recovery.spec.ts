import { test, expect, adminPage } from './fixtures';
import { db } from './db_utils';

test.describe('Missed Lead Recovery Work Triage UI', () => {
    test.beforeAll(async () => {
        const tenantId = 'e2e-tenant';

        // Ensure standard customers and standard inbound_signals to trigger the work triage
        await db.query(`
            INSERT INTO inbound_signals (id, tenant_id, source, raw_payload, status)
            VALUES ('sig-1', '${tenantId}', 'instagram_dm', '{}', 'PROCESSED')
            ON CONFLICT (id) DO NOTHING
        `);

        await db.query(`
            INSERT INTO daily_work_items (id, tenant_id, signal_id, intent, customer_info, suggested_actions, status)
            VALUES ('work-missed-lead-1', '${tenantId}', 'sig-1', 'missed_lead_recovery',
            '{"name": "E2E Missed Lead User", "message": "Need a plumber ASAP for a leaky pipe."}',
            '{"draft_reply": "Hi E2E Missed Lead User, sorry for the delay! We''re currently reviewing your request and will get back to you shortly. Did you still need help?"}',
            'PENDING')
            ON CONFLICT (id) DO UPDATE SET status = 'PENDING', customer_info = EXCLUDED.customer_info, suggested_actions = EXCLUDED.suggested_actions
        `);
    });

    test('owner can review and take over missed lead recovery items', async ({ page, context }) => {
        const p = await adminPage(page, context);

        // Go to Triage dashboard
        await p.goto('/ui/triage.html?bypass_cache=true');

        // Wait for list to load
        await p.waitForSelector('.app-list-item');

        // Verify the lead is displayed
        const card = p.getByTestId('triage-card-work-missed-lead-1');
        await expect(card).toBeVisible();
        await card.click();

        // Verify detail UI
        await expect(p.locator('#triage-detail').getByText('E2E Missed Lead User', { exact: true })).toBeVisible();
        await expect(p.locator('#triage-detail').getByText('Need a plumber ASAP for a leaky pipe.', { exact: true })).toBeVisible();

        // Verify the generated action message
        const editDraft = p.locator('#edit-draft-reply');
        await expect(editDraft).toHaveValue(/Hi E2E Missed Lead User, sorry for the delay!/);

        // Verify custom action button based on intent
        const approveBtn = p.getByTestId('approve-btn');
        await expect(approveBtn).toHaveText(/✨ Take Over/);

        // Action taken
        const decision = p.waitForResponse(response => response.url().includes('/triage/action') && response.request().method() === 'POST');
        await approveBtn.click();

        // Expected success
        expect((await decision).status()).toBe(200);
        await expect(card).toBeHidden();
        const saved = await db.query('SELECT status FROM daily_work_items WHERE id = $1', ['work-missed-lead-1']);
        expect(saved[0].status).toBe('APPROVED');
    });
});
