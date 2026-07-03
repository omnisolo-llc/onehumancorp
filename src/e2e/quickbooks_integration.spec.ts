import { test, expect } from '@playwright/test';

test.describe('QuickBooks Integration', () => {
  test('Owner can navigate to Integrations and see QuickBooks Online option', async ({ page }) => {
    await page.goto('/integrations');

    await expect(page.locator('text=QuickBooks Online')).toBeVisible();
    await expect(page.locator('text=Sync payments and invoices automatically to your accounting ledger.')).toBeVisible();

    const connectButton = page.locator('text=QuickBooks Online').locator('..').locator('button', { hasText: 'Connect' });
    await expect(connectButton).toBeVisible();

    await connectButton.click();
    await expect(page.locator('text=Connect QuickBooks Online')).toBeVisible();

    const continueButton = page.locator('text=Continue to QuickBooks');
    await expect(continueButton).toBeVisible();

    const [request] = await Promise.all([
      page.waitForRequest(req => req.url().includes('appcenter.intuit.com')),
      continueButton.click()
    ]);

    expect(request.url()).toContain('client_id=client');
    expect(request.url()).toContain('response_type=code');
  });
});
