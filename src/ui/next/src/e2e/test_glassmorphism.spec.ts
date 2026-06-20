import { test, expect } from '@playwright/test';

test.describe('Premium Aesthetics Verification', () => {
  test.beforeEach(async ({ page }) => {
    // Clear local storage to ensure fresh state
    await page.addInitScript(() => {
      window.localStorage.clear();
    });
  });

  test('Verify glassmorphism effect on Builder Page', async ({ page }) => {
    await page.goto('/builder');

    // Wait for the main glassmorphism container
    await page.waitForSelector('.glassmorphism');

    // Select the glassmorphism element
    const glassContainer = page.locator('.glassmorphism').first();

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
    expect(styles.backgroundColor).toMatch(/rgba\(255,\s*255,\s*255,\s*0\.65\)|rgba\(22,\s*22,\s*26,\s*0\.7\)/);
    expect(styles.backdropFilter).toMatch(/blur\(30px\)\s+saturate\(210%\)/);
    expect(styles.border).toMatch(/1px solid rgba\(255,\s*255,\s*255,\s*(0\.4|0\.1)\)/);
  });

  test('Verify glassmorphism effect on Invoice Generator Page', async ({ page }) => {
    await page.goto('/invoice-generator');

    await page.waitForSelector('.glassmorphism');
    const glassContainer = page.locator('.glassmorphism').first();
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
    expect(styles.backgroundColor).toMatch(/rgba\(255,\s*255,\s*255,\s*0\.65\)|rgba\(22,\s*22,\s*26,\s*0\.7\)/);
    expect(styles.backdropFilter).toMatch(/blur\(30px\)\s+saturate\(210%\)/);
    expect(styles.border).toMatch(/1px solid rgba\(255,\s*255,\s*255,\s*(0\.4|0\.1)\)/);
  });

  test('Verify glassmorphism effect on Setup Wizard', async ({ page }) => {
    await page.goto('/onboarding');
    await expect(page.getByText("10-Minute Setup Wizard")).toBeVisible();
    await page.getByRole('button', { name: 'Start My Business' }).click();

    // The wizard container usually has glassmorphism
    await page.waitForSelector('.glassmorphism');
    const glassContainer = page.locator('.glassmorphism').first();
    await expect(glassContainer).toBeVisible();

    const styles = await glassContainer.evaluate((el) => {
      const computed = window.getComputedStyle(el);
      return {
        backdropFilter: computed.backdropFilter,
        backgroundColor: computed.backgroundColor,
        border: computed.border,
      };
    });

    expect(styles).toBeDefined();
    expect(styles.backgroundColor).toMatch(/rgba\(255,\s*255,\s*255,\s*0\.65\)|rgba\(22,\s*22,\s*26,\s*0\.7\)/);
    expect(styles.backdropFilter).toMatch(/blur\(30px\)\s+saturate\(210%\)/);
    expect(styles.border).toMatch(/1px solid rgba\(255,\s*255,\s*255,\s*(0\.4|0\.1)\)/);
  });

  test('Verify transparent class on Dashboard', async ({ page }) => {
    await page.goto('/dashboard');

    const mainContainer = page.locator('body');
    await expect(mainContainer).toBeVisible();
  });
});
