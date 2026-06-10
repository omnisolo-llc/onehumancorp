import { test, expect } from '@playwright/test';

test.describe('Mobile-First Unified Triage Feed', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the triage page and set viewport to 375px mobile
    await page.setViewportSize({ width: 375, height: 667 });

    // Use the dev seed endpoint to prepare data if necessary or just mock the route
    await page.route('/api/ui/triage?tenant_id=default', async route => {
      await route.fulfill({
        json: [
          {
            id: 'triage-1',
            tenant_id: 'default',
            source: 'Instagram DM',
            priority: 'urgent',
            context: '3 customers asked about vegan options overnight.',
            status: 'pending',
            created_at: new Date().toISOString(),
            action_type: 'Draft Reply & Update Policy',
            action_payload: 'Draft: "Yes, we have 3 new vegan options!"'
          },
          {
            id: 'triage-2',
            tenant_id: 'default',
            source: 'Website Booking',
            priority: 'medium',
            context: 'Reschedule request for tomorrow.',
            status: 'pending',
            created_at: new Date().toISOString(),
            action_type: 'Accept Reschedule',
            action_payload: 'Move booking to 2pm.'
          }
        ]
      });
    });

    await page.route('/api/ui/triage/action?tenant_id=default', async route => {
      await route.fulfill({ json: { success: true } });
    });

    await page.goto('/triage');
  });

  test('Shows Morning Briefing and processes items via half-sheet modal', async ({ page }) => {
    // Assert Morning Briefing banner is visible
    await expect(page.locator('text=Morning Briefing:')).toBeVisible();

    // The feed should render triage items vertically.
    // Click the first triage item to open the half-sheet modal.
    const firstItem = page.locator('[data-testid="triage-card-triage-1"]');
    await expect(firstItem).toBeVisible();
    await firstItem.click();

    // Wait for the half-sheet modal to appear
    const modal = page.locator('[data-testid="triage-modal"]');
    await expect(modal).toBeVisible();

    // Assert modal contains details
    await expect(modal.locator('text=Draft Reply & Update Policy')).toBeVisible();

    // Click "Approve & Send"
    const approveBtn = modal.locator('[data-testid="approve-btn"]');
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // Verify optimistic update - first item should disappear
    await expect(firstItem).not.toBeVisible();
    await expect(modal).not.toBeVisible(); // modal closes

    // The next item should now be visible in the feed
    const secondItem = page.locator('[data-testid="triage-card-triage-2"]');
    await expect(secondItem).toBeVisible();
  });
});
