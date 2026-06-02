import { expect, test } from './fixtures';

test('Verify standalone to cloud team invite growth loop', async ({ page }) => {
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

  // Playwright test environment can mock clipboard if needed, but UI updates
  // correctly regardless so we just ensure text changes.
  await copyBtn.click();
  await expect(copyBtn).toHaveText('Copied!');
});
