import { test, expect, adminPage } from '../fixtures';

test.describe('Unified Feed E2E', () => {
  test('Complete flow: view feed and approve an item', async ({ browser }) => {
    const page = await adminPage(browser);

    // Start from the home page
    await page.goto('/dashboard');

    // Navigate to the unified feed
    await page.goto('/unified-feed');

    // Wait for the feed to load
    const title = page.locator('text=Today');
    await expect(title).toBeVisible();

    // The backend provides seed data (see simulate_inbound_signal_handler / api routes)
    // we assume data is available or wait for it.
    // If the feed is empty, we must fail the test because the requirement is to verify the approve flow.
    // Wait for the feed list to load, expecting at least one item.
    // We expect the button to exist.
    const approveBtn = page.locator('button:has-text("Approve")').first();
    await expect(approveBtn).toBeVisible({ timeout: 10000 });

    await approveBtn.click();

    // We can't use arbitrary timeouts. We wait for either the button to disappear or
    // the status of the item to update, or for the "All caught up!" message if it was the last item.
    await expect(approveBtn).not.toBeVisible({ timeout: 10000 });
  });
});
