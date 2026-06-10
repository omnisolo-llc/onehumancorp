import { expect } from './fixtures';
import { test } from '@playwright/test';

test('twilio omnichannel webhook and triage', async ({ page, request }) => {
  const webhookUrl = '/api/v1/webhooks/twilio';

  // 1. Send Twilio Webhook POST (simulate an incoming WhatsApp message)
  const body = new URLSearchParams({
    'From': 'whatsapp:+15551234567',
    'To': 'whatsapp:+15559876543',
    'Body': 'Hello from Playwright Twilio Test',
  });

  const response = await request.post(webhookUrl, {
    headers: {
      'Content-Type': 'application/x-www-form-urlencoded',
    },
    data: body.toString(),
  });

  expect(response.status()).toBe(200);

  // 2. Navigate to Dashboard -> Triage
  await page.setViewportSize({ width: 1440, height: 900 });
  await page.goto('/login');
  await page.fill('input[placeholder="Email or Username"]', 'Maya');
  await page.getByRole('button', { name: 'Log In' }).click();

  await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

  // 3. Look for the Triage Action for this message
  await expect(page.getByText('Hello from Playwright Twilio Test').first()).toBeVisible({ timeout: 20000 });

  const messageCard = page.locator('.app-card', { hasText: 'Hello from Playwright Twilio Test' }).first();
  await expect(messageCard).toBeVisible();

  const approveButton = messageCard.getByRole('button', { name: 'Approve' });
  if (await approveButton.isVisible()) {
    await approveButton.click();
    await expect(approveButton).not.toBeVisible({ timeout: 10000 });
  }
});
