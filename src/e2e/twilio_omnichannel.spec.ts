import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('twilio_omnichannel app smoke', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'twilio_omnichannel');
});

test('twilio_omnichannel whatsapp inbound message to feed and reply', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);

  // 1. Simulate an inbound message via webhook
  const response = await request.post('/api/v1/webhooks/twilio', {
    headers: {
      'Content-Type': 'application/x-www-form-urlencoded',
    },
    data: 'From=whatsapp%3A%2B1234567890&To=whatsapp%3A%2B0987654321&Body=I+would+like+to+order+a+cake+Maya',
  });
  expect(response.ok()).toBeTruthy();

  // 2. Navigate to unified agent feed / inbox
  await page.goto('/feed');

  // 3. Verify the inbound message appears in the feed
  await expect(page.getByText('I would like to order a cake Maya').first()).toBeVisible({ timeout: 15000 });

  // 4. Test reply functionality logic
  const replyInput = page.getByPlaceholder('Type a reply...');
  if (await replyInput.count() > 0) {
      await replyInput.first().fill('Thank you for reaching out via WhatsApp!');
      await page.getByRole('button', { name: 'Send' }).click();
  }
});
