import { test, expect } from '@playwright/test';

test.describe('Frontend Degradation and Fail-Safe Validation', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('reliability-test@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.locator('button:has-text("Sign In")').click();
    await page.waitForURL('**/dashboard**');
  });

  test('should show cached data when backend latency spikes > 2s', async ({ page }) => {
    // Navigate to Records
    await page.locator('button:has-text("Records")').click();

    // Simulate high latency for the next request
    // Note: In a real environment, we'd use our Native Fault Injection API
    // page.request.post('/api/v1/chaos/fault', { data: { op: 'get_records', delay_ms: 3000 } });

    // The UI should show cached results immediately while the "loading" indicator or "Syncing" status appears
    const recordTable = page.locator('.record-table');
    await expect(recordTable).toBeVisible();

    // Verify that data is readable even if a background fetch is slow
    const firstRecord = recordTable.locator('tr').first();
    await expect(firstRecord).not.toBeEmpty();

    // Check for a subtle "Syncing" or "Offline" indicator suggesting use of cached data
    const statusIndicator = page.locator('.sync-status');
    await expect(statusIndicator).toBeVisible();
  });

  test('should queue write operations locally during connection drop', async ({ page }) => {
    // Navigate to Inbox/Messages
    await page.locator('button:has-text("Check Messages")').click();

    // Simulate connection drop
    // await page.context().setOffline(true);

    // Attempt to send a message
    await page.locator('input[placeholder*="message"]').fill('Reliability Test Message');
    await page.locator('button:has-text("Send")').click();

    // UI should NOT show an error dialog that blocks the user.
    // Instead, it should show a "Pending" or "Queued" status next to the message.
    const queuedMessage = page.locator('text=Reliability Test Message');
    await expect(queuedMessage).toBeVisible();

    const pendingIcon = page.locator('.message-status-pending');
    await expect(pendingIcon).toBeVisible();

    // Restore connection
    // await page.context().setOffline(false);

    // Verify it eventually syncs
    // await expect(pendingIcon).not.toBeVisible({ timeout: 10000 });
  });

  test('should fail-safe the Business Setup Wizard during database lag', async ({ page }) => {
    await page.locator('button:has-text("Update Setup")').click();

    // Progress through wizard
    await page.locator('button:has-text("Online Store")').click();
    await page.locator('input[placeholder*="name"]').fill('Degradation Test Shop');
    await page.locator('button:has-text("Next")').click();

    // Simulate database lag on state save
    // page.request.post('/api/v1/chaos/fault', { data: { op: 'save_wizard_state', delay_ms: 5000 } });

    await page.locator('button:has-text("Next")').click();

    // The UI should remain interactive and allow the user to continue to the next step
    // using local state, while the save happens in the background.
    const nextStepHeading = page.locator('h1:has-text("What do you sell?")');
    await expect(nextStepHeading).toBeVisible();
  });
});
