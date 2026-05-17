import { test, expect } from '@playwright/test';

test.describe('Health Monitoring Resilience E2E', () => {
  test('renders diagnostics health state', async ({ page }) => {
    await page.goto('/diagnostics');
    const screen = page.locator('#diagnostics-screen');

    await expect(screen).toBeVisible();
    await expect(screen).toContainText('API Server: healthy');
    await expect(screen).toContainText('Component Health');
    await expect(screen).toContainText('Database component healthy');
  });

  test('renders service health state', async ({ page }) => {
    await page.goto('/services');
    await expect(page.locator('#services-screen')).toContainText('Web Server');
    await expect(page.locator('#services-screen')).toContainText('Service log output: healthy');
  });

  test('keeps agents page reachable from dashboard', async ({ page }) => {
    await page.goto('/dashboard');
    await page.getByRole('button', { name: 'My AI Assistants' }).click();
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
    await expect(page.locator('#agents-screen')).toContainText('Status: Active');
  });
});
