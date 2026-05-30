import { test, expect } from './fixtures';

test.describe('Wizard Refinement E2E', () => {
  test('keeps the setup flow plain-language and reversible', async ({ page }) => {
    await page.goto('/website-builder');
    await expect(page.getByText('Welcome to OHC Smart Builder')).toBeVisible();
    await expect(page.locator('#bio-input')).toBeVisible();

    // Simulate reversible interaction
    const btn = page.getByRole('button', { name: /Build My Storefront/ });
    await expect(btn).toBeVisible();
    // Verify it requires typing
    await expect(btn).toBeDisabled();

    await page.locator('#bio-input').fill('I sell custom handmade vegan shoes online');
    await expect(btn).toBeEnabled();
    // Simulate generation initiation
    await btn.click();
    await expect(page.getByText('Agents are building your store...')).toBeVisible();
  });

  test('exposes AI helper and prompt tuning areas', async ({ page }) => {
    await page.goto('/dashboard');
    // Using locator that targets the link specifically
    await page.getByRole('link', { name: /AI Departments/ }).click();
    await expect(page.getByRole('heading', { name: 'AI Departments' })).toBeVisible();
  });

  test('settings remain accessible from dashboard quick actions', async ({ page }) => {
    await page.goto('/dashboard');

    // We expect the Advanced Settings text near the toggle button to be visible
    await expect(page.getByText('Advanced Settings')).toBeVisible();

    // Click the toggle button to make sure it exists
    await page.locator('button.w-10.h-6.rounded-full').first().click();
  });
});
