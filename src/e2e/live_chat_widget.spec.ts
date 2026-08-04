import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('current app smoke test', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'live_chat_widget');
});

test.describe('Embeddable Live Web Widget E2E', () => {
  test.beforeEach(async ({ page }) => {
    // Starts from a clean page
    await page.goto('/dashboard');
    // Navigates to the widget demo page (simulating opening a website with the widget)
    await page.goto('/chat-widget-demo');
  });

  test('should verify widget demo page content', async ({ page }) => {
    await expect(page.locator('text=Your Custom Website')).toBeVisible();
  });

  test('should open chat widget via floating button', async ({ page }) => {
    const openBtn = page.getByTestId('open-live-chat-btn');
    await expect(openBtn).toBeVisible();
    await openBtn.click();
    await expect(page.locator('text=Live Support')).toBeVisible();
  });

  test('should simulate initial greeting', async ({ page }) => {
    const openBtn = page.getByTestId('open-live-chat-btn');
    await openBtn.click();
    await expect(page.locator('text=Hello! How can we help you today?')).toBeVisible();
  });

  test('should allow user input and check if user message appears', async ({ page }) => {
    const openBtn = page.getByTestId('open-live-chat-btn');
    await openBtn.click();
    const chatInput = page.getByTestId('live-chat-input');
    await chatInput.fill('I have a question about Product 1.');
    const sendBtn = page.getByTestId('live-chat-send');
    await sendBtn.click();
    await expect(page.locator('text=I have a question about Product 1.')).toBeVisible();
  });

  test('should check for simulated agent reply', async ({ page }) => {
    const openBtn = page.getByTestId('open-live-chat-btn');
    await openBtn.click();
    const chatInput = page.getByTestId('live-chat-input');
    await chatInput.fill('I have a question about Product 1.');
    const sendBtn = page.getByTestId('live-chat-send');
    await sendBtn.click();
    await expect(page.locator('text=Thank you for reaching out!')).toBeVisible({ timeout: 5000 });
  });
});
