import { test } from "./fixtures";
import { expect } from "@playwright/test";

test.describe('Twilio WhatsApp Webhook Flow', () => {
  test('receives whatsapp webhook, creates draft, and approves', async ({ page, request }) => {
    // Note: The Playwright tests run against a live local dev server where we can hit the backend directly.
    const baseUrl = process.env.OHC_BASE_URL || 'http://localhost:8080';

    // Simulate a Twilio Webhook POST request for an incoming WhatsApp message
    const webhookRes = await request.post(`${baseUrl}/api/v1/webhooks/twilio`, {
      headers: {
        'Content-Type': 'application/x-www-form-urlencoded',
        'X-Twilio-Signature': 'invalid_for_test_but_ignored_if_test_token',
      },
      data: new URLSearchParams({
        'From': 'whatsapp:+19999999999',
        'To': 'whatsapp:+18888888888',
        'Body': 'Hello, I need help with my cake order!',
      }).toString(),
    });

    expect(webhookRes.ok()).toBeTruthy();

    // Now check the triage feed/dashboard for the new draft
    await page.goto(`${baseUrl}/`);

    // Wait for the ambassador reply draft to appear
    await expect(page.locator('text=New whatsapp message from')).toBeVisible({ timeout: 15000 });

    // Click 1-Tap Approve
    const approveBtn = page.locator('button:has-text("✨ 1-Tap Approve")').first();
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // It should be removed from the pending list after approval
    await expect(page.locator('text=New whatsapp message from')).toBeHidden({ timeout: 10000 });
  });
});
