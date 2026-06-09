import { test, expect } from './fixtures';

test.describe('POS Offline Terminal', () => {
  test.beforeEach(async ({ page, context }) => {
    await context.clearCookies();
    await page.goto('/pos/terminal');
    await page.evaluate(() => {
        localStorage.clear();
    });
    await page.reload();
  });

  test('Processes payment offline and queues it for sync', async ({ page, context }) => {
    // 1. Check initial UI load
    await expect(page.getByText('Terminal Locked')).toBeVisible();

    // Unlock POS with pin '1111'
    for (let i = 0; i < 4; i++) {
       await page.getByRole('button', { name: '1' }).click();
    }

    // Verify logged in view
    await expect(page.getByText('Clock In')).toBeVisible();
    await page.getByRole('button', { name: 'Clock In' }).click();

    // 2. Discover readers and Connect (simulates StartSession)
    await page.getByRole('button', { name: 'Discover Readers' }).click();

    // We expect at least one simulated reader to be available to connect
    const connectButton = page.getByRole('button', { name: 'Connect' }).first();
    await expect(connectButton).toBeVisible({ timeout: 5000 });
    await connectButton.click();

    await expect(page.getByText('Connected to reader')).toBeVisible({ timeout: 5000 });
    await expect(page.getByText('Connected, but session start failed')).toBeHidden();

    // 3. Toggle Offline mode
    await context.setOffline(true);
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));
    await expect(page.locator('text=Offline Mode').first()).toBeVisible();

    // 4. Process Payment offline
    await page.locator('button', { hasText: 'Charge $' }).click();

    // Wait for offline process simulation
    await expect(page.locator('text=Payment saved offline')).toBeVisible({ timeout: 5000 });

    // Verify localStorage has the queued transaction
    const tx = await page.evaluate(() => JSON.parse(localStorage.getItem('ohc_offline_pos_tx') || '[]'));
    expect(tx.length).toBe(1);
    expect(tx[0].amount_cents).toBeGreaterThan(0);

    // 5. Restore network
    await context.setOffline(false);
    await page.evaluate(() => window.dispatchEvent(new Event('online')));
    await expect(page.locator('text=Offline Mode').first()).toBeHidden();

    // Wait for sync to complete (interval is 5s)
    await expect(async () => {
        const remainingTx = await page.evaluate(() => JSON.parse(localStorage.getItem('ohc_offline_pos_tx') || '[]'));
        expect(remainingTx.length).toBe(0);
    }).toPass({ timeout: 15000 });
  });
});
