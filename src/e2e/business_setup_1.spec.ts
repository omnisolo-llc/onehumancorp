import { test, expect } from './fixtures';

test.describe('Business Setup Wizard', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/website-builder');
    await expect(page.locator('#setup-screen')).toBeVisible();
  });

  test('shows the current setup welcome step', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Welcome to OHC Smart Builder' })).toBeVisible();
    await expect(page.getByRole('button', { name: /Build My Storefront/ })).toBeVisible();
  });

  test('builds storefront from bio', async ({ page }) => {
    // Need to trigger the mock in the route by specific text to avoid network mock
    await page.getByPlaceholder('e.g. I run a mobile dog grooming service in Portland').fill('I run a local bakery named maya');
    await page.getByRole('button', { name: /Build My Storefront/ }).click();

    // Wait for blocks to load (preview mode)
    // Wait for at least generating state to appear
    await expect(page.getByText('Agents are building your store...')).toBeVisible({ timeout: 15000 });
  });
});
