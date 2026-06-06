import { test, expect } from '@playwright/test';

test.describe('Documentation Components', () => {

  test('Help Center Search functionality', async ({ page }) => {
    await page.goto('/help');
    await expect(page.locator('h1', { hasText: 'Help Center' })).toBeVisible();

    const searchInput = page.getByPlaceholder('Search for help articles and videos...');
    await searchInput.fill('Getting Started');

    // Should show the article
    await expect(page.locator('h2', { hasText: 'Getting Started' }).first()).toBeVisible();

    // Fill with something not found
    await searchInput.fill('XYZNonExistent');
    await expect(page.locator('text=No results found matching "XYZNonExistent"')).toBeVisible();
  });

  test('Tooltip rendering on hover', async ({ page }) => {
    // Go to api docs where tooltip exists
    await page.goto('/api-docs');
    const tooltipTarget = page.locator('span', { hasText: 'Advanced:' });

    await expect(tooltipTarget).toBeVisible();
    await tooltipTarget.hover();

    // Wait for tooltip to appear
    await expect(page.locator('text=Direct API access is only for custom integrations.')).toBeVisible();
  });

  test('Video tutorials page rendering', async ({ page }) => {
    await page.goto('/help/videos');
    await expect(page.locator('h1', { hasText: 'Video Guides' })).toBeVisible();

    // Wait for page to load.
    // We expect some videos to render or at least the back button
    await expect(page.locator('text=Back to Help Center')).toBeVisible();
  });

  test('Interactive Walkthrough visibility', async ({ page }) => {
    // Enable walkthrough via query param test bypass
    await page.goto('/dashboard?test_walkthrough=true');
    // Wait for the WalkthroughTarget to be present
    // It's attached to 'help-widget-container' in layout
    // In order to test the walkthrough, we open the Help widget and click a walkthrough tour button.
    const helpBtn = page.getByRole('button', { name: 'Help', exact: true }).first();
    await expect(helpBtn).toBeVisible();
    await helpBtn.click();

    // Now it expects exactly one Help widget container, ignoring hidden ones
    // Check if help widget is open
    await expect(page.locator('#help-widget-container').last()).toBeVisible();

    // Click the "Tour: Set up your store" button
    const tourBtn = page.getByRole('button', { name: 'Tour: Virtual Meeting Room & UltraPlan' });
    await expect(tourBtn).toBeVisible();
    await tourBtn.click();

    // Check if the first step speech bubble dialog is visible
    const bubble = page.getByRole('dialog', { name: 'Quick Guide walkthrough step' });
    await expect(bubble).toBeVisible();

    // Finish the tour
    const finishBtn = page.getByRole('button', { name: 'Next' }).or(page.getByRole('button', { name: 'Finish' }));
    await finishBtn.click();
  });

  test('Help Chat widget visibility', async ({ page }) => {
    await page.goto('/dashboard?test_chat=true');

    // The floating button should be visible
    const chatBtn = page.getByRole('button', { name: 'Open help chat' });
    await expect(chatBtn).toBeVisible();

    // Open chat
    await chatBtn.click();

    // Check if chat window is visible
    await expect(page.locator('h3', { hasText: 'Ask AI Help' })).toBeVisible();

    // Check input
    const input = page.getByPlaceholder('Ask me anything...');
    await expect(input).toBeVisible();

    // Close chat
    const closeBtn = page.getByRole('button', { name: 'Close help chat' });
    await closeBtn.click();

    // Verify it's closed
    await expect(page.locator('h3', { hasText: 'Ask AI Help' })).not.toBeVisible();
  });

});
