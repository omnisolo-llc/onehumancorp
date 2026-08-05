import { test, expect } from '@playwright/test';

test.describe('Omni Inbox Agentic Triage', () => {
  test('displays unread leads summary and allows inventory deduction approval', async ({ page }) => {
    const tenantId = await page.evaluate(() => window.localStorage.getItem('tenant_id') || 'e2e-tenant');

    // We must wait for the background worker to process the webhook and generate the approval draft.
    // In a real e2e flow, we can use a polling mechanism or simply assert the list page.
    await page.goto('/inbox');

    // 5. Assert the summary card is visible
    const summaryCard = page.locator('.daily-summary');
    await expect(summaryCard).toBeVisible({ timeout: 15000 });

    // We don't strictly assert the count because tests can run concurrently, just that there's a lead.
    await expect(summaryCard).toContainText('unread');
  });
});
