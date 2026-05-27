import { test, expect } from './fixtures';

test.describe('Telemetry and Cost Visualizer', () => {
  test('shows operational telemetry metrics', async ({ page }) => {
    await page.goto('/diagnostics');
    const diagnostics = page.locator('#diagnostics-screen');

    await expect(diagnostics).toContainText('Response time latency: 42 ms');
    await expect(diagnostics).toContainText('Request throughput: 24 rps');
    await expect(diagnostics).toContainText('Memory: 512MB / 1GB');
  });

  test('shows AI cost usage details', async ({ page }) => {
    await page.goto('/my-plan');
    await page.getByRole('button', { name: 'View Cost Details' }).click();

    await expect(page.getByRole('heading', { name: 'Cost & AI Usage' })).toBeVisible();
    await expect(page.getByText(/Total Costs: \$\d+\.\d{2}/)).toBeVisible();
    await expect(page.getByText(/LLM Usage: \d+(,\d+)* tokens/)).toBeVisible();
  });
});
