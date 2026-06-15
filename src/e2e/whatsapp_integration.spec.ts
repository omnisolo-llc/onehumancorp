import { test, expect } from './fixtures';

test('whatsapp integration inbound routing', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);

  await page.setViewportSize({ width: 375, height: 812 });

  // 1. Send an inbound WhatsApp message via the Twilio webhook
  const webhookResponse = await request.post('/api/v1/webhooks/twilio', {
    headers: {
      'Content-Type': 'application/x-www-form-urlencoded',
    },
    data: 'From=whatsapp%3A%2B15551234567&To=whatsapp%3A%2B15559876543&Body=Hello%20from%20WhatsApp',
  });
  expect(webhookResponse.status()).toBe(200);

  // 2. Navigate to Inbox or Triage feed to see the message
  await page.goto('/inbox');

  // Wait for message to appear
  await expect(page.locator('text=Hello from WhatsApp')).toBeVisible({ timeout: 15000 });
  await expect(page.locator('text=whatsapp:+15551234567')).toBeVisible();

  // 3. Connect WhatsApp Integration
  await page.goto('/integrations');
  await expect(page.getByRole('heading', { name: 'Tool Integrations' }).first()).toBeVisible({ timeout: 5000 });

  const whatsappCard = page.locator('h3', { hasText: 'WhatsApp Cloud API' }).locator('..');
  const connectButton = whatsappCard.getByRole('button', { name: 'Connect' });
  await connectButton.click();

  const continueButton = page.getByRole('button', { name: 'Continue with Meta' });
  await expect(continueButton).toBeVisible();
  await continueButton.click();

  // Expect success status
  await expect(page.locator('text=WhatsApp Cloud API connected.')).toBeVisible({ timeout: 5000 });
});
