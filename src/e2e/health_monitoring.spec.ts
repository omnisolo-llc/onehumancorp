import { test, expect } from './fixtures';

test.describe('Health Monitoring Resilience E2E', () => {
  test('renders diagnostics health state', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await page.goto('/diagnostics');
    const screen = page.locator('#diagnostics-screen');

    await expect(screen).toBeVisible();
    await expect(screen).toContainText('API Server: healthy');
    await expect(screen).toContainText('Component Health');
    await expect(screen).toContainText('Database: Healthy');
  });

  test('renders service health state', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await page.goto('/services');
    await expect(page.locator('#services-screen')).toContainText('Web Server');
    await expect(page.locator('#services-screen')).toContainText('Service log output: healthy');
  });

  test('keeps agents page reachable from dashboard', async ({ page }) => {
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
    await page.goto('/dashboard');
    await page.getByRole('button', { name: 'Manage AI Assistants' }).click();
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
    await expect(page.locator('#team-screen')).toContainText('Status: Active');
  });
});
