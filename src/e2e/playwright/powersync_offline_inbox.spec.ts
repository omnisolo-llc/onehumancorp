import { test, expect } from '@playwright/test';

test.describe('PowerSync Offline Unified Inbox', () => {
  test('should render inbox messages via PowerSync provider and show offline status', async ({ page }) => {
    // Navigate to inbox
    await page.goto('/inbox');

    // Wait for AppShell title to appear
    await expect(page.locator('text="Unified Inbox"').first()).toBeVisible({ timeout: 10000 });

    // The component is using PowerSyncProvider so the subtitle reflects local-first setup
    await expect(page.locator('text="Local-first offline unified customer conversations and drafts."').first()).toBeVisible();

    // Verify Message Queue section is loaded securely via PowerSync
    await expect(page.locator('text="Loaded securely via PowerSync local embedded DB."').first()).toBeVisible();

    // Ensure that it either shows "No inbox messages found offline" or the messages list
    const emptyState = page.locator('text="No inbox messages found offline. Connect to sync."');
    const hasMessages = await emptyState.isVisible().catch(() => false);

    if (!hasMessages) {
       // Should have list items if messages exist
       await expect(page.locator('.app-list-item').first()).toBeVisible();
    }
  });
});
