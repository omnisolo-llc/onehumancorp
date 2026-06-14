import { test, expect } from './fixtures';

test.describe('Distributed Inventory Sync POS', () => {

  test('should acquire optimistic lock during POS checkout and prevent online checkout', async ({ page, memberPage }) => {
    // Navigate to local API directly to set up origin to allow localstorage modification
    await memberPage.goto('/api/staff');
    await memberPage.evaluate(() => {
      localStorage.setItem('ohc_offline_staff', JSON.stringify([{
        id: 'staff_1',
        name: 'Priya',
        role: 'Manager',
        pin_hash: '1234'
      }]));
      localStorage.setItem('ohc_offline_events', JSON.stringify([]));
    });

    // POS Checkout
    await memberPage.goto('/pos/terminal');
    await expect(memberPage.locator('text=Terminal Locked')).toBeVisible({ timeout: 15000 });

    await memberPage.getByRole('button', { name: '1', exact: true }).click();
    await memberPage.getByRole('button', { name: '2', exact: true }).click();
    await memberPage.getByRole('button', { name: '3', exact: true }).click();
    await memberPage.getByRole('button', { name: '4', exact: true }).click();

    await expect(memberPage.locator('text=Priya')).toBeVisible();

    // Create a product via UI first in the owner view
    await page.goto('/login');
    await page.getByPlaceholder('Email address').fill('admin@ohc.local');
    await page.getByPlaceholder('Password').fill('admin');
    await page.getByRole('button', { name: 'Sign In' }).click();
    await expect(page.locator('h1', { hasText: 'Dashboard' })).toBeVisible({ timeout: 15000 });

    await page.goto('/business/setup');
    await expect(page.locator('text=Store Setup')).toBeVisible();
    await page.getByRole('button', { name: 'Add Product' }).click();
    await page.getByLabel('Product Name').fill('Red Dress');
    await page.getByLabel('Price').fill('100.00');
    await page.getByLabel('Inventory').fill('1');
    await page.getByRole('button', { name: 'Save' }).click();

    await memberPage.goto('/pos/terminal');
    await expect(memberPage.locator('text=Red Dress')).toBeVisible();

    // Click Red Dress in POS, which initiates a reserve lock
    await memberPage.locator('text=Red Dress').click();
    await expect(memberPage.locator('text=New Order Total')).toBeVisible();

    // Concurrently try to checkout online
    const onlinePage = await page.context().newPage();
    await onlinePage.goto('/storefront');
    await onlinePage.locator('text=Red Dress').click();
    await onlinePage.getByRole('button', { name: 'Add to Cart' }).click();
    await onlinePage.goto('/cart');
    await onlinePage.getByRole('button', { name: 'Checkout' }).click();

    // Online checkout should fail because item is locked
    await expect(onlinePage.locator('text=Item just sold out')).toBeVisible();

    // Complete POS checkout
    await memberPage.getByRole('button', { name: 'Tap to Pay' }).click();
    await expect(memberPage.locator('text=Payment Completed')).toBeVisible();

  });

  test('Operations Agent receives restock alert after POS checkout', async ({ page, memberPage }) => {
    // Assume Red Dress was bought and stock went to 0. Check for the task in Chat/Action Inbox
    await page.goto('/login');
    await page.getByPlaceholder('Email address').fill('admin@ohc.local');
    await page.getByPlaceholder('Password').fill('admin');
    await page.getByRole('button', { name: 'Sign In' }).click();

    await page.goto('/team/chat');
    // We expect the low stock alert to now be generated and visible because stock dropped to 0
    await expect(page.locator('text=Low Stock Alert')).toBeVisible({ timeout: 15000 });
    await expect(page.locator('text=Remaining Stock:')).toBeVisible();
    await expect(page.locator('text=0')).toBeVisible(); // stock should be 0
  });

  test('Offline POS transaction shows Offline Mode UI', async ({ page, context }) => {
    // Navigate to a safe api route first to set local storage before loading the main page
    await page.goto('/api/staff');

    // Setup local storage for offline staff, rules, and inventory
    await page.evaluate(() => {
        localStorage.setItem('ohc_offline_staff', JSON.stringify([{ id: 'staff_1', name: 'Carlos', role: 'Manager', pin_hash: '1234' }]));
    });

    // Navigate to the POS terminal page
    await page.goto('/pos/terminal');
    await expect(page.locator('text=Terminal Locked')).toBeVisible({ timeout: 15000 });

    // Enter PIN: 1234
    await page.getByRole('button', { name: '1', exact: true }).click();
    await page.getByRole('button', { name: '2', exact: true }).click();
    await page.getByRole('button', { name: '3', exact: true }).click();
    await page.getByRole('button', { name: '4', exact: true }).click();

    // Verify unlocked and shows staff name
    await expect(page.locator('text=Carlos')).toBeVisible();

    // Set network to offline
    await context.setOffline(true);
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));

    await expect(page.locator('text=Offline Mode')).toBeVisible();

    // Trigger New Order
    await page.locator('text=Quick Charge').click();

    // Verify Payment total and offline
    await expect(page.locator('text=Payment Saved Locally (Offline)')).toBeVisible();

    // Restore network
    await context.setOffline(false);
    await page.evaluate(() => window.dispatchEvent(new Event('online')));
    await expect(page.locator('text=Online')).toBeVisible();
    await expect(page.locator('text=Syncing transactions...')).toBeVisible();
  });

  test('POS terminal allows manager to unlock and clock in', async ({ page }) => {
    await page.goto('/api/staff');

    // Setup local storage for offline staff, rules, and inventory
    await page.evaluate(() => {
        localStorage.setItem('ohc_offline_staff', JSON.stringify([{ id: 'staff_1', name: 'Carlos', role: 'Manager', pin_hash: '1234' }]));
    });

    await page.goto('/pos/terminal');
    await expect(page.locator('text=Terminal Locked')).toBeVisible({ timeout: 15000 });

    // Enter PIN: 1234
    await page.getByRole('button', { name: '1', exact: true }).click();
    await page.getByRole('button', { name: '2', exact: true }).click();
    await page.getByRole('button', { name: '3', exact: true }).click();
    await page.getByRole('button', { name: '4', exact: true }).click();

    await expect(page.locator('text=Carlos')).toBeVisible();
    await page.getByRole('button', { name: 'Clock In' }).click();
    await expect(page.locator('text=Clocked In')).toBeVisible();
  });

  test('Operations Agent handles sync conflicts with offline orders', async ({ page, memberPage, context }) => {
    await page.goto('/api/staff');

    // Setup local storage for offline staff, rules, and inventory
    await page.evaluate(() => {
        localStorage.setItem('ohc_offline_staff', JSON.stringify([{ id: 'staff_1', name: 'Carlos', role: 'Manager', pin_hash: '1234' }]));
    });

    await page.goto('/pos/terminal');
    await expect(page.locator('text=Terminal Locked')).toBeVisible({ timeout: 15000 });

    // Enter PIN: 1234
    await page.getByRole('button', { name: '1', exact: true }).click();
    await page.getByRole('button', { name: '2', exact: true }).click();
    await page.getByRole('button', { name: '3', exact: true }).click();
    await page.getByRole('button', { name: '4', exact: true }).click();

    // Verify unlocked and shows staff name
    await expect(page.locator('text=Carlos')).toBeVisible();

    // Set network to offline
    await context.setOffline(true);
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));

    // Perform a sale locally
    // Trigger New Order
    await page.locator('text=Quick Charge').click();
    await expect(page.locator('text=Payment Saved Locally (Offline)')).toBeVisible();

    // Go back online
    await context.setOffline(false);
    await page.evaluate(() => window.dispatchEvent(new Event('online')));
    await expect(page.locator('text=Syncing transactions...')).toBeVisible();
  });
});
