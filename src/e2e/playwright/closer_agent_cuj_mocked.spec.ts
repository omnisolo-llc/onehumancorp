import { test, expect } from '@playwright/test';

test.describe('Closer Agent CUJ (Mocked)', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('full closer agent flow: intake -> draft -> approve -> follow-up', async ({ page }) => {
    // 1. Mock the agent feed to have a Draft Quote item
    const quoteId = '123e4567-e89b-12d3-a456-426614174000';
    await page.route('**/api/agent-feed', async route => {
      const json = {
        items: [
          {
            id: 'feed-1',
            tenant_id: 'tenant-1',
            event_source: 'instagram_dm',
            lifecycle_state: 'PENDING_APPROVAL',
            created_at: new Date().toISOString(),
            updated_at: new Date().toISOString(),
            context_payload: {
              customer_name: 'Carlos Handyman',
              context: 'Fix leaky sink inquiry'
            },
            proposed_action: {
              action_type: 'Draft Quote',
              quote_id: quoteId
            }
          }
        ]
      };
      await route.fulfill({ json });
    });

    await page.goto('/feed');
    await expect(page.locator('text=Drafted Estimate for Carlos Handyman')).toBeVisible();

    // 2. Click Review Estimate and navigate to quote review screen
    const reviewBtn = page.locator('button', { hasText: 'Review Estimate' });

    // Mock the quote details API
    await page.route(`**/api/quotes/${quoteId}`, async route => {
      const json = {
        id: quoteId,
        customer_id: 'cust-1',
        status: 'DRAFT',
        total_amount_cents: 15000,
        required_deposit_cents: 5000,
        line_items: [
          { id: 'li-1', description: 'Sink Repair', unit_price_cents: 15000, quantity: 1, is_optional: false }
        ]
      };
      await route.fulfill({ json });
    });

    await reviewBtn.click();
    await expect(page).toHaveURL(new RegExp(`/quotes/${quoteId}`));
    await expect(page.locator('text=Sink Repair')).toBeVisible();
    await expect(page.locator('text=$150.00')).toBeVisible();

    // 3. Approve and Send Quote
    await page.route(`**/api/quotes/${quoteId}/approve`, async route => {
      const json = {
        id: quoteId,
        status: 'ACCEPTED',
        total_amount_cents: 15000,
        required_deposit_cents: 5000,
        stripe_payment_link: 'https://checkout.stripe.com/pay/mock_link'
      };
      await route.fulfill({ json });
    });

    const approveBtn = page.locator('button', { hasText: 'Approve & Send Quote' });
    await approveBtn.click();

    await expect(page.locator('text=ACCEPTED')).toBeVisible();
    await expect(page.locator('text=mock_link')).toBeVisible();

    // 4. Simulate Follow-up Card Appearance
    await page.route('**/api/agent-feed', async route => {
      const json = {
        items: [
          {
            id: 'feed-2',
            tenant_id: 'tenant-1',
            event_source: 'deposit_follow_up',
            lifecycle_state: 'PENDING_APPROVAL',
            created_at: new Date().toISOString(),
            updated_at: new Date().toISOString(),
            context_payload: {
              customer_name: 'Carlos Handyman',
              amount_cents: 15000
            },
            proposed_action: {
              action_type: 'Draft Follow-up',
              draft_reply: 'Hi Carlos, just following up on the estimate...',
              quote_id: quoteId
            }
          }
        ]
      };
      await route.fulfill({ json });
    });

    await page.goto('/feed');
    await expect(page.locator('text=Unpaid Deposit: Carlos Handyman')).toBeVisible();
    await expect(page.locator('button', { hasText: 'Send Follow-up' })).toBeVisible();
  });
});
