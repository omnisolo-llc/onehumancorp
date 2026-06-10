import { test, expect } from '@playwright/test';

test.describe('Help Center Documentation', () => {
  test('should display help articles and search functionality', async ({ page }) => {
    // Navigate to the Help Center page
    await page.goto('/help');

    // Wait for the main heading to ensure the page has loaded
    await expect(page.getByRole('heading', { name: 'Help Center' })).toBeVisible();

    // Search for a specific term
    const searchInput = page.getByPlaceholder('Search for help articles and videos...');
    await searchInput.fill('store');

    // Wait for search results
    await page.waitForTimeout(500); // Debounce delay if any

    // Verify search results filter correctly
    await expect(page.getByRole('heading', { name: 'My Store', exact: true })).toBeVisible();

    // Clear search
    await searchInput.fill('');
    await page.waitForTimeout(500);

    // Verify initial categories are restored
    await expect(page.getByRole('heading', { name: 'Getting Started' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Payments' })).toBeVisible();
  });

  test('should open video tutorials', async ({ page }) => {
    await page.goto('/help');

    // Check if Video Tutorials section exists
    await expect(page.getByRole('heading', { name: 'Video Tutorials' })).toBeVisible();

    // Click on the first video
    const videoCard = page.locator('.aspect-\\[9\\/16\\]').first();
    await videoCard.click();

    // Wait for the modal and verify "Close video" button exists
    const closeButton = page.getByRole('button', { name: 'Close video' });
    await expect(closeButton).toBeVisible();

    // Close the video
    await closeButton.click();
    await expect(closeButton).not.toBeVisible();
  });

  test('should show advanced users section', async ({ page }) => {
    await page.goto('/help');

    await expect(page.getByRole('heading', { name: 'Advanced Users' })).toBeVisible();

    const apiLink = page.getByRole('link', { name: 'API Documentation' });
    await expect(apiLink).toBeVisible();
    await expect(apiLink).toHaveAttribute('href', '/api-docs');
  });

  test('should render API Docs tooltip', async ({ page }) => {
    await page.goto('/api-docs');

    // Wait for tooltip registry text to load
    await page.waitForTimeout(1000);

    // Find the tooltip target text and hover over it
    const tooltipTarget = page.locator('span:has-text("Advanced:")');
    await expect(tooltipTarget).toBeVisible();

    await tooltipTarget.hover();

    // Verify the tooltip content is displayed
    const tooltipContent = page.getByText('Direct API access is only for custom integrations.');
    await expect(tooltipContent).toBeVisible();
  });

  test('should show Ask AI Support Agent when search has no results', async ({ page }) => {
    await page.goto('/help');

    const searchInput = page.getByPlaceholder('Search for help articles and videos...');
    await searchInput.fill('thiswillnotmatchanything12345');

    // Wait for the debounce/search to complete
    await page.waitForTimeout(500);

    const askAIBtn = page.getByRole('button', { name: 'Ask AI Support Agent' });
    await expect(askAIBtn).toBeVisible();

    // Wait for tooltip registry text to load
    await page.waitForTimeout(1000);

    // Hover over the Ask AI button wrapper with tooltip
    await askAIBtn.hover();

    // Verify tooltip
    const tooltipContent = page.getByText('Open AI Help Chat to get answers instantly.');
    await expect(tooltipContent).toBeVisible();
  });
});
