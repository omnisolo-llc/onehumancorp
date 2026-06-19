import { test, expect } from '@playwright/test';

test.describe('Offline Action Queue E2E', () => {
  test('1. should show offline indicator when network goes down', async ({ page }) => {
    await page.goto('/field-ops/jobs');
    await expect(page.locator('text="Today\'s Route"').first()).toBeVisible({ timeout: 15000 });

    // Go offline
    await page.context().setOffline(true);
    await expect(page.locator('text="Offline Mode"').first()).toBeVisible({ timeout: 5000 });
  });

  test('2. should optimistic update UI and queue action offline', async ({ page }) => {
    await page.goto('/field-ops/jobs');
    await expect(page.locator('text="Today\'s Route"').first()).toBeVisible({ timeout: 15000 });

    await page.context().setOffline(true);

    // Interact
    await page.locator('text="Heading to Job"').first().click();
    await page.locator('text="Start Work"').first().click();
    await page.locator('text="Job Done"').first().click();

    // Optimistic UI updates to completed
    await expect(page.locator('text="COMPLETED"').first()).toBeVisible();

    // Should show "Working Offline" or Syncing queue indicator from NetworkStatusIndicator
    await expect(page.locator('text="Working Offline"').first()).toBeVisible();
  });

  test('3. should drain action queue when coming back online', async ({ page }) => {
    await page.goto('/field-ops/jobs');
    await expect(page.locator('text="Today\'s Route"').first()).toBeVisible({ timeout: 15000 });

    await page.context().setOffline(true);

    // Mutate state
    await page.locator('text="Heading to Job"').first().click();

    // Check queue indicator
    await expect(page.locator('text="Working Offline"').first()).toBeVisible();

    // Go online
    await page.context().setOffline(false);

    // Ensure the queue indicator changes to syncing or disappears
    await expect(page.locator('text="Working Offline"')).not.toBeVisible();
  });

  test('4. should process mutations correctly on backend', async ({ page }) => {
    await page.goto('/field-ops/jobs');
    await expect(page.locator('text="Today\'s Route"').first()).toBeVisible({ timeout: 15000 });

    await page.context().setOffline(true);

    // Complete the job with notes
    await page.locator('text="Heading to Job"').first().click();
    await page.locator('text="Start Work"').first().click();
    await page.fill('textarea[placeholder="E.g., Needs a replacement quote."]', 'Action Queue E2E notes');
    await page.locator('text="Job Done"').first().click();

    await page.context().setOffline(false);

    // Wait for the sync
    await expect.poll(async () => {
        try {
           const res = await page.evaluate(async () => {
              const fetchRes = await fetch('/api/v1/sync/power_sync_pull');
              if (!fetchRes.ok) return false;
              return true;
           });
           return res;
        } catch (e) {
            return false;
        }
    }, { timeout: 15000 }).toBe(true);

    await page.goto('/dashboard');
    await expect(page.locator('text="Action Queue E2E notes"').first()).toBeVisible({ timeout: 15000 });
  });

  test('5. should retain pending actions across reloads if offline', async ({ page }) => {
    await page.goto('/field-ops/jobs');
    await expect(page.locator('text="Today\'s Route"').first()).toBeVisible({ timeout: 15000 });

    await page.context().setOffline(true);

    // Mutate state
    await page.locator('text="Heading to Job"').first().click();

    // Reload page while offline
    await page.reload();
    await expect(page.locator('text="Working Offline"').first()).toBeVisible();

    // Action should still be in queue (we can verify by checking if it attempts to sync when online)
    await page.context().setOffline(false);
    await expect(page.locator('text="Working Offline"')).not.toBeVisible();
  });
});
