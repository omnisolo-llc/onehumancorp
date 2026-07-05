import { test, expect } from '@playwright/test';

test.describe('Subscription Management Portal', () => {

  test('successfully pauses subscription via magic link', async ({ page }) => {
    // Navigate to the portal with magic link query params
    await page.goto('/subscriptions/manage?token=fake-token&action=pause');

    // Check if the page renders the right content based on action
    await expect(page.getByRole('heading', { name: 'Pause Subscription' })).toBeVisible();
    await expect(page.getByText(/Are you sure you want to pause your subscription\?/)).toBeVisible();

    // Click confirm
    await page.getByRole('button', { name: 'Confirm pause' }).click();

    // Should show success state
    await expect(page.getByRole('heading', { name: 'Success!' })).toBeVisible();
    await expect(page.getByText('Your subscription has been updated successfully.')).toBeVisible();
  });

  test('shows invalid link state when missing params', async ({ page }) => {
    // Navigate without params
    await page.goto('/subscriptions/manage');

    await expect(page.getByRole('heading', { name: 'Invalid Link' })).toBeVisible();
    await expect(page.getByText('This subscription management link is missing required information.')).toBeVisible();
  });
});
