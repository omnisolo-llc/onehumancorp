import { test, expect } from './fixtures';

test.describe('Swarm Observability Dashboard', () => {
  test('displays core metrics correctly', async ({ page }) => {
    // Navigate to the observability dashboard
    await page.goto('/swarm-observability');

    // Wait for the page to load
    await expect(page.getByRole('heading', { name: 'Swarm Observability Dashboard' })).toBeVisible();

    // Verify sections are visible
    await expect(page.getByText('Active Agents')).toBeVisible();
    await expect(page.getByText('Pending Missions')).toBeVisible();
    await expect(page.getByText('Avg Task Latency')).toBeVisible();
    await expect(page.getByText('Database Mode')).toBeVisible();
  });
});
