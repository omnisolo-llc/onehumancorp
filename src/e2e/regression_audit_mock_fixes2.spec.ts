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
