import { test, expect } from '@playwright/test';

test.describe('Walkthrough and Tooltips features', () => {
  test('Dashboard walkthrough and help center elements are visible and work', async ({ page }) => {
    // Navigate using the admin credentials implicitly logged in by global setup, or just go directly
    await page.goto('/dashboard.html');

    // Check Walkthrough button
    const walkBtn = page.locator('#dashboard-walkthrough-btn');
    await expect(walkBtn).toBeVisible();
    await walkBtn.click();

    // The walkthrough overlay should appear
    const overlay = page.locator('.ohc-walkthrough-overlay');
    await expect(overlay).toBeVisible();

    const bubble = page.locator('.ohc-walkthrough-bubble');
    await expect(bubble).toBeVisible();
    await expect(bubble).toContainText('Welcome');

    // Close the walkthrough
    const closeBtn = page.locator('.ohc-walkthrough-close');
    await closeBtn.click();
    await expect(overlay).not.toBeVisible();
  });

  test('POS walkthrough and help center elements are visible and work', async ({ page }) => {
    await page.goto('/pos.html');

    // Check Walkthrough button
    const walkBtn = page.locator('#pos-walkthrough-btn');
    await expect(walkBtn).toBeVisible();
    await walkBtn.click();

    // The walkthrough overlay should appear
    const overlay = page.locator('.ohc-walkthrough-overlay');
    await expect(overlay).toBeVisible();

    const bubble = page.locator('.ohc-walkthrough-bubble');
    await expect(bubble).toBeVisible();
    await expect(bubble).toContainText('Accept Payment');

    // Close the walkthrough
    const closeBtn = page.locator('.ohc-walkthrough-close');
    await closeBtn.click();
    await expect(overlay).not.toBeVisible();

    // Check Help Center button
    const helpBtn = page.locator('#help-center-nav-btn');
    await expect(helpBtn).toBeVisible();
  });

  test('Assistant walkthrough and help center elements are visible and work', async ({ page }) => {
    await page.goto('/assistant.html');

    // Check Walkthrough button
    const walkBtn = page.locator('#assistant-walkthrough-btn');
    await expect(walkBtn).toBeVisible();
    await walkBtn.click();

    // The walkthrough overlay should appear
    const overlay = page.locator('.ohc-walkthrough-overlay');
    await expect(overlay).toBeVisible();

    const bubble = page.locator('.ohc-walkthrough-bubble');
    await expect(bubble).toBeVisible();
    await expect(bubble).toContainText('Activate your AI Support Agent');

    // Close the walkthrough
    const closeBtn = page.locator('.ohc-walkthrough-close');
    await closeBtn.click();
    await expect(overlay).not.toBeVisible();

    // Check Help Center button
    const helpBtn = page.locator('#help-center-nav-btn');
    await expect(helpBtn).toBeVisible();
  });

  test('Tooltips are injected into the page', async ({ page }) => {
    await page.goto('/dashboard.html');

    // Check tooltips registry is available
    const tooltips = await page.evaluate(() => window['OHC_TOOLTIPS']);
    expect(tooltips).toBeDefined();
    expect(tooltips['dashboard-walkthrough-btn']).toBe('Take a tour of the dashboard');
  });

  test('Help Center elements are visible', async ({ page }) => {
    await page.goto('/help.html');

    // Verify title
    await expect(page.locator('h1')).toHaveText('Help Center');

    // Verify search
    const search = page.locator('#search-input');
    await expect(search).toBeVisible();

    // Wait for the articles to load
    const results = page.locator('#results');
    await expect(results).toBeVisible();

    // The chat widget should also be there
    const chatBtn = page.locator('#ohc-help-btn');
    await expect(chatBtn).toBeVisible();
  });
});
