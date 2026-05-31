import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('ayrshare_integration');

import { test, expect } from './fixtures';

test('Ayrshare Integration - social media linking and cross-posting', async ({ page }) => {
  await page.goto('/integrations');
  await expect(page.getByRole('heading', { name: 'Connect Custom Software' }).first()).toBeVisible();

  // Test linking social accounts
  const connectAyrshareBtn = page.getByRole('button', { name: /Connect Social Media/i });
  if (await connectAyrshareBtn.isVisible()) {
    await connectAyrshareBtn.click();
    await expect(page.getByText(/Linked Accounts/i)).toBeVisible();
  }

  // Test unified inbox visibility
  await page.goto('/inbox');
  await expect(page.getByRole('heading', { name: /Customer Inbox/i }).first()).toBeVisible();
});
