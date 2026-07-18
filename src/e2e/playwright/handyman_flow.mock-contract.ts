
import { test, expect } from '@playwright/test';

test.describe('Agentic Field Service Scheduling & Quoting', () => {
    test('CUJ: Handyman quote draft and tentative booking flow', async ({ page, request }) => {
        // Step 1: Simulate the incoming customer lead
        const leadPayload = {
            message: "My sink is leaking, can you fix it tomorrow at 2 PM?",
            customer_name: "Test Customer"
        };

        // Push an event indicating a lead came in. We use the webhook endpoint which acts like an intake.
        const res = await request.post('/api/v1/webhook', {
            headers: {
                'x-tenant-id': 'default_tenant',
                'content-type': 'application/json'
            },
            data: {
                event_type: 'tenant.work_intake.received',
                payload: leadPayload
            }
        });

        // Step 2: The sales agent and operations agent asynchronously process this.
        // We wait and check the UI for the drafted quote.
        await page.goto('/');

        // Log in (assuming auto-login or simple state based on default_tenant)
        // Check the Team Inbox or Triage feed for the "Quote Ready for Review"
        await page.goto('/team');

        // Wait for the quote card to appear. It should contain our text.
        await expect(page.locator('text=Action Required: Approve Estimate').first()).toBeVisible({ timeout: 15000 });
        await expect(page.locator('text=My sink is leaking, can you fix it tomorrow at 2 PM?').first()).toBeVisible();
        await expect(page.locator('text=Proposed Time:').first()).toBeVisible();

        // Step 3: Click "Approve & Send"
        // Wait for the specific button on the quote card.
        const approveBtn = page.getByRole('button', { name: 'Approve & Send' }).first();
        await approveBtn.click();

        // Wait for it to disappear or show a success state.
        await expect(page.locator('text=Action Required: Approve Estimate').first()).toBeHidden({ timeout: 10000 });

        // Verification: The backend lock test confirms the logic; the UI test confirms the owner sees it and can approve it.
    });
});
