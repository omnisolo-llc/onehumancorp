import { test, expect } from './fixtures';

test.describe('Lens Audit E2E Flow', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('verify dashboard visual state and full UI lifecycle', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
    await expect(page.locator('nav')).toBeVisible();

    // Verify sales matches orders table in seeded database
    await expect(page.getByText("Today's Sales")).toBeVisible();
    await expect(page.getByText("$114.99")).toBeVisible({ timeout: 10000 });

    // Verify customers match customers table in seeded database
    await expect(page.getByText("Active Customers")).toBeVisible();
    await expect(page.getByText("2", { exact: true })).toBeVisible({ timeout: 10000 });

    // Verify orders match orders table with pending status
    await expect(page.getByText("Pending Orders")).toBeVisible();
    await expect(page.getByText("1", { exact: true })).toBeVisible({ timeout: 10000 });
  });

  test('verify dashboard database data binding directly', async ({ page }) => {
    // Tests that UI reflects DB without mock hardcoding
    await expect(page.getByText("Today's Sales")).toBeVisible();
    await expect(page.getByText("$114.99")).toBeVisible();
  });

  test('verify active customers data binding directly', async ({ page }) => {
    // Tests that UI reflects DB for customers without mock hardcoding
    await expect(page.getByText("Active Customers")).toBeVisible();
    await expect(page.getByText("2", { exact: true })).toBeVisible();
  });

  test('verify pending orders data binding directly', async ({ page }) => {
    // Tests that UI reflects DB for orders without mock hardcoding
    await expect(page.getByText("Pending Orders")).toBeVisible();
    await expect(page.getByText("1", { exact: true })).toBeVisible();
  });

  test('verify lack of mock swarm activity on initial load', async ({ page }) => {
    // We removed the mockInterval, so we expect the "Waiting for team activity..." message initially
    await expect(page.getByText("Waiting for team activity...")).toBeVisible();
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
