import { test, expect } from '@playwright/test';

test.describe('Staff Mesh E2E (Maya the Baker)', () => {
  const TENANT_ID = 'e2e_maya_bakery';

  test.beforeEach(async ({ page }) => {
    // Set up local storage mock for tenant id
    await page.goto('/team');
    await page.evaluate((tenantId) => {
      localStorage.setItem('tenant_id', tenantId);
    }, TENANT_ID);
    await page.reload();
  });

  test('Maya adds a staff member and copies invite link', async ({ page }) => {
    // Navigate to team page
    await page.goto('/team');

    // Currently we'll just check that it loads
    await expect(page.locator('h1')).toContainText('Your Team');

    // In a real implementation we would click a + button
    // The placeholder doesn't have it yet, so we just check the page exists
  });

  test('Maya visits the staff page', async ({ page }) => {
    await page.goto('/team/staff');
    await expect(page.locator('body')).toContainText('Staff Page Placeholder');
  });
});
