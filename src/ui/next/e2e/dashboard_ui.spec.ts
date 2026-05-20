import { test, expect } from '@playwright/test';

test.describe('Dashboard E2E', () => {
  test.beforeEach(async ({ page }) => {
    // Note: Adjust the URL based on baseURL. According to playwright.config.ts, baseURL is used.
    await page.goto('http://localhost:3000/dashboard');
  });

  test('dashboard page loads correctly and displays the Business Snapshot section', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Business Snapshot' })).toBeVisible();
  });

  test('Team Activity section header is visible', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Team Activity' })).toBeVisible();
  });

  test('Swarm Online indicator is visible', async ({ page }) => {
    await expect(page.getByText('Swarm Online')).toBeVisible();
  });

  test('Waiting for team activity loading state is visible initially', async ({ page }) => {
    await expect(page.getByText('Waiting for team activity...')).toBeVisible();
  });

  test('Simulated mock activity records appear in the DOM after delay', async ({ page }) => {
    // Wait for the initial loading text to disappear
    await expect(page.getByText('Waiting for team activity...')).toBeVisible();

    // The mock interval fires every 4500ms. We wait for one of the mock actions to appear.
    // In our mocked actions we have elements that could be "Reviewing customer inquiry", etc.
    // Using a regex to match one of the actions.
    const activityLocator = page.getByText(/Reviewing customer inquiry|Generating weekly report|Optimizing website layout|Responding to support ticket|Updating product inventory/i);

    // Wait up to 8000ms for the mock interval to fire and render the UI changes.
    await expect(activityLocator.first()).toBeVisible({ timeout: 8000 });
  });
});
