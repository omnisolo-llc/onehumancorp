import { test, expect } from '@playwright/test';
import { adminUserTest } from './fixtures';

adminUserTest('Unified Inbox CUJ', async ({ page }) => {
  await page.goto('/inbox');
  await expect(page).toHaveURL(/.*inbox.*/);
});
