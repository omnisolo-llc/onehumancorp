import { test, expect } from '../../../../e2e/fixtures';

test.describe('Quote Draft Feed UI', () => {
  test('should render quote draft action card and allow review and approval', async ({ page }) => {
    // Navigate to login
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();

    // Ensure dashboard loads
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    const tenantId = await page.evaluate(() => localStorage.getItem('tenant_id') || 'e2e-tenant');

    const uniqueId = `e2e-quote-draft-test-${Date.now()}`;
    const mockQuoteId = `quote-${Date.now()}`;

    const resCreate = await page.request.post(`/api/v1/agent-feed?tenant_id=${tenantId}`, {
      headers: {
        'x-tenant-id': tenantId,
        'x-user-id': 'default'
      },
      data: {
        event_source: 'Sales Agent',
        context_payload: {},
        proposed_action: {
            feature_type: 'quote_draft',
            quote_id: mockQuoteId,
            customer_inquiry: `Inquiry for ${uniqueId}`,
            scope: `Scope for ${uniqueId}`,
            suggested_price: 1500
        }
      }
    });

    expect(resCreate.ok()).toBeTruthy();

    await expect(page.locator(`text=Action Required: Approve Estimate`)).toBeVisible({ timeout: 10000 });
    await expect(page.locator(`text=${uniqueId}`)).toBeVisible({ timeout: 10000 });

    const approveBtn = page.getByTestId('approve-quote-draft').first();
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // Verify successful API request for the approve action. Wait for the feed item state change or optimistic UI disappearance.
    await expect(page.locator(`text=${uniqueId}`)).not.toBeVisible({ timeout: 5000 });
  });
});
