import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('yield_management');

test.describe('Autonomous Yield Management Engine', () => {
  test('should display yield opportunities and allow approval', async ({ page }) => {
    // Navigate to the Yield Opportunities page
    await page.goto('/yield');

    // Check if the title is correct
    await expect(page.locator('h1', { hasText: 'Yield Opportunities' })).toBeVisible({ timeout: 15000 });

    // Since we're using the e2e-tenant, we should see the mock opportunity
    const oppCard = page.locator('text=Fill Empty Slots');
    await expect(oppCard).toBeVisible();

    // Verify opportunity details
    await expect(page.locator('text=You have 3 empty slots')).toBeVisible();
    await expect(page.locator('text=20% discount')).toBeVisible();
    await expect(page.locator('text=past students')).toBeVisible();

    // Approve the opportunity
    const approveBtn = page.locator('button', { hasText: 'Approve Offer' });
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // Wait for the opportunity to disappear after approval
    await expect(oppCard).not.toBeVisible();
    await expect(page.locator('text=No opportunities at this time.')).toBeVisible();
  });
});
