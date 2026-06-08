import { test, expect } from '@playwright/test';

test.describe('Dashboard Actionable Feed', () => {
  // Use a 375px mobile viewport for the mobile-first feature
  test.use({ viewport: { width: 375, height: 812 } });

  test('should display unified agent feed and approve an action on mobile (375px)', async ({ page }) => {
    // Mock the approvals feed
    await page.route('/api/agents/approvals*', async route => {
      const url = route.request().url();
      if (url.includes('activity')) {
         await route.fulfill({ json: { pending_approvals: [{ id: '1', department: 'sales', description: 'Draft quote for Plumbing Fix', status: 'Approved', action_risk: 'HIGH' }] } });
      } else {
         await route.fulfill({ json: { pending_approvals: [{
           id: '1',
           department: 'sales',
           description: 'Draft quote for Plumbing Fix',
           status: 'DRAFT',
           action_risk: 'HIGH',
           payload: {
             feature_type: 'quote_draft',
             service: 'Plumbing Fix',
             customer_inquiry: 'I need a quote',
             suggested_price: 250.0,
             scope: 'Standard materials',
             suggested_time: 'Tomorrow at 2 PM'
           }
         }] } });
      }
    });

    await page.route('/api/agents/approvals/1', async route => {
      await route.fulfill({ json: { success: true } });
    });

    await page.goto('/dashboard');

    // Check main dashboard components are present (to ensure page loads properly)
    await expect(page.getByRole('heading', { name: 'Business Analytics' })).toBeVisible();
    await expect(page.locator('text="Operations Map"').first()).toBeVisible();
    await expect(page.locator('text="Recent Orders"')).toBeVisible();

    // Verify the feed is rendered within a 375px constraint (testing the max-width)
    const feedSection = page.locator('section[aria-label="Unified Agent Feed"]');
    await expect(feedSection).toBeVisible();

    // Verify the "Proposals" tab is visible and shows at least 1 proposal
    const proposalsTab = page.locator('button', { hasText: 'Proposals' });
    await expect(proposalsTab).toBeVisible();

    // Make sure the new Quote Draft card appears
    const draftCard = page.getByTestId('draft-quote-card').first();
    await expect(draftCard).toBeVisible();

    // Find the approve button inside the proposal
    const approveButton = page.getByRole('button', { name: 'Approve & Send Proposal' }).first();
    await expect(approveButton).toBeVisible();

    // Ensure the touch target meets the 44x44px mobile requirement
    const box = await approveButton.boundingBox();
    expect(box?.width).toBeGreaterThanOrEqual(44);
    expect(box?.height).toBeGreaterThanOrEqual(44);

    // Approve the action
    await approveButton.click();
  });
});
