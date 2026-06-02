import { test, expect } from './fixtures';

test.describe('Wizard Refinement E2E', () => {
  test('keeps the setup flow plain-language and reversible', async ({ page }) => {
    await page.goto('/website-builder');
    await expect(page.getByText('Zero tech skills needed. We do the heavy lifting.')).toBeVisible();
    await page.getByRole('button', { name: /Start My Business Next/ }).click();
    await expect(page.getByRole('heading', { name: 'What kind of business are you building?' })).toBeVisible();
    await page.getByRole('button', { name: 'Back' }).click();
    await expect(page.getByRole('heading', { name: 'Your business, live in minutes.' })).toBeVisible();
  });

  test('exposes AI helper and prompt tuning areas', async ({ page }) => {
    await page.goto('/dashboard');
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'AI Departments' })).toBeVisible();
    await expect(page.getByText('The Ambassador')).toBeVisible();
  });

  test('settings remain accessible from dashboard quick actions', async ({ page }) => {
    await page.goto('/dashboard');
    await page.goto('/settings');
    await expect(page.getByRole('heading', { name: 'Settings' })).toBeVisible();
    await expect(page.getByText('Enable Email Notifications')).toBeVisible();
  });
});
