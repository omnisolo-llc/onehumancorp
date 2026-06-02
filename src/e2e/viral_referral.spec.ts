import { test, expect } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('viral_referral', async ({ page }) => {
  await page.goto('/dashboard');

  // Click 'Invite a Business & Earn $50' button
  const inviteBtn = page.getByRole('button', { name: /Invite a Business & Earn/i });
  await expect(inviteBtn).toBeVisible();
  await inviteBtn.click();

  // Expect the API to be called and modal to show up
  const modalHeader = page.getByRole('heading', { name: 'Help a Business Grow!' });
  await expect(modalHeader).toBeVisible();

  // Verify referal link contains dynamic=true
  const inputLink = page.locator('input.flex-1.bg-gray-50');
  await expect(inputLink).toHaveValue(/dynamic=true/);
});
