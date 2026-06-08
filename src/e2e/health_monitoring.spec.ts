import { test, expect } from './fixtures';

test.describe('Health Monitoring Resilience E2E', () => {
  test('renders diagnostics health state', async ({ page }) => {
    await page.goto('/diagnostics');
    const screen = page.locator('#diagnostics-screen');

    await expect(screen).toBeVisible();
    await expect(screen).toContainText('System Status: All systems operational');
    await expect(screen).toContainText('Database: Healthy');
    await expect(screen).toContainText('Redis: Healthy');
  });

  test('renders service health state', async ({ page }) => {
    await page.goto('/services');
    await expect(page.locator('#services-screen')).toContainText('Service Manager');
    await expect(page.locator('#services-screen')).toContainText('Status: running');
    await expect(page.locator('#services-screen')).toContainText('Resource usage: CPU 5%, memory 128MB');
  });

  test('keeps agents page reachable from dashboard', async ({ page }) => {
    await page.goto('/dashboard');
    await page.getByRole('link', { name: 'Agents' }).click();
    await expect(page.getByRole('heading', { name: 'Jarvis Assistant' })).toBeVisible();
    await expect(page.getByText('Your autonomous business team.')).toBeVisible();
  });
});
