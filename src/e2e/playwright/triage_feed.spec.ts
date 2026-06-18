import { test, expect } from '@playwright/test';

test.describe('Actionable Daily Briefing on Triage Feed', () => {
  const tenantId = 'triage-feed-test-tenant';

  test.beforeEach(async ({ request }) => {
    // Navigate and set local storage for auth
    const authHeaders = {
      'Authorization': 'Bearer test-token',
    };

    // Seed data
    const res = await request.post(`/api/ui/triage/create?tenant_id=${tenantId}`, {
      data: {
        source: 'Instagram DM',
        priority: 'High',
        context: 'Do you make vegan cakes for this Saturday?',
        action_type: 'Draft Reply',
        action_payload: 'Yes we do! That would be $50.'
      },
      headers: authHeaders
    });
    // Ensure the response is ok
  });

  test('should render triage cards, handle decisions and show empty state', async ({ page }) => {
    await page.route('**/api/ui/triage*', async (route) => {
      if (route.request().method() === 'GET') {
        await route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify([
            {
              id: 'test-item-1',
              tenant_id: tenantId,
              source: 'Instagram DM',
              priority: 'High',
              context: 'Do you make vegan cakes for this Saturday?',
              action_type: 'Draft Reply',
              action_payload: 'Yes we do! That would be $50.',
              created_at: new Date().toISOString()
            }
          ])
        });
      } else if (route.request().method() === 'POST' && route.request().url().includes('action')) {
        await route.fulfill({ status: 200, body: JSON.stringify({ success: true }) });
      } else {
        await route.continue();
      }
    });

    await page.goto('/');

    await page.evaluate((tId) => {
      localStorage.setItem('has_onboarded', 'true');
      localStorage.setItem('tenant_id', tId);
      localStorage.setItem('tenant', tId);
      localStorage.setItem('token', 'test-token');
      localStorage.setItem('user_id', 'test-user');
    }, tenantId);

    await page.goto('/triage');

    // Ensure the Action Card renders correctly
    await expect(page.locator('text=Proposed Action')).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text=Do you make vegan cakes for this Saturday?')).toBeVisible();

    // Verify tap target for button
    const approveBtn = page.getByRole('button', { name: 'Approve & Execute' }).first();
    await expect(approveBtn).toBeVisible();

    const boundingBox = await approveBtn.boundingBox();
    expect(boundingBox?.height).toBeGreaterThanOrEqual(44);
    expect(boundingBox?.width).toBeGreaterThanOrEqual(44);

    // Click the approve button (Approve & Execute)
    await approveBtn.click();

    // Ensure the card is dismissed (optimistic UI update)
    await expect(page.locator('text=Do you make vegan cakes for this Saturday?')).not.toBeVisible();

    // Check if empty state is visible
    await expect(page.locator("text=All caught up! You're a hero.")).toBeVisible();
  });
});
