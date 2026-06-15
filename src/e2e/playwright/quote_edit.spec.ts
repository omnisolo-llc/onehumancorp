import { test, expect } from './fixtures';

test.describe('Quote Edit E2E', () => {
  test('verify editing a quote works naturally', async ({ page }) => {
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

    // Open edit sheet
    const editBtn = page.locator('#edit-quote-btn');
    await expect(editBtn).toBeVisible({ timeout: 5000 });
    await editBtn.click();

    // Save edits
    const saveBtn = page.locator('#btn-save-edits');
    await expect(saveBtn).toBeVisible({ timeout: 5000 });
    await saveBtn.click();

    // Make sure edit sheet closes
    await expect(saveBtn).not.toBeVisible({ timeout: 5000 });
  });
});
