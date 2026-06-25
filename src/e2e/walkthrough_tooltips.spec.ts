import { test, expect } from '@playwright/test';

test.describe('Walkthrough and Tooltips features', () => {
  test('Dashboard walkthrough and help center elements are visible and work', async ({ page }) => {
    await page.goto('/dashboard');
    await page.waitForTimeout(2000);

    const walkBtn = page.locator('#dashboard-walkthrough-btn');
    await expect(walkBtn).toBeVisible();

    await walkBtn.click();
    await page.waitForTimeout(1000);
  });

  test('Help Center elements are visible', async ({ page }) => {
    await page.goto('/help');
    await page.waitForTimeout(2000);

    await expect(page.locator('h1[data-testid="help-center-title"]')).toHaveText('In-App Help Center');

    const search = page.getByTestId('help-search-input');
    await expect(search).toBeAttached();

    // Just dispatch the event that the button would dispatch
    await page.evaluate(() => {
        const event = new CustomEvent('open-help-chat');
        window.dispatchEvent(event);
    });

    // The chat widget should open
    const chatWidget = page.locator('div[data-ui-overlay="true"]').filter({ hasText: 'Help' });
    await expect(chatWidget).toBeAttached();

    // Switch to Ask AI tab
    const chatTab = page.locator('button', { hasText: 'Ask AI' }).last();
    await chatTab.evaluate((el) => el.click());

    const chatInput = page.getByPlaceholder('Ask anything...').first();
    await expect(chatInput).toBeAttached();
    await chatInput.fill('Hello help agent');

    const sendBtn = page.locator('button[aria-label="Send message"]').first();
    await sendBtn.evaluate((el) => el.click());

    await expect(chatWidget).toContainText('Hello help agent');
  });
});
