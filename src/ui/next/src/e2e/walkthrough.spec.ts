import { test, expect } from '@playwright/test';

test.describe('Interactive Walkthroughs', () => {
  test('renders help widget and completes the store setup walkthrough', async ({ page }) => {
    // Navigate to a page with the walkthrough target and the help widget
    await page.goto('/dashboard'); // or /storefront-builder which has bio-input

    // Open the help widget
    const helpButton = page.locator('button[aria-label="Help"]').first();
    await expect(helpButton).toBeVisible();
    await helpButton.click();

    // Click on the store setup walkthrough
    const tourButton = page.locator('button', { hasText: 'Tour: Virtual Meeting Room & UltraPlan' });
    await expect(tourButton).toBeVisible();
    await tourButton.click();

    // In a real flow, you might be redirected or the elements are present.
    // Since /dashboard does not have bio-input, let's navigate directly to storefront builder if it redirects,
    // or just mock the route if possible. We will assume the Tour redirects or we just navigate to where bio-input is.

    // Instead of dashboard, let's go straight to builder since that's where the target elements are:
    await page.goto('/builder?test_walkthrough=true');

    // Re-open help widget on the right page
    const builderHelpButton = page.locator('button[aria-label="Help"]').first();
    await expect(builderHelpButton).toBeVisible();
    await builderHelpButton.click();

    const builderTourButton = page.locator('button', { hasText: 'Tour: Virtual Meeting Room & UltraPlan' });
    await expect(builderTourButton).toBeVisible();
    await builderTourButton.click();

    // Assert the first step is shown
    const speechBubble = page.locator('div[role="dialog"]');
    await expect(speechBubble).toBeVisible();
    await expect(page.getByText('Agents join the Virtual Meeting Room to debate and plan before executing tasks.')).toBeVisible();

    // Click Next
    await page.locator('button', { hasText: /^Next$/ }).click();

    // Assert the second step is shown
    await expect(page.getByText('Phase 1: Brainstorming. Phase 2: Refinement. Phase 3: Consensus (UltraPlan protocol).')).toBeVisible();

    // Click Finish
    await page.locator('button', { hasText: /^Finish$/ }).click();

    // Assert the bubble is gone
    await expect(speechBubble).not.toBeVisible();
  });

  test('user can exit the walkthrough early by clicking the skip/close button', async ({ page }) => {
    await page.goto('/builder?test_walkthrough=true');

    // Re-open help widget
    const builderHelpButton = page.locator('button[aria-label="Help"]').first();
    await expect(builderHelpButton).toBeVisible();
    await builderHelpButton.click();

    const builderTourButton = page.locator('button', { hasText: 'Tour: Virtual Meeting Room & UltraPlan' });
    await expect(builderTourButton).toBeVisible();
    await builderTourButton.click();

    // Assert the first step is shown
    const speechBubble = page.locator('div[role="dialog"]');
    await expect(speechBubble).toBeVisible();

    // Highlight overlay should be visible
    const highlightOverlay = page.locator('.fixed.z-\\[90\\]');
    await expect(highlightOverlay).toBeVisible();

    // Click the skip/close button in the walkthrough header
    const closeButton = speechBubble.locator('button', { hasNotText: 'Next' }).first();
    await expect(closeButton).toBeVisible();
    await closeButton.click();

    // Assert both the bubble and the overlay are removed from the DOM
    await expect(speechBubble).not.toBeVisible();
    await expect(highlightOverlay).not.toBeVisible();
  });
});
