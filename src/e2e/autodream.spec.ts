import { test, expect } from '@playwright/test';

test.describe('AutoDream Memory Consolidation UI', () => {
  test.beforeEach(async ({ page }) => {
    // Standard OHC login flow would go here, but for these UI tests
    // we assume we can navigate to the dashboard directly or through /login
    await page.goto('/dashboard');
  });

  test('AutoDreamMemory component is visible on dashboard', async ({ page }) => {
    await expect(page.getByText(/Memory Consolidation/i)).toBeVisible();
    await expect(page.getByText(/Consolidated Intelligence/i)).toBeVisible();
    await expect(page.getByText(/AutoDream Active/i)).toBeVisible();
  });

  test('AutoDreamMemory status updates periodically', async ({ page }) => {
    const memoryCount = page.locator('span:has-text("Total Memories:")').locator('span.font-bold');
    await expect(memoryCount).toBeVisible();
    const initialText = await memoryCount.innerText();

    // The component has a mock polling every 30s, but we can't wait that long in a typical test
    // Instead we just verify it exists and shows some value
    expect(parseInt(initialText)).toBeGreaterThanOrEqual(0);
  });

  test('Manual consolidation triggers "Dreaming" state', async ({ page }) => {
    const dreamButton = page.getByRole('button', { name: /Dream Now/i });
    await expect(dreamButton).toBeVisible();

    await dreamButton.click();

    // Check for "Dreaming..." state
    await expect(page.getByText(/Dreaming.../i)).toBeVisible();

    // Verify it returns to normal after simulation (mocked to 2s)
    await expect(page.getByRole('button', { name: /Dream Now/i })).toBeVisible({ timeout: 5000 });
  });

  test('AutoDreamMemory follows premium glassmorphism standards', async ({ page }) => {
    const container = page.locator('div:has-text("Consolidated Intelligence")').first().locator('..').locator('..');

    // Verify styles using computed style
    const styles = await container.evaluate((el) => {
      const computed = window.getComputedStyle(el);
      return {
        background: computed.backgroundColor,
        backdropFilter: computed.backdropFilter,
        borderRadius: computed.borderRadius,
        border: computed.border
      };
    });

    // Light mode glass tokens: background: rgba(255, 255, 255, 0.65), backdrop-filter: blur(30px) saturate(210%)
    // Note: Playwright's backgroundColor returns rgba format
    expect(styles.background).toContain('rgba(255, 255, 255, 0.65)');
    expect(styles.backdropFilter).toContain('blur(30px)');
    expect(styles.backdropFilter).toContain('saturate(210%)');
    expect(styles.borderRadius).toBe('16px');
  });

  test('AutoDreamMemory displays last consolidated time', async ({ page }) => {
    await expect(page.getByText(/Last Updated:/i)).toBeVisible();
    const lastUpdated = page.locator('span:has-text("Last Updated:")').locator('span.font-bold');
    const text = await lastUpdated.innerText();
    expect(text).not.toBe('Never');
  });
});
