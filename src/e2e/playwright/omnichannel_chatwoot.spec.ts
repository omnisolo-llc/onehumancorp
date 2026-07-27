import { test as base, expect, type BrowserContext, type Page } from '@playwright/test';

export const test = base.extend({
  page: async ({ page }, use) => {
    // we bypass the fixture loginAs for a direct test since we don't have the full app up via normal e2e ways
    await use(page);
  }
});

test.describe('Omnichannel Chatwoot Parity CUJ', () => {
  test('CUJ: Verify Inbox config, conversation threads, and AI Drafts', async ({ page }) => {
    await page.goto('/omni-chat');

    // 1. Open config and set working hours and out-of-office message
    await page.click('button:has-text("Config")');
    await expect(page.locator('h2')).toHaveText('Inbox Settings');

    const toggle = page.locator('input[type="checkbox"]');
    await toggle.check();
    expect(await toggle.isChecked()).toBe(true);

    const textarea = page.locator('textarea');
    await textarea.fill('Testing auto reply offline mode');
    await page.click('button:has-text("Save Settings")');

    // 2. Open a thread
    await page.click('text=Maya');

    // 3. Verify Thread View Elements
    await expect(page.locator('text=Do you make vegan cakes?')).toBeVisible();
    await expect(page.locator('text=Auto-reply sent (Off-hours)')).toBeVisible();

    // 4. Verify AI Draft generated
    await expect(page.locator('text=✨ AI Draft')).toBeVisible();
    await expect(page.locator('button:has-text("Approve & Send")')).toBeVisible();

  });
});
