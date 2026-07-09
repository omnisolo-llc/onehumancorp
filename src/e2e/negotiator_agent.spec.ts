import { test, expect } from './fixtures';

test.describe('Negotiator Agent for Automated Quoting and Deposit Collection', () => {
  const tenantId = 'e2e-negotiator-tenant';

  test('Owner dashboard feed shows autonomous quote generated and Stripe link is drafted', async ({ page }) => {
    // 1. Mobile viewport (375px)
    await page.setViewportSize({ width: 375, height: 812 });

    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');

    // 2. We send a webhook that the negotiator agent subscribes to.
    // The webhook payload simulates an omnichannel message that triggers the agent.
    // Must be triggered via the UI or real e2e route if available, but webhooks are external.
    // We will use page.request because webhooks come from outside.
    const res = await page.request.post('/api/v1/omnichannel/webhook', {
      data: {
        tenant_id: tenantId,
        source: 'SMS',
        sender_id: '+15551234567',
        message: 'Need a ceiling fan installed',
      }
    });

    expect(res.status()).toBe(200);

    // Give it time to process and trigger the NegotiatorAgent and ActionRouter.
    await new Promise(resolve => setTimeout(resolve, 5000));

    // 3. Verify the feed item for the quote draft
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');
    const feedItem = page.locator('text=Draft quote').first();
    await expect(feedItem).toBeVisible({ timeout: 15000 });

    // 4. Find the message in inbox
    await page.goto('/inbox');
    await page.waitForLoadState('networkidle');

    await expect(page.locator('text=Need a ceiling fan installed')).toBeVisible();
    await page.locator('text=Need a ceiling fan installed').click();

    // Verify the Stripe link was included in the reply.
    const replyLocator = page.locator('text=https://buy.stripe.com/test_').first();
    await expect(replyLocator).toBeVisible();
  });
});
