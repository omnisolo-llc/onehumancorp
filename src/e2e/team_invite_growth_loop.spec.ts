import { test, expect } from '@playwright/test';

// Use basic playwright test instead of the custom currentAppSmoke to avoid fixtures issues
test('Verify standalone to cloud team invite growth loop', async ({ page }) => {
  // Navigate to team page directly (avoid auth logic and network intercept restrictions)
  await page.goto('/team');

  // Find and click the invite button
  const inviteBtn = page.locator('button', { hasText: 'Invite to Cloud Team' });
  await expect(inviteBtn).toBeVisible();
  await inviteBtn.click();

  // Verify modal is shown
  const modalHeader = page.locator('h2', { hasText: 'Cloud Bridge Invite' });
  await expect(modalHeader).toBeVisible();

  // Click Copy Link button
  const copyBtn = page.locator('button', { hasText: 'Copy Link' });
  await expect(copyBtn).toBeVisible();

  // playwright's context has write access so clip board works in browser tests
  await copyBtn.click();

  // Verify button changes text
  await expect(copyBtn).toHaveText('Copied!');
});
