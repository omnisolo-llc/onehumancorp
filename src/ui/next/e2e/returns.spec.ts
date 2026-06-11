import { test, expect } from '@playwright/test';

test.describe('Omnichannel Returns & Exchange Orchestrator', () => {
  test('Customer initiates return and Owner approves it in Triage', async ({ page }) => {
    // 1. Customer initiates a return
    await page.goto('/returns');

    // Check UI elements
    await expect(page.locator('h1').last()).toContainText('Request a Return');

    // Fill out the form
    await page.fill('input[placeholder="e.g. ORD-12345"]', 'ORD-55555');
    await page.fill('input[placeholder="e.g. PROD-987"]', 'PROD-404');
    await page.fill('input[placeholder="e.g. 4500 for $45.00"]', '4500');

    // Mock the backend API for return initiation
    await page.route('/api/v1/returns/initiate', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ success: true }),
      });
    });

    // Submit
    await page.click('button[type="submit"]');

    // Verify success message
    await expect(page.locator('text=Return Requested!')).toBeVisible();

    // 2. Owner navigates to Triage
    await page.goto('/triage');

    // We'll mock the triage items response so we have the return item
    await page.route('/api/ui/triage**', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify([{
          id: 'return-req-123',
          tenant_id: 'default',
          priority: 'High',
          context: 'Return requested for Order #ORD-55555. Please review and approve restock & refund.',
          action_type: 'DraftForReview',
          action_payload: JSON.stringify({
            feature_type: "return_requested",
            order_id: "ORD-55555",
            product_id: "PROD-404",
            amount_cents: 4500,
            action: "Return & Refund"
          }),
          created_at: new Date().toISOString(),
        }])
      });
    });

    await page.reload();

    // Wait for item to load
    await expect(page.locator('text=Return requested for Order #ORD-55555').first()).toBeVisible();

    // Check payload parsing UI
    await expect(page.locator('text=Amount to Refund')).toBeVisible();
    await expect(page.locator('text=$45.00')).toBeVisible();

    // 3. Mock the decision endpoint
    await page.route('**/api/agents/approvals/return-req-123**', async route => {
      const payload = JSON.parse(route.request().postData() || '{}');
      expect(payload.approved).toBe(true);

      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ success: true }),
      });
    });

    // Approve the return
    await page.click('[data-testid="approve-btn"]');

    // Wait for the action to complete
    await expect(page.getByRole('status')).toContainText('Approved!', { timeout: 10000 });
  });
});
