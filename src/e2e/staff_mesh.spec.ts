import { test, expect } from '@playwright/test';

test.describe('Universal Staff & Shift Management Mesh CUJ', () => {
  test('Owner adds a staff member and staff member clocks in on POS terminal', async ({ page }) => {
    // Navigate to Team view as the business owner
    await page.goto('/team');

    // Expect Human Staff section to be visible
    await expect(page.locator('text=Human Staff')).toBeVisible();

    // Owner clicks "+" button to add staff
    await page.locator('button', { hasText: 'M12 4v16m8-8H4' }).click();

    // Fill out staff details
    await page.fill('input[placeholder="e.g. Sarah"]', 'Sarah');
    await page.fill('input[placeholder="(555) 019-9234"]', '555-0199');
    await page.selectOption('select', 'Cashier');

    // Handle alert prompt
    page.once('dialog', async (dialog) => {
      expect(dialog.message()).toContain('Staff member added! PIN setup link:');
      await dialog.accept();
    });

    // Send Invite
    await page.locator('button:has-text("Send Invite Link")').click();

    // Verify Sarah appears in Human Staff section
    await expect(page.locator('text=Sarah').first()).toBeVisible();
    await expect(page.locator('text=Cashier').first()).toBeVisible();

    // Now navigate to POS Terminal view as the newly added staff member
    await page.goto('/terminal');

    // Expect PIN input page
    await expect(page.locator('text=Enter PIN to unlock')).toBeVisible();

    // Enter correct PIN (1234)
    await page.locator('button:has-text("1")').click();
    await page.locator('button:has-text("2")').click();
    await page.locator('button:has-text("3")').click();
    await page.locator('button:has-text("4")').click();

    // Verify terminal unlock and role recognition
    await expect(page.locator('text=Logged in as Sarah (Cashier)')).toBeVisible({ timeout: 5000 });

    // Simulate going offline
    await page.evaluate(() => {
      Object.defineProperty(navigator, 'onLine', { value: false });
      window.dispatchEvent(new Event('offline'));
    });

    // Click clock in
    await page.locator('button:has-text("CLOCK IN")').click();

    // Verify offline notice
    await expect(page.locator('text=Offline Mode Active')).toBeVisible();
    await expect(page.locator('text=1 pending events')).toBeVisible();

    // Verify clock in state change
    await expect(page.locator('button:has-text("CLOCK OUT")')).toBeVisible();

    // Simulate coming back online
    await page.evaluate(() => {
      Object.defineProperty(navigator, 'onLine', { value: true });
      window.dispatchEvent(new Event('online'));
    });

    // Expect offline events to clear
    await expect(page.locator('text=1 pending events')).not.toBeVisible();
  });
});
