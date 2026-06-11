import { test, expect } from './fixtures';

test.describe('Mock Data Audit - Chat & Referrals', () => {
  test('help chat should interact with /api/chat instead of mock timeout', async ({ page }) => {
    await page.goto('/ui/setup.html');

    // Open chat
    const helpBtn = page.locator('#ohc-help-btn');
    await expect(helpBtn).toBeVisible();
    await helpBtn.click();

    // Check if the chat overlay is visible
    const overlay = page.locator('#ohc-help-chat-overlay');
    await expect(overlay).toBeVisible();

    // Enter text and send
    const input = page.locator('#ohc-help-input');
    await input.fill('getting started');

    // We want to intercept the request to /api/chat to ensure it's made
    const chatPromise = page.waitForRequest(req => req.url().includes('/api/chat') && req.method() === 'POST');

    const sendBtn = page.locator('#ohc-help-send');
    await sendBtn.click();

    // Await the API call
    const request = await chatPromise;
    expect(request.url()).toContain('/api/chat');

    // Assuming backend is live, we expect a response or we just assert that the request was made.
    // The previous mock was a setTimeout. Now it makes a real network request.
  });

  test('dashboard referral generation should hit /api/v1/growth/referrals/generate', async ({ page }) => {
    await page.goto('/ui/dashboard.html');

    const generateBtn = page.locator('#generate-invite-btn');
    await expect(generateBtn).toBeVisible();

    const generatePromise = page.waitForRequest(req => req.url().includes('/api/v1/growth/referrals/generate') && req.method() === 'POST');

    await generateBtn.click();

    const request = await generatePromise;
    expect(request.url()).toContain('/api/v1/growth/referrals/generate');
  });
});

  test('store manager chat should interact with /api/store-manager instead of mock timeout', async ({ page }) => {
    await page.goto('/store-manager');

    // Wait for the chat input
    const chatInput = page.getByPlaceholder('Tell me what to do...');
    await expect(chatInput).toBeVisible();

    await chatInput.fill('inventory');
    const sendBtn = page.locator('button').filter({ has: page.locator('svg') }).last();

    const responsePromise = page.waitForResponse(res => res.url().includes('/api/store-manager') && res.request().method() === 'POST');
    await page.keyboard.press('Enter');

    const response = await responsePromise;
    expect(response?.ok()).toBeTruthy();

    // Verify it responds with the mock action items mapped to the real backend
    await expect(page.getByText('Checking inventory. You are running low on Vanilla Extract. Should I order more?')).toBeVisible();

    // Verify actions appear
    const actionBtn = page.getByRole('button', { name: 'Yes, order 2 bottles' });
    await expect(actionBtn).toBeVisible();

    const actionResponsePromise = page.waitForResponse(res => res.url().includes('/api/store-manager') && res.request().method() === 'POST');
    await actionBtn.click();

    const actionResponse = await actionResponsePromise;
    expect(actionResponse?.ok()).toBeTruthy();

    // Fallback or explicit response should be shown
    await expect(page.getByText('I can help with that. Give me a moment to process.').last()).toBeVisible();
  });
