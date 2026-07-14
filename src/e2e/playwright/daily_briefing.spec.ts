import { test, expect } from '@playwright/test';

test.describe('Actionable Daily Briefing on Dashboard', () => {
  const tenantId = 'daily-brief-test-tenant';
  let triageItemId: string;

  test.beforeEach(async ({ request }) => {
    // Navigate and set local storage for auth
    const authHeaders = {
      'Authorization': 'Bearer test-token',
    };

    // Seed data
    const res = await request.post(`/api/ui/triage/create?tenant_id=${tenantId}`, {
      data: {
        source: 'Decision Assistant',
        priority: 'High',
        context: '3 new custom cake inquiries',
        action_type: 'Draft Replies'
      },
      headers: authHeaders
    });
    // Ensure the response is ok
    expect(res.ok()).toBeTruthy();
  });

  test('should render morning briefing text and actionable daily brief cards', async ({ page }) => {
    await page.goto('/');

    await page.evaluate((tId) => {
      localStorage.setItem('has_onboarded', 'true');
      localStorage.setItem('tenant_id', tId);
      localStorage.setItem('tenant', tId);
      localStorage.setItem('token', 'test-token');
      localStorage.setItem('user_id', 'test-user');
    }, tenantId);

    await page.goto('/dashboard');

    // Expect the Morning Briefing text block to load
    await expect(page.getByTestId('onboarding-welcome-card')).toBeVisible({ timeout: 10000 });

    // Ensure the Action Card renders correctly
    await expect(page.locator('text=Suggested Action')).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text=3 new custom cake inquiries')).toBeVisible();

    // Verify minimum tap target for button
    const approveBtn = page.getByRole('button', { name: 'Draft Replies' });
    await expect(approveBtn).toBeVisible();

    const boundingBox = await approveBtn.boundingBox();
    expect(boundingBox?.height).toBeGreaterThanOrEqual(44);
    expect(boundingBox?.width).toBeGreaterThanOrEqual(44);

    // Click the approve button (Draft Replies)
    await approveBtn.click();

    // Ensure the card is dismissed (optimistic UI update)
    await expect(page.locator('text=3 new custom cake inquiries')).not.toBeVisible();
  });
});
