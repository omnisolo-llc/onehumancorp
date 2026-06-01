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
    await page.getByRole('button', { name: 'Manage AI Assistants' }).click();
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
    await expect(page.getByText('Marketing Pro')).toBeVisible();
  });

  test('settings remain accessible from dashboard quick actions', async ({ page }) => {
    await page.goto('/dashboard');
    await page.getByRole('button', { name: 'Settings', exact: true }).click();
    await expect(page.getByRole('heading', { name: 'Settings' })).toBeVisible();
    await expect(page.getByText('Enable Email Notifications')).toBeVisible();
  });
});
