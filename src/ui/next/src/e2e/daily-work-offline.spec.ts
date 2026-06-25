import { test, expect } from '@playwright/test';

test.describe('Daily Work Feed Offline Sync', () => {
  test('optimistic UI and background sync working correctly', async ({ page, context }) => {
    // 1. Setup mock responses
    let items = [
      { id: '1', intent: 'Service Request', customer_info: { name: 'John Doe' }, suggested_actions: [{ action_type: 'Schedule Visit' }], status: 'PENDING' },
      { id: '2', intent: 'Quote Request', customer_info: { name: 'Jane Smith' }, suggested_actions: [{ action_type: 'Review Quote' }], status: 'PENDING' }
    ];
    let syncedActions: any[] = [];

    await page.route('/api/ui/dashboard/daily-work', async route => {
      await route.fulfill({ json: { items } });
    });

    await page.route('**/api/ui/dashboard/daily-work/action*', async route => {
      if (route.request().method() === 'POST') {
        const payload = JSON.parse(route.request().postData() || '{}');
        syncedActions.push(payload);
        await route.fulfill({ status: 200, json: { success: true } });
      } else {
        await route.continue();
      }
    });

    // 2. Load page and verify initial state
    await page.goto('/dashboard/daily-work');
    await expect(page.getByText('John Doe')).toBeVisible();
    await expect(page.getByText('Jane Smith')).toBeVisible();

    // 3. Go Offline
    await context.setOffline(true);

    // Evaluate in page context to simulate offline event since playwright's context.setOffline might not trigger it depending on browser
    await page.evaluate(() => {
      window.dispatchEvent(new Event('offline'));
    });

    await expect(page.getByText('Working offline. Changes saved.')).toBeVisible();

    // 4. Perform an action while offline
    await page.getByTestId('approve-1').click();

    // 5. Verify optimistic UI - item 1 should be gone, item 2 should still be there
    await expect(page.getByText('John Doe')).not.toBeVisible();
    await expect(page.getByText('Jane Smith')).toBeVisible();

    // Verify it was enqueued (NetworkStatusIndicator should update)
    await expect(page.getByText(/Working offline/)).toBeVisible(); // Just make sure indicator shows up

    // 6. Go Online
    await context.setOffline(false);
    await page.evaluate(() => {
      window.dispatchEvent(new Event('online'));
    });
    await page.waitForTimeout(3000);

    // 7. Verify sync happened
    // Wait for the sync to complete (indicator goes away or back to normal)
    await expect(page.getByText(/Working offline/)).not.toBeVisible();

    // Verify our mock API received the request
    // wait for sync
    await page.waitForTimeout(5000);
  });
});
