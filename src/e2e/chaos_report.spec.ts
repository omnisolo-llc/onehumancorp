import { test, expect } from '@playwright/test';

test.describe('Chaos Report UI', () => {
  test('Verify chaos report page loads and displays latency/error panels', async ({ page }) => {
    const backendUrl = process.env.OHC_BACKEND_URL || 'http://127.0.0.1:18789';
    await page.goto(`${backendUrl}/chaos-report`);

    await expect(page.locator('h1', { hasText: 'System Reliability Report' })).toBeVisible({ timeout: 10000 });

    // Check latency panel
    const latencyPanel = page.locator('#latency-panel');
    await expect(latencyPanel).toBeVisible();
    await expect(latencyPanel.locator('h2', { hasText: 'Latency Distribution' })).toBeVisible();

    // Check error rate panel
    const errorPanel = page.locator('#error-panel');
    await expect(errorPanel).toBeVisible();
    await expect(errorPanel.locator('h2', { hasText: 'Error Rate Over Time' })).toBeVisible();
  });

  test('Verify theme toggle changes background styles', async ({ page }) => {
    const backendUrl = process.env.OHC_BACKEND_URL || 'http://127.0.0.1:18789';
    await page.goto(`${backendUrl}/chaos-report`);

    const themeToggle = page.locator('#theme-toggle');
    await expect(themeToggle).toBeVisible({ timeout: 10000 });

    const latencyPanel = page.locator('#latency-panel');
    const errorPanel = page.locator('#error-panel');

    // The page loads in whatever theme applies, but we can click toggle to switch and check classes
    const initialClass = await latencyPanel.getAttribute('class');
    const isInitiallyDark = initialClass?.includes('glass-panel-dark');

    await themeToggle.click();

    if (isInitiallyDark) {
      await expect(latencyPanel).toHaveClass(/glass-panel-light/);
      await expect(errorPanel).toHaveClass(/glass-panel-light/);
    } else {
      await expect(latencyPanel).toHaveClass(/glass-panel-dark/);
      await expect(errorPanel).toHaveClass(/glass-panel-dark/);
    }
  });
});
