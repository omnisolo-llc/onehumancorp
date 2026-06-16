import { test, expect } from './fixtures';

test.describe('Twilio Webhook Carlos Quoting Flow', () => {
  test('Receiving SMS request generates an Action Required Draft Quote in feed', async ({ page, loginAs, adminUser }) => {
    // 1. Simulate inbound SMS via Twilio Webhook
    const res = await page.request.post('/api/v1/webhooks/twilio', {
      headers: {
        'Content-Type': 'application/x-www-form-urlencoded'
      },
      data: 'From=+15551234567&To=+15557654321&Body=My%20sink%20is%20leaking%2C%20can%20you%20come%20today%3F&NumMedia=0'
    });
    expect(res.status()).toBe(200);

    // 2. Wait for the agent to process the event
    await page.waitForTimeout(3000);

    // 3. Login to the mobile UI feed
    await loginAs(page, adminUser);
    await page.goto('/ui/dashboard.html');

    // 4. Verify the Draft Quote Suggestion card is visible in the feed
    await expect(page.getByTestId('quote-draft-card').first()).toBeVisible();

    // 5. Verify card contents
    await expect(page.getByText('Draft Quote')).first().toBeVisible();

    // 6. Tap "Approve & Send"
    const approveBtn = page.getByText('Approve & Send').first();
    await approveBtn.waitFor({ state: 'visible' });
    await approveBtn.click();

    // 7. Optimistic UI update should remove the card from the feed
    await expect(page.getByTestId('quote-draft-card')).toHaveCount(0);
  });
});
