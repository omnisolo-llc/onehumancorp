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
    await expect(page.getByRole('heading', { name: 'Service Manager' })).toBeVisible();
    await expect(page.locator('#services-screen')).toContainText(/Status: running|No runtime status is being reported/);
    await expect(page.locator('#services-screen')).toContainText(/Resource usage: CPU 5%, memory 128MB|not exposed by the service API/);
  });

  test('keeps agents page reachable from dashboard', async ({ page }) => {
    await page.goto('/dashboard');
    await page.getByRole('link', { name: 'AI Departments' }).click();
    await expect(page.getByRole('heading', { name: 'AI Departments' })).toBeVisible();
    await expect(page.getByText('Your autonomous business team.')).toBeVisible();
  });
});
