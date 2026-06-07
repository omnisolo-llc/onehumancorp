import { test, expect } from './fixtures';

test.describe('Abandoned Cart Recovery Feed', () => {
  test('should display abandoned cart recovery in agent activity feed and allow approval', async ({ page }) => {
    // 1. Log in and go to dashboard
    await page.goto('/dashboard');

    // 2. Wait for the feed to load
    await page.waitForLoadState('networkidle');

    // 3. Locate the 'Abandoned cart recovery: 10% discount for Sarah' card that is seeded by e2e-seed.sql
    // It should have 'Abandoned Carts: 3' and 'Potential Revenue: $120.00'
    const approvalCard = page.locator('div.glassmorphism').filter({ hasText: 'Abandoned cart recovery: 10% discount for Sarah' });
    await expect(approvalCard).toBeVisible();

    // Verify context details are visible
    await expect(approvalCard).toContainText('Abandoned Carts:');
    await expect(approvalCard).toContainText('3');
    await expect(approvalCard).toContainText('Potential Revenue:');
    await expect(approvalCard).toContainText('$120.00');

    // 4. Click Approve button
    const approveBtn = approvalCard.getByRole('button', { name: 'Approve' }).first();
    await expect(approveBtn).toBeVisible();

    // Click and expect the card to disappear optimistically or after network request
    await approveBtn.click();
    await expect(approvalCard).toBeHidden({ timeout: 10000 });

    // 5. Navigate to Activity Feed tab
    await page.getByRole('button', { name: 'Activity Feed' }).click();

    // Wait for the tab to load
    await page.waitForTimeout(1000); // Give it a moment to fetch activity if needed

    // 6. Verify that it appears in the Activity Feed as Approved
    const activityCard = page.locator('div.glassmorphism').filter({ hasText: 'Abandoned cart recovery: 10% discount for Sarah' });
    await expect(activityCard).toBeVisible();
    await expect(activityCard).toContainText('APPROVED');
  });
});
