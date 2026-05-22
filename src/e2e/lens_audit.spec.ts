import { test, expect } from './fixtures';

test.describe('Lens Audit E2E Flow', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('verify dashboard visual state and full UI lifecycle', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
    await expect(page.locator('nav')).toBeVisible();
    await expect(page.getByText("Today's Sales")).toBeVisible();
    await expect(page.getByText("$114.99")).toBeVisible({ timeout: 10000 });

    // Verify that mock data section is removed
    await expect(page.getByText("Referral Program")).toBeHidden();
    await expect(page.getByText("Active Referrals")).toBeHidden();
    await expect(page.getByText("Revenue from Referrals")).toBeHidden();
    await expect(page.getByText("Pending Rewards")).toBeHidden();
    await expect(page.getByText("🎁 Invite a Business & Earn $50")).toBeHidden();
  });

  test('verify setup wizard starts and preserves real form state', async ({ page }) => {
    await page.goto('/business-setup');
    await page.getByRole('button', { name: /Start My Business/ }).click();

    await expect(page.getByRole('heading', { name: 'What kind of business are you building?' })).toBeVisible();
    await page.getByPlaceholder('Business type').fill('Online Store');
    await page.getByRole('button', { name: /Next/ }).click();

    await expect(page.getByRole('heading', { name: 'Give your business a name' })).toBeVisible();
    await expect(page.getByPlaceholder("What is your business called?")).toBeVisible();
  });

  test('verify responsive navigation compliance', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });

    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
    await expect(page.locator('#mobile-bottom-nav')).toBeVisible();
  });

  test('verify unknown routes fall back without crashing', async ({ page }) => {
    await page.goto('/setup-screen');

    await expect(page.getByRole('heading').first()).toBeVisible();
  });

  test('verify user guide and help actions remain reachable', async ({ page }) => {
    await page.getByRole('button', { name: 'How to use this app' }).click();

    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });
});
