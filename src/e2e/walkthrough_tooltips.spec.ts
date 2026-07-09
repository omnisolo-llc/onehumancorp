import { test, expect } from '@playwright/test';

test.describe('Walkthrough and Tooltips features', () => {
  test('Dashboard walkthrough and help center elements are visible and work', async ({ page }) => {
    // Navigate using the admin credentials implicitly logged in by global setup, or just go directly
    await page.goto('/api/ui/dashboard.html');

    // Check Walkthrough button
    const walkBtn = page.locator('#dashboard-walkthrough-btn');
    await expect(walkBtn).toBeVisible();
    await walkBtn.evaluate((btn) => btn.click()); await page.waitForTimeout(500);

    // The walkthrough overlay should appear
    const overlay = page.locator('.ohc-walkthrough-overlay');
    await expect(overlay).toBeVisible();

    const bubble = page.locator('.ohc-walkthrough-bubble');
    await expect(bubble).toBeVisible();
    await expect(bubble).toContainText('Welcome');

    // Close the walkthrough
    const closeBtn = page.locator('.ohc-walkthrough-close');
    await closeBtn.evaluate((btn) => btn.click()); await page.waitForTimeout(500);
    await expect(overlay).not.toBeVisible();
  });

  test('Storefront walkthrough and help center elements are visible and work', async ({ page }) => {
    await page.goto('/storefront-builder');

    const walkBtn = page.locator('#storefront-walkthrough-btn');
    await expect(walkBtn).toBeVisible();
    await walkBtn.evaluate((btn) => btn.click()); await page.waitForTimeout(500);

    const overlay = page.locator('.ohc-walkthrough-overlay');
    await expect(overlay).toBeVisible();

    const bubble = page.locator('.ohc-walkthrough-bubble');
    await expect(bubble).toBeVisible();
    await expect(bubble).toContainText('Storefront Builder');

    const closeBtn = page.locator('.ohc-walkthrough-close');
    await closeBtn.evaluate((btn) => btn.click()); await page.waitForTimeout(500);
    await expect(overlay).not.toBeVisible();
  });

  test('POS walkthrough and help center elements are visible and work', async ({ page }) => {
    await page.goto('/payments');

    // Check Walkthrough button
    const walkBtn = page.locator('#pos-walkthrough-btn');
    await expect(walkBtn).toBeVisible();
    await walkBtn.evaluate((btn) => btn.click()); await page.waitForTimeout(500);

    // The walkthrough overlay should appear
    const overlay = page.locator('.ohc-walkthrough-overlay');
    await expect(overlay).toBeVisible();

    const bubble = page.locator('.ohc-walkthrough-bubble');
    await expect(bubble).toBeVisible();
    await expect(bubble).toContainText('Accept Payment');

    // Close the walkthrough
    const closeBtn = page.locator('.ohc-walkthrough-close');
    await closeBtn.evaluate((btn) => btn.click()); await page.waitForTimeout(500);
    await expect(overlay).not.toBeVisible();

    // Check Help Center button
    const helpBtn = page.locator('#help-center-nav-btn');
    await expect(helpBtn).toBeVisible();
  });

  test('Assistant walkthrough and help center elements are visible and work', async ({ page }) => {
    await page.goto('/agents');

    // Check Walkthrough button
    const walkBtn = page.locator('#assistant-walkthrough-btn');
    await expect(walkBtn).toBeVisible();
    await walkBtn.evaluate((btn) => btn.click()); await page.waitForTimeout(500);

    // The walkthrough overlay should appear
    const overlay = page.locator('.ohc-walkthrough-overlay');
    await expect(overlay).toBeVisible();

    const bubble = page.locator('.ohc-walkthrough-bubble');
    await expect(bubble).toBeVisible();
    await expect(bubble).toContainText('Activate your AI Support Agent');

    // Close the walkthrough
    const closeBtn = page.locator('.ohc-walkthrough-close');
    await closeBtn.evaluate((btn) => btn.click()); await page.waitForTimeout(500);
    await expect(overlay).not.toBeVisible();

    // Check Help Center button
    const helpBtn = page.locator('#help-center-nav-btn');
    await expect(helpBtn).toBeVisible();
  });

  test('Tooltips are injected into the page', async ({ page }) => {
    await page.goto('/api/ui/dashboard.html');

    // Check tooltips registry is available
    const tooltips = await page.evaluate(() => window['OHC_TOOLTIPS']);
    expect(tooltips).toBeDefined();
    expect(tooltips['dashboard-walkthrough-btn']).toBe('Take a tour of the dashboard');
  });

  test('Help Center elements are visible', async ({ page }) => {
    await page.goto('/api/ui/help.html');

    // Verify title
    await expect(page.locator('h1')).toHaveText('In-App Help Center');

    // Verify search
    const search = page.locator('#search-input');
    await expect(search).toBeVisible();

    // Wait for the articles to load
    const results = page.locator('#results');
    await expect(results).toBeVisible();

    // The chat widget should also be there
    const chatBtn = page.locator('#ohc-floating-help-btn').first();
    await expect(chatBtn).toBeVisible();
    await chatBtn.click({ force: true });

    // The chat widget should open
    const chatWidget = page.locator('#ohc-floating-help-widget').first();
    await expect(chatWidget).toBeVisible();

    // Switch to Ask AI tab
    const chatTab = page.locator('.ohc-help-tab[data-target="tab-chat"]');
    await chatTab.click();

    // Type in the input
    const chatInput = page.locator('#ohc-help-chat-input');
    await expect(chatInput).toBeVisible();
    await chatInput.fill('Hello help agent');

    // Click send
    const sendBtn = page.locator('#ohc-help-chat-send');
    await sendBtn.click();

    // Check that our message appears in the chat
    const messages = page.locator('#ohc-help-chat-messages');
    await expect(messages).toContainText('Hello help agent');
  });
});
