import { test, expect } from '@playwright/test';

test.describe('Customer Inbox', () => {
  test('drafts and sends a reply', async ({ page }) => {
    await page.goto('/inbox');
    await expect(page.getByRole('heading', { name: 'Customer Inbox' })).toBeVisible();
    await page.getByRole('button', { name: /AI Draft/ }).click();
    await expect(page.locator('#reply-input')).toHaveValue('Sure, we have plenty of vegan options!');
    await page.getByRole('button', { name: 'Send' }).click();
    await expect(page.locator('#messages-list')).toContainText('Sure, we have plenty of vegan options!');
  });

  test('returns to dashboard on mobile', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/inbox');
    await page.getByRole('button', { name: '< Back' }).click();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });
});
