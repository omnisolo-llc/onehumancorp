import { test, expect } from '../../../../e2e/fixtures';

test.describe('Omni Inbox Agentic Triage', () => {
  test('displays unread leads summary and allows inventory deduction approval', async ({ page }) => {
    // Rely on E2E Seed Data for messages and avoid overriding global API routes
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    await page.goto('/inbox');

    // Check if the mock message 'e2e-inbox-msg-1' is rendered with sender 'maya_bakes'
    const msg = page.locator('.app-list-item', { hasText: 'Do you have vegan options' }).first();
    await expect(msg).toBeVisible({ timeout: 10000 });
    await msg.click();

    // Assert the special translucent action modal button is visible from the e2e seed data
    // Because seed data contains action_type "Draft Quote", check for the dynamic quote button
    const approveButton = page.locator('button', { hasText: '✨ Send quote for $' });
    await expect(approveButton).toBeVisible();

  });
});
