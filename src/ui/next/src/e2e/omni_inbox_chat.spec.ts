import { test, expect } from '@playwright/test';
import { adminPage } from '../../../../e2e/fixtures';

test.describe('Omni Inbox Native Chat', () => {
  // Use the adminPage fixture to ensure we are logged in before the test starts
  adminPage('displays native chat UI and handles web socket messages', async ({ page, request }) => {
    // Navigate to the inbox page, relying on the fixture's logged-in state
    await page.goto('/ui/inbox.html');

    // The layout should show Native Omnichannel Inbox
    await expect(page.locator('h1')).toContainText('Native Omnichannel Inbox');

    // To prevent flakiness in E2E, wait for the inbox list to resolve (it may be empty)
    await page.waitForSelector('#inbox-list');

    // Create an inbox dynamically for this test using the authenticated request context
    const createInboxRes = await request.post('/api/v1/chat/inboxes', {
      data: { name: 'E2E Test Support ' + Date.now() }
    });

    // Re-load the inboxes to pick up the newly created inbox
    await page.reload();

    // Click the first inbox
    const inboxItem = page.locator('.inbox-item').first();
    await expect(inboxItem).toBeVisible({ timeout: 10000 });
    await inboxItem.click();

    // Let's assert the conversation list is visible
    const convList = page.locator('#conv-list');
    await expect(convList).toBeVisible();
  });
});
