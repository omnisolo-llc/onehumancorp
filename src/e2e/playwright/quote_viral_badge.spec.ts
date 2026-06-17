import { test, expect } from '../fixtures';

test.describe('Quote Viral Badge E2E', () => {
  test('verify viral quote badge appears on quote page naturally', async ({ page }) => {
    // Start from home/login
    await page.goto('/login');
    await page.getByPlaceholder('Email').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Sign in' }).click();

    // Wait for the dashboard to load
    await page.waitForURL('**/dashboard**');
    await page.waitForLoadState('networkidle');

    // Switch to proposals tab naturally
    const proposalsTab = page.locator('.triage-tab').filter({ hasText: 'Proposals' }).first();
    await proposalsTab.click();

    // Find the edit quote button inside the triage feed
    const editQuoteBtn = page.getByTestId('edit-quote-draft').first();
    await expect(editQuoteBtn).toBeVisible({ timeout: 15000 });

    // Click to navigate to the quote page naturally
    await editQuoteBtn.click();

    // Wait for the quote page to load
    await page.waitForLoadState('networkidle');

    // Assert the badge
    const badge = page.getByTestId('viral-quote-badge');
    await expect(badge).toBeVisible({ timeout: 15000 });
    await expect(badge).toContainText('Run your business like this with AI assistant.');
  });
});
