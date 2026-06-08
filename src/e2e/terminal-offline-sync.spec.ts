import { test, expect } from '@playwright/test';
import { memberPage as page } from './fixtures';

test.describe('Terminal Offline Sync & Session Sync', () => {
  test('Starts a session, works offline, and reconciles back online', async ({ page, context }) => {
    // 1. Navigate to the POS Terminal page
    await page.goto('/pos/terminal');

    // Check initial UI load
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
    await expect(page.getByText('Discovered')).toBeVisible();

    // We expect at least one simulated reader to be available to connect
    const connectButton = page.getByRole('button', { name: 'Connect' }).first();
    await connectButton.click();

    await expect(page.getByText('Connected to reader')).toBeVisible();

    // The test must assert that the session creation was successful, and we
    // now show an error if it fails, so we can assert the error message is NOT visible
    await expect(page.getByText('Connected, but session start failed')).toBeHidden();

    // 3. Toggle Offline mode
    await context.setOffline(true);
    // Give UI a moment to respond to offline event
    await page.waitForTimeout(1000);
    await expect(page.getByText('Offline Mode')).toBeVisible();

    // 4. Process Payment offline
    await page.getByRole('button', { name: /Charge \$/ }).click();

    // Status should update to offline queue
    await expect(page.getByText('Payment saved offline. Will sync when network is restored.')).toBeVisible();

    // 5. Restore network
    await context.setOffline(false);
    await page.waitForTimeout(1000);

    // The automatic sync process should eventually trigger
    await expect(page.getByText('Syncing offline events...')).toBeVisible({ timeout: 15000 });

    // Wait for sync banner to disappear
    await expect(page.getByText('Syncing offline events...')).toBeHidden({ timeout: 15000 });
  });
});
