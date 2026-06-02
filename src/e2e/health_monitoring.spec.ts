import { test, expect } from './fixtures';

test.describe('Health Monitoring Resilience E2E', () => {
  test('renders diagnostics health state', async ({ page }) => {
    await page.goto('/diagnostics');
    const screen = page.locator('#diagnostics-screen');

    await expect(screen).toBeVisible();
    await expect(screen).toContainText('Operational Telemetry');
    await expect(screen).toContainText('Response time latency');
    await expect(screen).toContainText('Request throughput');
  });

  test('renders service health state', async ({ page }) => {
    await page.goto('/services/new');
    await expect(page.getByRole('heading', { name: 'Add Service' })).toBeVisible();
    await expect(page.getByText('Service Title')).toBeVisible();
  });

  test('keeps agents page reachable from dashboard', async ({ page }) => {
    await page.goto('/dashboard');
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'AI Departments' })).toBeVisible();
    await expect(page.getByText('Pro Mode')).toBeVisible();
  });
});
