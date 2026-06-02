import { test, expect } from '@playwright/test';

test.describe('Universal Autonomous Staff & Shift Management Mesh', () => {
  test('Manager adds staff member and staff member clocks in offline', async ({ page, context }) => {
    // Navigate to team page
    await page.goto('/team');

    // Wait for the layout to be ready
    await page.waitForSelector('text=Your Team');

    // Open invite modal using the FAB
    await page.click('button[aria-label="Add Staff"]');

    // Check if modal opens
    await expect(page.locator('text=Who are you hiring?')).toBeVisible();

    // Enter Phone Number
    await page.fill('input[type="tel"]', '(555) 123-4567');

    // Select Role
    await page.selectOption('select', 'cashier');

    // Click Send SMS Invite to create the user on the backend
    await page.click('button:has-text("Send SMS Invite")');

    // Wait for the UI to show Invite Sent
    await expect(page.locator('button:has-text("Invite Sent!")')).toBeVisible();

    // Open a new tab for terminal
    const terminalPage = await context.newPage();
    await terminalPage.goto('/terminal');

    // Wait for terminal layout
    await terminalPage.waitForSelector('text=Enter your PIN to unlock');

    // Enter correct PIN '1234' (default set by the mock API)
    await terminalPage.click('button:has-text("1")');
    await terminalPage.click('button:has-text("2")');
    await terminalPage.click('button:has-text("3")');
    await terminalPage.click('button:has-text("4")');

    // Click unlock
    await terminalPage.click('button:has-text("Unlock")');

    // Check POS screen
    await terminalPage.waitForSelector('text=Point of Sale');
    await expect(terminalPage.locator('text=cashier')).toBeVisible();

    // Cashier role shouldn't see Reports or Settings
    await expect(terminalPage.locator('text=Reports')).toBeHidden();
    await expect(terminalPage.locator('text=Settings')).toBeHidden();

    // Can see New Sale and Orders
    await expect(terminalPage.locator('text=New Sale')).toBeVisible();
    await expect(terminalPage.locator('text=Orders')).toBeVisible();

    // Clock in
    await terminalPage.click('button:has-text("Clock In")');

    // Verify clock out button appears
    await terminalPage.waitForSelector('button:has-text("Clock Out & Lock")');

    // Verify local storage was updated (offline queue)
    const events = await terminalPage.evaluate(() => {
        return JSON.parse(localStorage.getItem('ohc_timecard_events') || '[]');
    });

    expect(events.length).toBeGreaterThan(0);
    expect(events[0].event_type).toBe('CLOCK_IN');

    // Clock out
    await terminalPage.click('button:has-text("Clock Out & Lock")');

    // Ensure terminal is locked again
    await terminalPage.waitForSelector('text=Enter your PIN to unlock');
  });
});
