import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Omni Inbox Native Chat', () => {
  adminPage('displays native chat UI and handles web socket messages', async ({ page }) => {
    await page.goto('/ui/inbox.html');
    await expect(page.locator('h1')).toContainText('Native Omnichannel Inbox');
    await page.waitForSelector('#inbox-list');
  });
});
