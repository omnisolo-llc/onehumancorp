import { test, expect } from './fixtures';

test.describe('OHC Premium Design Standards Audit', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('verify ohc-hybrid-panel container cards adopt 16px border-radius', async ({ page }) => {
    await page.goto('/');
    const dashboardLink = page.getByRole('link', { name: 'Dashboard' }).first();
    await dashboardLink.click();

    // Ensure we are on the dashboard
    await expect(page).toHaveURL(/.*\/dashboard/);

    const panel = page.locator('.ohc-hybrid-panel').first();
    await expect(panel).toBeVisible();

    // The specification says "16px for container cards"
    const borderRadius = await panel.evaluate((el) => {
        return window.getComputedStyle(el).borderRadius;
    });

    expect(borderRadius).toBe('16px');
  });

  test('verify upgrade checkout navigation handles routing instead of mockup alert', async ({ page }) => {
    await page.goto('/dashboard');

    // Click View AI Insights which triggers the upgrade modal
    const viewInsightsBtn = page.getByRole('button', { name: 'View AI Insights' });
    await viewInsightsBtn.click();

    // Find the Upgrade Now button
    const upgradeBtn = page.getByRole('button', { name: /Upgrade Now/ });
    await expect(upgradeBtn).toBeVisible();

    // Click it and verify it navigates to pricing (since we removed the mock alert)
    await upgradeBtn.click();

    await expect(page).toHaveURL(/.*\/pricing/);
  });

  test('verify dashboard action buttons adopt 8px border-radius', async ({ page }) => {
    await page.goto('/dashboard');

    // Check Seasonal Promos button
    const promoBtn = page.getByRole('link', { name: /Seasonal Promos/ }).first();
    await expect(promoBtn).toBeVisible();

    const borderRadius = await promoBtn.evaluate((el) => {
        return window.getComputedStyle(el).borderRadius;
    });

    expect(borderRadius).toBe('8px');
  });

  test('verify dashboard action modals adopt 16px container border-radius', async ({ page }) => {
    await page.goto('/dashboard');

    const generateBtn = page.getByRole('button', { name: /Generate Promotion/ });
    await generateBtn.click();

    // The modal should appear
    const modalContainer = page.locator('.fixed.inset-0 .bg-white.w-full.max-w-md').first();
    await expect(modalContainer).toBeVisible();

    const borderRadius = await modalContainer.evaluate((el) => {
        return window.getComputedStyle(el).borderRadius;
    });

    expect(borderRadius).toBe('16px');
  });

  test('verify dashboard modal copy buttons adopt 8px border-radius', async ({ page }) => {
    await page.goto('/dashboard');

    const generateBtn = page.getByRole('button', { name: /Generate Promotion/ });
    await generateBtn.click();

    // The modal should appear
    const copyBtn = page.getByRole('button', { name: /Copy Message/ });
    await expect(copyBtn).toBeVisible();

    const borderRadius = await copyBtn.evaluate((el) => {
        return window.getComputedStyle(el).borderRadius;
    });

    expect(borderRadius).toBe('8px');
  });
});

  test('verify glassmorphism styling on dark and light mode', async ({ page }) => {
    // Navigating to website-builder to restore the test
    await page.goto('/website-builder');

    // Test dark mode glassmorphism
    const container = page.locator('.glassmorphism');
    await expect(container).toBeVisible();

    // Evaluate if the container correctly computes backdrop-filter blur
    const hasBlur = await container.evaluate((el) => {
        const style = window.getComputedStyle(el);
        return style.backdropFilter.includes('blur(20px)') || style.webkitBackdropFilter.includes('blur(20px)');
    });

    expect(hasBlur).toBe(true);
  });
