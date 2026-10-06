import { test, expect } from '@playwright/test';

test.describe('Walkthrough and Tooltips features', () => {
  test('Dashboard walkthrough and help center elements are visible and work', async ({ page }) => {
    // Navigate using the admin credentials implicitly logged in by global setup, or just go directly
    await page.goto('/api/v1/ui/dashboard.html');

    // Check Walkthrough button
    const walkBtn = page.locator('#dashboard-walkthrough-btn');
    await expect(walkBtn).toBeVisible();
    await walkBtn.click();

    // The walkthrough overlay should appear
    const overlay = page.locator('.omnisolo-walkthrough-overlay');
    await expect(overlay).toBeVisible();

    const bubble = page.locator('.omnisolo-walkthrough-bubble');
    await expect(bubble).toBeVisible();
    await expect(bubble).toContainText('Welcome');

    // Close the walkthrough
    const closeBtn = bubble.getByRole('button', { name: 'Close walkthrough', exact: true });
    await closeBtn.click();
    await expect(overlay).toBeHidden();
    await expect(bubble).toBeHidden();
  });

  test('Storefront walkthrough and help center elements are visible and work', async ({ page }) => {
    await page.goto('/api/v1/ui/storefront.html');

    const walkBtn = page.locator('#storefront-walkthrough-btn');
    await expect(walkBtn).toBeVisible();
    await walkBtn.click();

    const overlay = page.locator('.omnisolo-walkthrough-overlay');
    await expect(overlay).toBeVisible();

    const bubble = page.locator('.omnisolo-walkthrough-bubble');
    await expect(bubble).toBeVisible();
    await expect(bubble).toContainText('Storefront Builder');

    const closeBtn = page.locator('.omnisolo-walkthrough-close');
    await closeBtn.click();
    await expect(overlay).not.toBeVisible();

    // Check Help Center button
    const helpBtn = page.locator('#help-center-nav-btn');
    await expect(helpBtn).toBeVisible();
  });

  test('POS walkthrough and help center elements are visible and work', async ({ page }) => {
    await page.goto('/api/v1/ui/pos.html');

    // Check Walkthrough button
    const walkBtn = page.locator('#pos-walkthrough-btn');
    await expect(walkBtn).toBeVisible();
    await walkBtn.click();

    // The walkthrough overlay should appear
    const overlay = page.locator('.omnisolo-walkthrough-overlay');
    await expect(overlay).toBeVisible();

    const bubble = page.locator('.omnisolo-walkthrough-bubble');
    await expect(bubble).toBeVisible();
    await expect(bubble).toContainText('Accept Payment');

    // Close the walkthrough
    const closeBtn = page.locator('.omnisolo-walkthrough-close');
    await closeBtn.click();
    await expect(overlay).not.toBeVisible();

    // Check Help Center button
    const helpBtn = page.locator('#help-center-nav-btn');
    await expect(helpBtn).toBeVisible();
  });

  test('Assistant walkthrough and help center elements are visible and work', async ({ page }) => {
    await page.goto('/api/v1/ui/assistant.html');

    // Check Walkthrough button
    const walkBtn = page.locator('#assistant-walkthrough-btn');
    await expect(walkBtn).toBeVisible();
    await walkBtn.click();

    // The walkthrough overlay should appear
    const overlay = page.locator('.omnisolo-walkthrough-overlay');
    await expect(overlay).toBeVisible();

    const bubble = page.locator('.omnisolo-walkthrough-bubble');
    await expect(bubble).toBeVisible();
    await expect(bubble).toContainText('Activate your AI Support Agent');

    // Close the walkthrough
    const closeBtn = page.locator('.omnisolo-walkthrough-close');
    await closeBtn.click();
    await expect(overlay).not.toBeVisible();

    // Check Help Center button
    const helpBtn = page.locator('#help-center-nav-btn');
    await expect(helpBtn).toBeVisible();
  });

  test('Tooltips are injected into the page', async ({ page }) => {
    const loaded = page.waitForResponse(response =>
      new URL(response.url()).pathname === '/api/v1/tooltips' && response.request().method() === 'GET');
    await page.goto('/api/v1/ui/dashboard.html');
    const response = await loaded;
    expect(response.status()).toBe(200);
    const tooltips = await response.json() as Record<string, string>;
    const expectedText = tooltips['help-btn-tooltip'];
    expect(expectedText).toEqual(expect.any(String));
    expect(expectedText.trim().length).toBeGreaterThan(0);

    // Observe the loaded provider through its accessible UI, not an asynchronous global.
    const target = page.locator('#help-btn-tooltip');
    const helpButton = target.getByRole('button', { name: 'Open help chat', exact: true });
    await expect(target).toHaveAttribute('data-tooltip', expectedText);
    await helpButton.hover();
    const tooltip = page.getByRole('tooltip');
    await expect(tooltip).toHaveText(expectedText);
    await expect(tooltip).toBeVisible();
    await page.mouse.move(0, 0);
    await expect(tooltip).toBeHidden();
    await helpButton.focus();
    await expect(tooltip).toBeVisible();
    await expect(target).toHaveAttribute('aria-describedby', 'help-btn-tooltip-description');
    await helpButton.press('Escape');
    await expect(tooltip).toBeHidden();
    await expect(helpButton).toBeFocused();
  });

  test('Help Center elements are visible', async ({ page }) => {
    await page.goto('/api/v1/ui/help.html');

    // Verify title
    await expect(page.locator('h1')).toHaveText('In-App Help Center');

    // Verify search
    const search = page.locator('#search-input');
    await expect(search).toBeVisible();

    // Wait for the articles to load
    const results = page.locator('#results');
    await expect(results).toBeVisible();

    // The chat widget should also be there
    const chatBtn = page.locator('#ohc-floating-help-btn');
    await expect(chatBtn).toBeVisible();
    await chatBtn.click();

    // The chat widget should open
    const chatWidget = page.locator('#ohc-floating-help-widget, #omnisolo-floating-help-widget').first();
    await expect(chatWidget).toBeVisible();

    // Switch to Ask AI tab
    const chatTab = page.locator('.omnisolo-help-tab[data-target="tab-chat"]');
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
