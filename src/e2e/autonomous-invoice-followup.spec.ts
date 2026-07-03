import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';
import { v4 as uuidv4 } from 'uuid';

test.describe('Autonomous Invoice Follow-Up', () => {
  test('Finance agent drafts polite reminder for overdue invoice', async ({ page, request, browser }) => {
    await adminPage(browser, page);
    const orgId = process.env.OHC_DEFAULT_TENANT_ID || 'e2e-tenant';
    const invoiceId = `inv_${uuidv4()}`;

    await test.step('Trigger invoice followup simulation', async () => {
        const response = await request.post('/api/agents/approvals/simulate-invoice-followup', {
            headers: {
                Authorization: `Bearer ${process.env.E2E_ADMIN_TOKEN || 'admin_test_token'}`,
            },
        });
        expect(response.ok()).toBeTruthy();
    });

    await test.step('Verify Agent Feed displays Invoice Followup drafts', async () => {
        await page.goto('/dashboard');
        const feedSection = page.locator('section[aria-label="Unified Agent Feed"]');
        await expect(feedSection).toBeVisible({ timeout: 15000 });

        const followupCard = feedSection.locator('text="Action Required: Approve Invoice Reminder"').first();
        await expect(followupCard).toBeVisible({ timeout: 15000 });

        const generatedMessage = feedSection.locator('text="Hi there, just checking in to see if you received invoice inv_simulated_123. Let us know if you have any questions!"').first();
        await expect(generatedMessage).toBeVisible();

        const approveButton = feedSection.locator('button', { hasText: 'Approve & Send' }).first();
        await expect(approveButton).toBeVisible();
        await approveButton.click();

        await expect(followupCard).not.toBeVisible({ timeout: 10000 });
    });
  });
});
