import { test, expect } from '@playwright/test';
import * as path from 'path';
import * as fs from 'fs';

test.describe('Onboarding Chat CUJ Flow', () => {

  test.beforeEach(async ({ page }) => {
    await page.addInitScript(() => {
      window.localStorage.clear();
      window.sessionStorage.clear();
    });
  });

  test('Completes the conversational onboarding flow using the real API backend', async ({ page }) => {
    // Setup Tauri mock routes that point to actual HTML files.
    // The HTML file makes requests to the real backend running on http://127.0.0.1:18789
    const workspaceRoot = process.env.TEST_WORKSPACE
        ? path.join(process.env.TEST_SRCDIR || process.cwd(), process.env.TEST_WORKSPACE)
        : process.cwd();

    const tauriUiDir = path.join(workspaceRoot, 'src/ui/tauri/src/ui');

    await page.route('http://mock/setup.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    await page.route('http://mock/success.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'success.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });

    // Go to the onboarding setup
    await page.goto('http://mock/setup.html');

    // Wait for the container to be visible
    const container = page.locator('.container');
    await expect(container).toBeVisible({ timeout: 30000 });

    // Since we made it the default active step, we should be in the chat step
    await expect(page.getByRole('heading', { name: "Setup Assistant" })).toBeVisible();

    // The chat assistant should have an initial message
    const chatMessages = page.locator('#chat-messages');
    await expect(chatMessages).toContainText('What do you want to build or manage today?');

    // Make sure we have the 44x44 image upload button
    const uploadBtn = page.locator('#chat-upload-btn');
    const uploadBtnBox = await uploadBtn.boundingBox();
    expect(uploadBtnBox?.width || 0).toBeGreaterThanOrEqual(44);
    expect(uploadBtnBox?.height || 0).toBeGreaterThanOrEqual(44);

    // Send the first message
    const chatInput = page.locator('#chat-input');
    await chatInput.fill("I am a plumber fixing pipes and stuff.");

    // We expect the send button to have a height >= 44px
    const sendBtn = page.locator('#chat-send-btn');
    const sendBtnBox = await sendBtn.boundingBox();
    expect(sendBtnBox?.height || 0).toBeGreaterThanOrEqual(44);

    await sendBtn.click();

    // Check that the user message appears
    await expect(chatMessages).toContainText('YouI am a plumber fixing pipes and stuff.');
    // Check that the assistant replies (via the real backend fallback)
    await expect(chatMessages).toContainText('Great! Could you provide an example photo or a little more detail about what you sell?');

    // Send the second message to trigger `is_complete = true`
    await chatInput.fill("I fix leaky pipes and install faucets.");
    await sendBtn.click();

    await expect(chatMessages).toContainText('YouI fix leaky pipes and install faucets.');
    await expect(chatMessages).toContainText("Give me a minute... I'm building your business.");

    // It should automatically transition to show the Ready to Launch sliding summary card
    await expect(page.getByRole('heading', { name: "Ready to Launch" })).toBeVisible({ timeout: 10000 });

    const approvalDetails = page.locator('#approval-details');
    // Ensure the intake correctly derived the type/products from the fallback or real API
    await expect(approvalDetails).toContainText('Business Name:');

    const approveBtn = page.locator('#approve-publish-btn-chat');
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // It should redirect to success.html
    await expect(page.getByRole('heading', { name: "You're all set!" })).toBeVisible({ timeout: 20000 });
  });

});
