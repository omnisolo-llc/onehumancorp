import { test, expect } from '@playwright/test';

test.describe('Omni Inbox Agentic Triage', () => {
  test('displays unread leads summary and allows inventory deduction approval', async ({ page }) => {
    const testTenant = 'e2e-omni-inbox-tenant-' + Date.now();

    await page.goto('/login');
    await page.evaluate((t) => { window['localStorage'].setItem('tenant_id', t); window['localStorage'].setItem('tenant', t); }, testTenant);
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    const createPayload = () => JSON.parse(JSON.stringify([
      {
        source: 'Instagram DM',
        priority: 'high',
        context: 'Message: Customer asked about vegan cakes.',
        action_type: 'Draft Reply',
        action_payload: 'Yes! We have 2 available. Should I hold them for you? [Send & Deduct Inventory]',
        customer_id: 'maya_customer_123',
        status: 'pending'
      }
    ]));

    const seedData = createPayload();
    for (const data of seedData) {
      await page.request.post(`/api/triage/create?tenant_id=${encodeURIComponent(testTenant)}`, {
        data
      });
    }

    // Navigate to the inbox page
    await page.goto('/inbox');

    // Assert the summary card is visible
    const summaryCard = page.locator('.daily-summary');
    await expect(summaryCard).toBeVisible();

    // Assert the message is visible in the list
    const messageButton = page.locator('button', { hasText: 'Instagram DM' });
    await expect(messageButton).toBeVisible();

    await messageButton.click();

    await expect(page.locator('text="[Send & Deduct Inventory]"')).toBeVisible();

    // The button might have text related to approval
    const approveButton = page.locator('button', { hasText: 'Approve' }).first();
    await expect(approveButton).toBeVisible();

    await approveButton.click();

    // We expect it to disappear or show sent
    await expect(page.locator('text="Instagram DM"').first()).not.toBeVisible({ timeout: 10000 });
  });
});
