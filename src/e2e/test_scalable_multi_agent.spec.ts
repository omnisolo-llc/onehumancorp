import { test, expect } from './fixtures';

test.describe('Scalable multi-agent deployment', () => {
  test('user can adjust scale and deploy a fleet of agents', async ({ page, unlimitedAdminUser, loginAs }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/scaling');

    // Wait for the UI to load
    await expect(page.locator('#scaling-screen')).toBeVisible();

    // Verify initial state
    await expect(page.locator('text=3 agents')).toBeVisible();

    // Increase scale
    await page.getByRole('button', { name: 'Increase Scale (+1)' }).click();
    await expect(page.locator('text=4 agents')).toBeVisible();

    // Max scale
    await page.getByRole('button', { name: 'Max Scale (1000)' }).click();
    await expect(page.getByText('1000 agents', { exact: true })).toBeVisible();
    await expect(page.locator('text=Cloud Distributed')).toBeVisible();

    // Decrease back down a bit for the test to run quickly
    await page.getByRole('button', { name: 'Decrease Scale' }).click();
    // Now it should be 999
    await expect(page.locator('text=999 agents')).toBeVisible();

    // Set text
    await page.fill('input[placeholder="e.g. Analyze dataset"]', 'Test Scalable Task');

    // Run deployment
    // We test the API call in E2E since 1000 requests to rust backend might overwhelm test env? No, Playwright shouldn't test backend.
    // The rust backend uses a semaphore and handles 1000 tasks instantly in test mode.
    await page.locator('#deploy-agents-btn').click();

    // Wait for results
    await expect(page.locator('h3:has-text("Results")')).toBeVisible({ timeout: 15000 });
    await expect(page.locator('h3:has-text("Results (999 outputs)")')).toBeVisible();

    // Verify some outputs are shown
    await expect(page.locator('text=Agent 1:').first()).toBeVisible();
    await expect(page.locator('text=and 979 more results not shown.')).toBeVisible();
  });
});
