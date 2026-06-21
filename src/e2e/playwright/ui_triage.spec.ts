import { test, expect } from '@playwright/test';

test.describe('Owner-Centric AI Work Triage and Unified Feed', () => {
  const tenantId = 'triage-test-tenant';

  test.beforeEach(async ({ request }) => {
    // Navigate and set local storage for auth
    const authHeaders = {
      'Authorization': 'Bearer test-token',
    };

    // Seed mock event via the create endpoint
    const res = await request.post(`/api/triage/create?tenant_id=${tenantId}`, {
      data: {
        source: 'Instagram',
        priority: 'high',
        context: 'Wants a custom 8-inch vegan cake for Saturday',
        action_type: 'Draft Quote',
        action_payload: 'Proposed $45 quote.'
      },
      headers: authHeaders
    });

    // Ensure the response is ok
    expect(res.ok()).toBeTruthy();
  });

  test('should render a single-column responsive feed and approve an item', async ({ page }) => {
    // Emulate mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });

    await page.goto('/');

    await page.evaluate((tId) => {
      localStorage.setItem('has_onboarded', 'true');
      localStorage.setItem('tenant_id', tId);
      localStorage.setItem('tenant', tId);
      localStorage.setItem('token', 'test-token');
      localStorage.setItem('user_id', 'test-user');
    }, tenantId);

    // Navigate to Triage page
    await page.goto('/triage');

    // Wait for the triage feed items to render
    const triageCard = page.locator('text=Wants a custom 8-inch vegan cake for Saturday');
    await expect(triageCard).toBeVisible({ timeout: 10000 });

    // Find the primary button text containing our dynamic acceptance criteria text
    const approveBtn = page.getByRole('button', { name: 'Approve & Send Drafted Quote ($45)' });
    await expect(approveBtn).toBeVisible();

    // Verify touch targets are at least 44x44px for the button
    const boundingBox = await approveBtn.boundingBox();
    expect(boundingBox?.height).toBeGreaterThanOrEqual(44);
    expect(boundingBox?.width).toBeGreaterThanOrEqual(44);

    // Click the approve button
    await approveBtn.click();

    // Ensure optimistic UI hides the item
    await expect(triageCard).not.toBeVisible();

    // Check for success status banner
    await expect(page.locator('text=Approved!')).toBeVisible();
  });
});
