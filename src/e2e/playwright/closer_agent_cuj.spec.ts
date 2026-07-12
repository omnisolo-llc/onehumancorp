import { test, expect } from '../fixtures';

test.describe('Closer Agent CUJ (End-to-End)', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('full closer agent flow: intake -> draft -> approve -> follow-up', async ({ page, adminUser, loginAs }) => {
    // 1. Log in via actual UI E2E test protocol
    await loginAs(page, adminUser);

    // We send a request to the backend directly via API to simulate the webhook or inbox intake.
    // The backend `message_triage_worker` would process this and output a quote draft in the agent feed.
    // But since the worker might be async and we want to reliably test the Closer Agent approval UI flow:

    // First, let's just trigger the quote creation API to get a real quote in the DB
    const res = await page.request.post('/api/v1/quotes/draft_agent', {
      data: {
        tenant_id: 'default_tenant',
        customer_id: '00000000-0000-0000-0000-000000000001',
        inquiry: 'Can you install 3 ceiling fans tomorrow?',
      }
    });
    expect(res.ok()).toBeTruthy();
    const quoteData = await res.json();
    const quoteId = quoteData.id;
    expect(quoteId).toBeDefined();

    // Now push a feed item to the Agent Feed using this quote ID
    const feedRes = await page.request.post('/api/v1/agent-feed', {
      data: {
        event_source: 'instagram_dm',
        context_payload: {
          customer_name: 'Carlos Handyman',
          context: 'Fix leaky sink inquiry'
        },
        proposed_action: {
          action_type: 'Draft Quote',
          quote_id: quoteId
        }
      }
    });
    expect(feedRes.ok()).toBeTruthy();

    await page.goto('/feed');
    // 2. Locate the "Review Estimate" button for this quote draft.
    // Wait for the Agent Feed to load and display our Quote item.
    await expect(page.locator('text=Review Estimate').first()).toBeVisible({ timeout: 15000 });

    const reviewBtn = page.locator('button', { hasText: 'Review Estimate' }).first();
    await reviewBtn.click();

    await expect(page).toHaveURL(new RegExp(`/quotes/${quoteId}`));

    // Wait for the quote data to load in UI
    await expect(page.locator('text=Review Estimate')).toBeVisible({ timeout: 15000 });

    // 3. Approve and Send Quote (Real backend call via Next.js server actions / API)
    page.on('dialog', dialog => dialog.accept());
    const approveBtn = page.locator('button', { hasText: 'Approve & Send Quote' });
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // 4. Assert Quote status and Stripe payment link
    await expect(page.locator('text=SENT')).toBeVisible({ timeout: 15000 });

    // Check that stripe_payment_link is displayed.
    await expect(page.locator('text=checkout.stripe.com')).toBeVisible({ timeout: 15000 });
  });
});
