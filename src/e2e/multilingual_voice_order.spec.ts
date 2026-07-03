import { test, expect } from '@playwright/test';

test.describe('Multilingual Voice Order Interceptor & Agentic KDS', () => {
  test('simulates incoming voice webhook and verifies translated KDS card appears', async ({ request, page }) => {
    const callSid = 'CA_e2e_test_voice_order';
    const fromPhone = '%2B15551234567';
    const toPhone = '%2B15559876543';

    // 1. Initiate Call
    const incomingResponse = await request.post('/api/v1/webhooks/twilio_voice', {
      headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
      data: `CallSid=${callSid}&CallStatus=ringing&From=${fromPhone}&To=${toPhone}`,
    });
    expect(incomingResponse.status()).toBe(200);

    // 2. Simulate User Speech (Gather)
    const gatherResponse = await request.post('/api/v1/webhooks/twilio_voice/gather', {
      headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
      data: `CallSid=${callSid}&SpeechResult=I%20want%20to%20order%202%20halal%20chicken%20plates%20for%20pickup%20at%2012%3A30&From=${fromPhone}&To=${toPhone}`,
    });
    expect(gatherResponse.status()).toBe(200);

    // 3. Complete Call
    const statusResponse = await request.post('/api/v1/webhooks/twilio_voice/status', {
      headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
      data: `CallSid=${callSid}&CallStatus=completed&From=${fromPhone}&To=${toPhone}`,
    });
    expect(statusResponse.status()).toBe(200);

    // 4. Login to the mobile UI (375px viewport)
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/login');
    await page.fill('input[type="email"]', 'admin@ohc.local');
    await page.fill('input[type="password"]', 'admin');
    await page.click('button:has-text("Sign In")');

    // Wait for the feed to load
    await page.waitForURL('/dashboard');
    await page.goto('/feed');

    // 3. Verify the translated order card appears
    // The Twilio webhook might not have actual transcripts in this end-to-end simulated flow unless we mock the DB or voice engine state first.
    // Wait, the acceptance criteria states: "Provide Playwright E2E tests: Simulate the incoming webhook, then have a user log in to the mobile UI (375px viewport), verify the translated order card appears in the feed, and successfully tap the "Mark Ready" button to dismiss it."

    // Wait for the kds order card to be visible in the feed
    const kdsCard = page.locator('[data-testid="kds-order-card"]').first();
    const markReadyBtn = page.locator('[data-testid="approve-kds-order"]').first();

    // Wait until the KDS card is actually visible.
    // We expect the backend task to be processed and surface in the feed.
    await expect(kdsCard).toBeVisible({ timeout: 15000 });

    // Assert the button is visible and tap it
    await expect(markReadyBtn).toBeVisible();
    await markReadyBtn.click();

    // After clicking mark ready, the card should eventually disappear from the feed
    // or enter a loading state
    await expect(kdsCard).not.toBeVisible({ timeout: 10000 });

  });
});
