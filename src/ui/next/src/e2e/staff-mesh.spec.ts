import { test, expect } from '../../../../e2e/fixtures';

test.describe('Universal Autonomous Staff & Shift Management Mesh', () => {
  test('CUJ: Manager creates staff, staff logs in offline via PIN', async ({ page }) => {
    // Navigate to local API directly to set up origin to allow localstorage modification
    await page.goto('/api/staff');
    await page.evaluate(() => {
      localStorage.setItem('ohc_offline_staff', JSON.stringify([{
        id: 'staff_1',
        name: 'John Connor',
        role: 'Manager',
        pin_hash: '1234'
      }]));
      localStorage.setItem('ohc_offline_events', JSON.stringify([]));
    });

    // Go to the terminal
    await page.goto('/pos/terminal');

    // Wait for staff to sync (since we just created it and terminal fetches on load)
    await page.waitForTimeout(2000);

    // Ensure we are locked
    await expect(page.locator('h1', { hasText: 'Terminal Locked' })).toBeVisible({ timeout: 25000 });

    // Click inside the body to ensure interaction context
    await page.mouse.click(10, 10);
    await page.waitForTimeout(1000);

    // Enter default PIN: 1, 2, 3, 4 (Mocked API sets PIN to 1234)
    await page.getByRole('button', { name: '1', exact: true }).click();
    await page.getByRole('button', { name: '2', exact: true }).click();
    await page.getByRole('button', { name: '3', exact: true }).click();
    await page.getByRole('button', { name: '4', exact: true }).click();

    // Give it a moment to unlock
    await page.waitForTimeout(1000);

    // Verify successful unlock and correct user data shown
    await expect(page.locator('h1', { hasText: 'John Connor' })).toBeVisible({ timeout: 10000 });
    await expect(page.locator('p', { hasText: 'Manager' })).toBeVisible({ timeout: 10000 });

    // Verify role-based UI (Manager should see Reports)
    await expect(page.locator('span', { hasText: 'Reports' })).toBeVisible();

    // Verify Not Clocked In state initially
    await expect(page.locator('h2', { hasText: 'Not Clocked In' })).toBeVisible();

    // Click Clock In
    await page.getByRole('button', { name: 'Clock In' }).click();

    // Verify state changes to Clocked In
    await expect(page.locator('h2', { hasText: 'Clocked In' })).toBeVisible();

    // Check that we can clock out
    await expect(page.getByRole('button', { name: 'Clock Out' })).toBeVisible();

    // Lock terminal - we must use the specific lock button from the header, not clock out
    await page.getByRole('button', { name: 'Lock', exact: true }).click();

    // Back to locked screen
    await expect(page.locator('h1', { hasText: 'Terminal Locked' })).toBeVisible();
  });
});
