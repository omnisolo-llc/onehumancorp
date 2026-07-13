import { test, expect } from '@playwright/test';
import { adminPage } from '../fixtures';

test.describe('Agentic Automated Quoting & Proposal Generator', () => {

  test('draft quote with AI from inbox message on mobile view', async ({ browser }) => {
    // 1. Context setup and login
    const context = await browser.newContext();
    const page = await adminPage(context);

    // Mobile viewport (375px)
    await page.setViewportSize({ width: 375, height: 812 });

    const tenantId = 'e2e-tenant'; // Fallback / standard e2e tenant id
    const uniqueCustomer = `customer-${Date.now()}`;

    // 2. Inject a test message into the real UI/DB using a webhook to simulate receiving it
    const webhookResponse = await page.request.post('/api/v1/omnichannel/webhook', {
      data: {
        tenant_id: tenantId,
        channel: 'instagram_dm',
        sender_id: uniqueCustomer,
        message: 'Can you fix a leaky pipe tomorrow?',
      }
    });

    expect(webhookResponse.ok()).toBeTruthy();

    // 3. Go to the owner feed
    await page.goto(`/`);
    await page.waitForLoadState('networkidle');

    // Wait for the quote draft card to appear in the Action Required feed.
    // The Closer Agent (message_triage_worker) should process the message, generate a quote, and place it here.
    const quoteCard = page.getByTestId('quote-draft-card').first();
    await expect(quoteCard).toBeVisible({ timeout: 20000 });

    // 4. Click Approve & Send
    const approveBtn = quoteCard.getByTestId('feed-approve-btn');
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // Wait for card to visually process/disappear
    await expect(quoteCard).not.toBeVisible({ timeout: 15000 });

    // 5. Navigate to quotes page to verify it's SENT and has a payment link
    // E2E test runs with Next.js frontend or Tauri, but backend sets status = 'SENT'
    await page.goto(`/quotes`);
    await page.waitForLoadState('networkidle');

    // Wait for the table row that has "SENT" to be visible
    await expect(page.locator('text=SENT').first()).toBeVisible({ timeout: 15000 });

    // In our quotes page, maybe it renders a stripe link column or a link button.
    // Let's assert there's a link pointing to checkout.stripe.com
    const stripeLinks = page.locator('a[href*="checkout.stripe.com"]');
    expect(await stripeLinks.count()).toBeGreaterThan(0);
  });
});