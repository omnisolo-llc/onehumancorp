import { test, expect } from '../../../../e2e/fixtures';

test.describe('Menu Management Offline Sync', () => {
  test.describe.configure({ mode: 'serial' });

  test.beforeEach(async ({ page, context }) => {
    // Clear cookies and state
    await context.clearCookies();
    await page.goto('/menu-management');
    await page.evaluate(() => localStorage.clear());
    await page.reload();

    // Wait for page to load
    await expect(page.locator('text=Today\'s Active Menu').first()).toBeVisible({ timeout: 10000 });
  });

  test('performs optimistic UI updates and syncs queue when offline', async ({ page, context }) => {
    // Check initial state
    await expect(page.locator('text=Today\'s Active Menu').first()).toBeVisible();

    const toggleButton = page.locator('button[id^="sold-out-toggle-"]').first();

    const hasMenuItems = await toggleButton.count() > 0;
    if (!hasMenuItems) {
        return;
    }

    const initialText = await toggleButton.textContent() || '';

    // Set network to offline
    await context.setOffline(true);
    await page.evaluate(() => window.dispatchEvent(new Event('offline')));

    // Expect the offline banner
    await expect(page.locator('text=Offline - Changes saved locally')).toBeVisible();

    // Perform optimistic action: Toggle sold out
    await toggleButton.click();

    // Expect the text to change optimistically
    await expect(toggleButton).not.toHaveText(initialText);

    // Expect the Pending Sync indicator to show up
    await expect(page.locator('text=Pending Sync')).toBeVisible();

    // Restore network
    await context.setOffline(false);
    await page.evaluate(() => window.dispatchEvent(new Event('online')));

    // Expect offline badge to disappear (queue should process) and success toast
    await expect(page.locator('text=Menu updated online')).toBeVisible({ timeout: 15000 });
    await expect(page.locator('text=Pending Sync')).toBeHidden();
  });
});
