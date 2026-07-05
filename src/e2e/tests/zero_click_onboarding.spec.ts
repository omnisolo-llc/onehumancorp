import { test, expect } from '@playwright/test';
import * as path from 'path';
import * as fs from 'fs';

test.describe('Zero-Click Onboarding to Agent Feed', () => {
  test.beforeEach(async ({ page }) => {
    const workspaceRoot = process.env.TEST_WORKSPACE
        ? path.join(process.env.TEST_SRCDIR || process.cwd(), process.env.TEST_WORKSPACE)
        : process.cwd();

    const tauriUiDir = path.join(workspaceRoot, 'src/ui/tauri/src/ui');

    await page.route('http://mock/setup.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    await page.route('http://mock/api/tooltips', async route => {
        await route.fulfill({ contentType: 'application/json', body: "{}" });
    });
    await page.route('**/success.html*', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'success.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });
  });

  test('User completes chat onboarding and sees welcome card on feed', async ({ page }) => {
    // Navigate to the setup route
    await page.goto('http://mock/setup.html');

    // Make sure we're on a mobile viewport
    await page.setViewportSize({ width: 375, height: 812 });

    // Click Conversational Setup
    const conversationalSetupBtn = page.locator('text=Conversational Setup').first();
    await expect(conversationalSetupBtn).toBeVisible();
    await conversationalSetupBtn.click();

    // Wait for chat input to be visible
    const chatInput = page.locator('#chat-input');
    await expect(chatInput).toBeVisible();

    // Type a simple sentence and press Enter
    await page.route('**/api/onboarding/start_zero_click', async route => {
        await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ organization_id: "test-org" }) });
    });
    await page.route('**/api/onboarding/chat', async route => {
        await route.fulfill({
            status: 200,
            contentType: 'application/json',
            body: JSON.stringify({ reply: "Give me a minute... I'm building your business.", is_complete: true, intake_data: { business_name: 'test', categories: ['test'], location: 'test', target_audience: 'test', initial_products: [] } })
        });
    });
    await page.route('**/api/onboarding/start', async route => {
        await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ organization_id: "test-org" }) });
    });
    await chatInput.fill('I run a mobile dog grooming service in Austin');
    await page.locator('#chat-send-btn').click();

    await expect(page.getByRole('heading', { name: /You're Live!/ })).toBeVisible({ timeout: 60000 });

    // Check horizontal scroll by verifying document width equals window innerWidth
    const hasHorizontalScroll = await page.evaluate(() => {
        return document.documentElement.scrollWidth > window.innerWidth;
    });
    expect(hasHorizontalScroll).toBeFalsy();
  });

  test('Conversational Setup prevents empty submissions', async ({ page }) => {
    await page.goto('http://mock/setup.html');

    // Click Conversational Setup
    const conversationalSetupBtn = page.locator('text=Conversational Setup').first();
    await expect(conversationalSetupBtn).toBeVisible();
    await conversationalSetupBtn.click();

    // Ensure input is empty and send
    const chatInput = page.locator('#chat-input');
    await expect(chatInput).toBeVisible();
    await chatInput.fill('');
    await page.locator('#chat-send-btn').click();

    // Message shouldn't appear in chat history
    const userMessages = page.locator('.chat-message.user');
    await expect(userMessages).toHaveCount(0);
  });

  test('Conversational Setup opens image upload input when toggled', async ({ page }) => {
    await page.goto('http://mock/setup.html');

    // Click Conversational Setup
    const conversationalSetupBtn = page.locator('text=Conversational Setup').first();
    await expect(conversationalSetupBtn).toBeVisible();
    await conversationalSetupBtn.click();

    // The image container should be hidden by default
    const imageContainer = page.locator('#chat-image-container');
    await expect(imageContainer).toBeHidden();

    // Click the toggle button
    const uploadBtn = page.locator('#chat-upload-btn');
    await uploadBtn.click();

    // Image container should now be visible
    await expect(imageContainer).toBeVisible();
  });

  test('Conversational Setup maintains history after reload', async ({ page }) => {
    await page.goto('http://mock/setup.html');

    // Start conversational flow
    const conversationalSetupBtn = page.locator('text=Conversational Setup').first();
    await expect(conversationalSetupBtn).toBeVisible();
    await conversationalSetupBtn.click();

    // Type a message
    const chatInput = page.locator('#chat-input');
    await expect(chatInput).toBeVisible();
    await chatInput.fill('This is a test message to ensure history persistence.');
    await page.route('**/api/onboarding/chat', async route => {
        await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ reply: "Hello!" }) });
    });
    await page.locator('#chat-send-btn').click();

    // Wait for the message to appear
    const userMessages = page.locator('.chat-message.user');
    await expect(userMessages).toHaveCount(1);

    // Ensure draft save occurs
    await page.waitForTimeout(1000);

    // Reload page
    await page.reload();

    // The step and chat history should have persisted
    await expect(page.locator('#step-chat')).toBeVisible();
    await expect(page.locator('.chat-message.user').first()).toContainText('This is a test message to ensure history persistence.');
  });

  test('Conversational Setup renders user messages correctly', async ({ page }) => {
    await page.goto('http://mock/setup.html');

    // Start conversational flow
    const conversationalSetupBtn = page.locator('text=Conversational Setup').first();
    await expect(conversationalSetupBtn).toBeVisible();
    await conversationalSetupBtn.click();

    // Type a message
    const chatInput = page.locator('#chat-input');
    await expect(chatInput).toBeVisible();
    await chatInput.fill('Testing chat bubble formatting');
    await page.route('**/api/onboarding/chat', async route => {
        await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ reply: "Hello!" }) });
    });
    await page.locator('#chat-send-btn').click();

    // Check message wrapper layout
    const lastUserMessage = page.locator('.chat-message.user').last();
    await expect(lastUserMessage).toBeVisible();

    const senderTitle = lastUserMessage.locator('.chat-sender');
    await expect(senderTitle).toHaveText('You');

    const bubbleContent = lastUserMessage.locator('.chat-bubble');
    await expect(bubbleContent).toHaveText('Testing chat bubble formatting');

  });
});
