import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';
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
            ON CONFLICT (id) DO NOTHING
        `);
    });

    test('owner can review and take over missed lead recovery items', async ({ page }) => {
        // Go to Triage dashboard
        await adminPage.goto('/ui/triage.html');

        // Wait for list to load
        await adminPage.waitForSelector('.app-list-item');

        // Verify the lead is displayed
        const card = adminPage.getByTestId('triage-card-work-missed-lead-1');
        await expect(card).toBeVisible();
        await card.click();

        // Verify detail UI
        await expect(adminPage.locator('text=E2E Missed Lead User')).toBeVisible();
        await expect(adminPage.locator('text=Need a plumber ASAP for a leaky pipe.')).toBeVisible();

        // Verify the generated action message
        const editDraft = adminPage.locator('#edit-draft-reply');
        await expect(editDraft).toHaveValue(/Hi E2E Missed Lead User, sorry for the delay!/);

        // Verify custom action button based on intent
        const approveBtn = adminPage.getByTestId('approve-btn');
        await expect(approveBtn).toHaveText(/✨ Take Over/);

        // Action taken
        await approveBtn.click();

        // Expected success
        await expect(adminPage.locator('.action-status.success')).toBeVisible({ timeout: 5000 });
        await expect(adminPage.locator('.action-status.success')).toHaveText('Approved!');
    });
});
