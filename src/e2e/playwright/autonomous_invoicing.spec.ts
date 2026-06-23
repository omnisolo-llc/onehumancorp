import { test, expect } from '@playwright/test';

test.describe('Autonomous Instant Localized Invoicing', () => {
  const tenantId = 'e2e-tenant'; // Seeded test tenant

  test('should trigger invoice draft, display in agent feed, and approve', async ({ page }) => {
    // 1. User logs in
    await page.goto('/');

    await page.evaluate((tId) => {
      localStorage.setItem('has_onboarded', 'true');
      localStorage.setItem('tenant_id', tId);
      localStorage.setItem('tenant', tId);
      localStorage.setItem('token', 'test-token');
      localStorage.setItem('user_id', 'test-user');
    }, tenantId);

    // Trigger the invoice generation via our test endpoint
    // In a full application, this would be a "Mark Completed" action on a booking,
    // but the backend will simulate the resulting drafted invoice and agent feed entry.
    const res = await page.request.post('/api/invoices/trigger_autonomous_draft', {
      headers: {
        'x-tenant-id': tenantId,
      }
    });
    expect(res.ok()).toBeTruthy();
    const draftData = await res.json();
    const invoiceId = draftData.invoice_id;

    // 2. Go to feed
    await page.goto('/feed');

    // Wait for feed to load
    await expect(page.locator('section[aria-label="Unified Agent Feed"]')).toBeVisible();

    // 3. Verify new invoice card appears in UI
    const card = page.locator(`[data-testid="invoice-draft-card-${invoiceId}"]`);
    await expect(card).toBeVisible({ timeout: 15000 });
    await expect(card).toContainText('Draft Invoice Ready');

    // 4. Click Review & Send
    const reviewBtn = card.getByTestId('review-invoice-btn');
    await expect(reviewBtn).toBeVisible();
    await reviewBtn.click();

    // 5. Verify the translucent modal/glass UI appears
    const modal = page.locator('#invoice-review-modal');
    await expect(modal).toBeVisible();
    await expect(modal).toContainText('Approve & Send via SMS');

    // 6. Click Approve & Send via SMS
    const approveBtn = page.getByTestId('approve-send-sms-btn');
    await approveBtn.click();

    // 7. Verify the invoice card disappears or updates to "Sent"
    await expect(modal).not.toBeVisible({ timeout: 15000 });
    await expect(card).not.toBeVisible({ timeout: 15000 });

    // (Optional) Check the backend to ensure the invoice status is 'sent' and ledger transaction created
    const checkRes = await page.request.get(`/api/invoices/${invoiceId}/status`, {
      headers: {
        'x-tenant-id': tenantId,
      }
    });
    expect(checkRes.ok()).toBeTruthy();
    const checkData = await checkRes.json();
    expect(checkData.status).toBe('sent');
  });
});
