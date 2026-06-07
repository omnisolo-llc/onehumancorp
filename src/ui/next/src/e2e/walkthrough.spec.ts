import { test, expect } from '@playwright/test';

test.describe('Interactive Walkthroughs', () => {
  test('renders onboarding walkthrough', async ({ page }) => {
    await page.goto('/onboarding');
    const tourButton = page.locator('button', { hasText: 'Tour' });
    await expect(tourButton).toBeVisible();
    await tourButton.click();

    const speechBubble = page.locator('div[role="dialog"]');
    await expect(speechBubble).toBeVisible();
    await expect(page.getByText('Enter your business name here.')).toBeVisible();
  });

  test('renders POS terminal walkthrough', async ({ page }) => {
    await page.goto('/pos/terminal');
    const tourButton = page.locator('button', { hasText: 'Tour' });
    await expect(tourButton).toBeVisible();
    await tourButton.click();

    const speechBubble = page.locator('div[role="dialog"]');
    await expect(speechBubble).toBeVisible();
    await expect(page.getByText('Click here to start a new order and accept a payment.')).toBeVisible();
  });

  test('renders agents walkthrough', async ({ page }) => {
    await page.goto('/agents');
    const tourButton = page.locator('button', { hasText: 'Tour' });
    await expect(tourButton).toBeVisible();
    await tourButton.click();

    const speechBubble = page.locator('div[role="dialog"]');
    await expect(speechBubble).toBeVisible();
    await expect(page.getByText('Activate your AI Support Agent')).toBeVisible();
  });

  test('renders help widget and completes the store setup walkthrough', async ({ page }) => {
    // Navigate to a page with the walkthrough target and the help widget
    await page.goto('/dashboard'); // or /storefront-builder which has bio-input

    // Open the help widget
    const helpButton = page.locator('#help-widget-container button').first();
    await expect(helpButton).toBeVisible();
    await helpButton.click();

    // Click on the store setup walkthrough
    const tourButton = page.locator('button', { hasText: 'Tour: Set up your store' });
    await expect(tourButton).toBeVisible();
    await tourButton.click();

    // In a real flow, you might be redirected or the elements are present.
    // Since /dashboard does not have bio-input, let's navigate directly to storefront builder if it redirects,
    // or just mock the route if possible. We will assume the Tour redirects or we just navigate to where bio-input is.

    // Instead of dashboard, let's go straight to builder since that's where the target elements are:
    await page.goto('/builder');

    // Re-open help widget on the right page
    const builderHelpButton = page.locator('#help-widget-container button').first();
    await expect(builderHelpButton).toBeVisible();
    await builderHelpButton.click();

    const builderTourButton = page.locator('button', { hasText: 'Tour: Set up your store' });
    await expect(builderTourButton).toBeVisible();
    await builderTourButton.click();

    // Assert the first step is shown
    const speechBubble = page.locator('div[role="dialog"]');
    await expect(speechBubble).toBeVisible();
    await expect(page.getByText('Enter your business description.')).toBeVisible();

    // Click Next
    await page.getByRole('button', { name: 'Next' }).click();

    // Assert the second step is shown
    await expect(page.getByText('Click to generate!')).toBeVisible();

    // Click Finish
    await page.getByRole('button', { name: 'Finish' }).click();

    // Assert the bubble is gone
    await expect(speechBubble).not.toBeVisible();
  });
});
