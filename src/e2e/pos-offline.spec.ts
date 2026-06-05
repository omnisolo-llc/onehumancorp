import { test, expect } from '@playwright/test';

test.describe('Offline-Capable POS Engine', () => {
  test('Persona: Business Owner can sync offline transactions when back online', async ({ page, request }) => {
    // Seed staff via standard post to the Next.js local memory backend
    await request.post('/api/staff', {
        data: { name: 'Test Staff', role: 'Cashier', pin_hash: '1234' }
    });

    await page.goto('/pos/terminal');

    // Make sure we are at the locked screen
    await expect(page.locator('h1').filter({ hasText: 'Terminal Locked' })).toBeVisible();

    // Wait for the app to finish the background fetch and populate cache
    await page.waitForTimeout(1000);

    // Click PIN: 1, 2, 3, 4
    await page.getByRole('button', { name: '1' }).click();
    await page.getByRole('button', { name: '2' }).click();
    await page.getByRole('button', { name: '3' }).click();
    await page.getByRole('button', { name: '4' }).click();

    // It should unlock and show the name
    await expect(page.locator('h1').filter({ hasText: 'Test Staff' })).toBeVisible();
    await expect(page.locator('text=Not Clocked In')).toBeVisible();

    // Clock In
    await page.getByRole('button', { name: 'Clock In' }).click();
    await expect(page.locator('text=Clocked In')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Clock Out' })).toBeVisible();

    // New Order alert mock
    page.once('dialog', dialog => dialog.accept());
    await page.locator('text=New Order').click();

    // Verify localStorage has events
    const events = await page.evaluate(() => JSON.parse(localStorage.getItem('ohc_offline_events') || '[]'));
    expect(events.length).toBeGreaterThan(0);
    expect(events[0].event_type).toBe('CLOCK_IN');

    // Note: the component syncs every 10 seconds. We can wait for it.
    await expect(async () => {
       const eventsAfterSync = await page.evaluate(() => JSON.parse(localStorage.getItem('ohc_offline_events') || '[]'));
       expect(eventsAfterSync.length).toBe(0);
    }).toPass({ timeout: 15000 });
  });
});
