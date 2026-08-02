import { test, expect } from '@playwright/test';

test.describe('Native Omnichannel Chat Web Widget', () => {
  test('Customer can open widget and send a message', async ({ page }) => {
    // Navigate to a page where the widget is expected to be loaded
    await page.goto('/demo-storefront');

    // Wait for the floating action button to appear and click it
    const widgetFab = page.locator('#ohc-chat-widget-fab');
    await widgetFab.waitFor({ state: 'visible' });
    await widgetFab.click();

    // Verify the widget container expands
    const widgetContainer = page.locator('#ohc-chat-widget-container');
    await expect(widgetContainer).toBeVisible();

    // Fill in a message and send it
    const messageInput = page.locator('#ohc-chat-widget-input');
    await messageInput.fill('Hello, I need help with my cake order.');
    const sendButton = page.locator('#ohc-chat-widget-send');
    await sendButton.click();

    // Verify the message appears in the chat stream
    const messageBubble = page.locator('.ohc-chat-message:has-text("Hello, I need help with my cake order.")');
    await expect(messageBubble).toBeVisible();
  });

  test('Widget initializes with custom color from config', async ({ page }) => {
    await page.goto('/demo-storefront');
    const widgetFab = page.locator('#ohc-chat-widget-fab');
    await widgetFab.waitFor({ state: 'visible' });
    // This assumes the config dictates a specific hex color #0066FF
    await expect(widgetFab).toHaveCSS('background-color', 'rgb(0, 102, 255)');
  });

  test('Widget can switch to offline state when disconnected', async ({ page }) => {
    await page.goto('/demo-storefront');
    // Simulate network disconnect
    await page.context().setOffline(true);

    const widgetFab = page.locator('#ohc-chat-widget-fab');
    await widgetFab.click();

    // An offline banner should appear indicating degraded state
    const offlineBanner = page.locator('.ohc-chat-offline-banner');
    await expect(offlineBanner).toBeVisible();
    await page.context().setOffline(false);
  });

  test('Customer receives a simulated AI agent reply', async ({ page }) => {
    await page.goto('/demo-storefront');

    const widgetFab = page.locator('#ohc-chat-widget-fab');
    await widgetFab.click();

    const messageInput = page.locator('#ohc-chat-widget-input');
    await messageInput.fill('What are your hours?');
    await page.locator('#ohc-chat-widget-send').click();

    // The AI agent should reply shortly
    const agentReply = page.locator('.ohc-chat-message.agent:has-text("We are open 9 AM to 5 PM")');
    await expect(agentReply).toBeVisible({ timeout: 5000 });
  });

  test('Widget respects touch targets for mobile (375px)', async ({ page }) => {
    // Set viewport to mobile size
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/demo-storefront');

    const widgetFab = page.locator('#ohc-chat-widget-fab');
    await widgetFab.waitFor({ state: 'visible' });

    // Verify touch target is at least 44x44
    const box = await widgetFab.boundingBox();
    expect(box?.width).toBeGreaterThanOrEqual(44);
    expect(box?.height).toBeGreaterThanOrEqual(44);
  });
});
