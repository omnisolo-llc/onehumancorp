import { test, expect } from './fixtures';

test.describe('Unified Agent Feed Mobile UI', () => {
  test.use({ viewport: { width: 375, height: 667 } });

  test('should display glassmorphism styled action feed on mobile', async ({ page }) => {

    await page.addInitScript(() => {
      localStorage.setItem('tenant_id', 'e2e-tenant');
      localStorage.setItem('user_id', 'e2e-admin-user');
    });

    await page.goto('/dashboard');

    const activityFeedTab = page.locator('button', { hasText: 'Activity Feed' });
    await expect(activityFeedTab).toBeVisible();
    await activityFeedTab.click();

    // Verify "The Manager" and "The Promoter" aliases are shown from the seeded activity data
    await expect(page.getByText('The Manager').first()).toBeVisible();
    await expect(page.getByText('processed 3 orders').first()).toBeVisible();

    await expect(page.getByText('The Promoter').first()).toBeVisible();
    await expect(page.getByText('paused ads for Sold Out Vegan Cake').first()).toBeVisible();

    // Verify new interaction buttons
    const undoButton = page.locator('button', { hasText: 'Undo' }).first();
    await expect(undoButton).toBeVisible();

    // Check minimal touch target 44x44
    const undoBox = await undoButton.boundingBox();
    expect(undoBox?.height).toBeGreaterThanOrEqual(44);

    const seeDetailsButton = page.locator('button', { hasText: 'See Details' }).first();
    await expect(seeDetailsButton).toBeVisible();

    const detailsBox = await seeDetailsButton.boundingBox();
    expect(detailsBox?.height).toBeGreaterThanOrEqual(44);
  });
});
