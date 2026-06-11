import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('twilio_whatsapp integration handles incoming WhatsApp message and replies', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);

  const connectRes = await request.post('/api/agents/settings/integrations/whatsapp', {
    data: {
      base_url: 'https://api.twilio.com',
      bot_token: 'test_sid',
      api_token: 'test_token',
      from_phone: '+1234567890'
    }
  });

  const webhookData = new URLSearchParams({
    From: 'whatsapp:+19998887777',
    To: 'whatsapp:+1234567890',
    Body: 'Hello, I want to order a cake!'
  });

  const webhookRes = await request.post('/api/v1/webhooks/twilio', {
    headers: {
      'Content-Type': 'application/x-www-form-urlencoded'
    },
    data: webhookData.toString()
  });
  expect(webhookRes.ok()).toBeTruthy();

  await page.goto('/inbox');

  await expect(page.locator('text=Hello, I want to order a cake!')).toBeVisible({ timeout: 15000 });
  await expect(page.locator('text=whatsapp')).toBeVisible();

  await currentAppSmoke(page, request, 'twilio_whatsapp');
});
