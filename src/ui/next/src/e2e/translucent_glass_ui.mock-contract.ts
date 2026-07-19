import { test, expect } from '../../../../e2e/fixtures';

test.describe('Translucent Glass UI Aesthetics Verification', () => {
  test.beforeEach(async ({ page }) => {
    // Clear local storage to ensure fresh state
    await page.addInitScript(() => {
      window.localStorage.clear();
    });
  });

  test('Verify Card component glassmorphism effect on Diagnostics Page', async ({ page }) => {
    await page.goto('/diagnostics');

    // Wait for the main glassmorphism container (Card component outputs div)
    await page.waitForSelector('.backdrop-blur-\\[30px\\]');

    // Select the glassmorphism element
    const glassContainer = page.locator('.backdrop-blur-\\[30px\\]').first();

    // Ensure it exists and is visible
    await expect(glassContainer).toBeVisible();

    // Evaluate the computed styles to guarantee the premium aesthetics
    const styles = await glassContainer.evaluate((el) => {
      const computed = window.getComputedStyle(el);
      return {
        backdropFilter: computed.backdropFilter,
        backgroundColor: computed.backgroundColor,
        border: computed.border,
      };
    });

    expect(styles).toBeDefined();
    expect(styles.backgroundColor).toMatch(/rgba?\(\d+,\s*\d+,\s*\d+(?:,\s*[0-9.]+)?\)|rgb\(\d+,\s*\d+,\s*\d+\)/);
    expect(styles.backdropFilter).toMatch(/blur\(30px\)\s+saturate\((?:210%|2\.1)\)/);
    expect(styles.border).toBeDefined();
  });

  test('Verify Card component glassmorphism effect on Cost Dashboard Page', async ({ page }) => {
    await page.goto('/cost-dashboard');

    await page.waitForSelector('.backdrop-blur-\\[30px\\]');
    const glassContainer = page.locator('.backdrop-blur-\\[30px\\]').first();
    await expect(glassContainer).toBeVisible();

    // Evaluate the computed styles to guarantee the premium aesthetics
    const styles = await glassContainer.evaluate((el) => {
      const computed = window.getComputedStyle(el);
      return {
        backdropFilter: computed.backdropFilter,
        backgroundColor: computed.backgroundColor,
        border: computed.border,
      };
    });

    expect(styles).toBeDefined();
    expect(styles.backgroundColor).toMatch(/rgba?\(\d+,\s*\d+,\s*\d+(?:,\s*[0-9.]+)?\)|rgb\(\d+,\s*\d+,\s*\d+\)/);
    expect(styles.backdropFilter).toMatch(/blur\(30px\)\s+saturate\((?:210%|2\.1)\)/);
    expect(styles.border).toBeDefined();
  });
});
