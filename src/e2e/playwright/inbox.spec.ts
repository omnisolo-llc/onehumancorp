import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('inbox', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'inbox');
});

test('native omnichannel chat ws integration', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);

  // Go to inbox page
  await page.goto('/inbox');

  // Wait for it to settle and connect
  await expect(page.locator('[data-testid="inbox-settled"]')).toBeVisible({ timeout: 15000 });

  // Look for the inbox empty state or the message list
  await expect(page.locator('text=Native Rust omnichannel chat engine unified conversations.')).toBeVisible();
});
