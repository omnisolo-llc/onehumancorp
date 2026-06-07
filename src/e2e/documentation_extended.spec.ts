import { test, expect } from './fixtures';

test.describe('Extended Documentation & Help Features', () => {

  test('should display empty state when Help Center search yields no results', async ({ page }) => {
    await page.goto('/help');

    const searchInput = page.getByPlaceholder('Search for help articles and videos...');
    await searchInput.fill('XYZNonExistent123');

    // Wait for the empty state to appear
    await expect(page.getByText('No results found matching "XYZNonExistent123"')).toBeVisible({ timeout: 10000 });
  });

  test('should launch interactive walkthrough from Help widget', async ({ page }) => {
    await page.goto('/');

    // Open help widget
    const helpBtn = page.getByRole('button', { name: 'Help', exact: true });
    await expect(helpBtn).toBeVisible();
    await helpBtn.click();

    // Click the walkthrough button
    const walkthroughBtn = page.getByRole('button', { name: 'Tour: Activate your AI Support Agent' });
    await expect(walkthroughBtn).toBeVisible();
    await walkthroughBtn.click();

    // Verify walkthrough bubble appears
    await expect(page.getByRole('dialog', { name: 'Activate your AI Support Agent walkthrough step' })).toBeVisible({ timeout: 10000 });
  });

  test('should display duration badges on video tutorials', async ({ page }) => {
    await page.goto('/help/videos');

    // Wait for the video list to load
    await expect(page.getByRole('heading', { name: 'Video Guides', exact: true })).toBeVisible();

    // We expect the specific duration "1:20" from the mocked API / E2E backend for "How to set up your first store easily"
    await expect(page.getByText('1:20', { exact: true })).toBeVisible({ timeout: 10000 });
  });

  test('should display external link to full technical changelog in release notes', async ({ page }) => {
    await page.goto('/changelog');

    await expect(page.getByRole('heading', { name: 'Release Notes & Changelog' })).toBeVisible();

    const externalLink = page.getByRole('link', { name: 'Read the full technical changelog on our website →' });
    await expect(externalLink).toBeVisible();
    await expect(externalLink).toHaveAttribute('href', 'https://onehumancorp.com/changelog');
  });

  test('should load Swagger UI in API Documentation page', async ({ page }) => {
    await page.goto('/api-docs');

    // Check for advanced badge
    await expect(page.getByText('Advanced:')).toBeVisible();

    // Check that Swagger UI rendered something (e.g., the title or main container)
    // The Swagger UI container class is typically swagger-ui, and it renders the title
    await expect(page.locator('.swagger-ui')).toBeVisible({ timeout: 10000 });
    await expect(page.getByText('OHC Advanced API Reference')).toBeVisible();
  });

});
