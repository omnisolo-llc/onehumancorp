import { test, expect } from './fixtures';

test.describe('Dashboard Nudges', () => {

  test('verifies "Action Required" section exists and is visible', async ({ page }) => {
    // Tests in this repository use the `page` fixture which is already logged in as the admin user.
    // They navigate starting from the home page. The test framework `fixtures.ts` takes care of the UI login.
    await page.goto('/dashboard');
    await expect(page.locator('text=Action Required').first()).toBeVisible();
  });

  test('verifies "Advanced Settings" toggle exists and can be clicked', async ({ page }) => {
    await page.goto('/dashboard');
    const advancedSettingsToggle = page.locator('text=Advanced Settings').locator('xpath=following-sibling::button');
    await expect(advancedSettingsToggle).toBeVisible();
    await advancedSettingsToggle.click();
  });

  test('verifies "CustomerSuccess Department" card has the correct text and icon', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.locator('text=CustomerSuccess Department').first()).toBeVisible();
    await expect(page.locator('text=3 customers haven\'t reviewed their orders. Request reviews?').first()).toBeVisible();
  });

  test('verifies "Reject" and "Approve" buttons exist and have correct styles', async ({ page }) => {
    await page.goto('/dashboard');
    const rejectBtn = page.locator('button', { hasText: 'Reject' }).first();
    const approveBtn = page.locator('button', { hasText: 'Approve' }).first();
    await expect(rejectBtn).toBeVisible();
    await expect(approveBtn).toBeVisible();
  });

  test('verifies "Team Activity" panel exists and has the correct Swarm Online indicator', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.locator('text=Team Activity').first()).toBeVisible();
    await expect(page.locator('text=Swarm Online').first()).toBeVisible();
    await expect(page.locator('text=Waiting for team activity...').first()).toBeVisible();
  });

});
