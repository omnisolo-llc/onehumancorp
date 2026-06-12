import { test, expect } from './fixtures';

test.describe('Help Center Interactive Elements', () => {
  test('Persona: Business Owner uses interactive walkthroughs and tooltips', async ({ page }) => {
    // Navigate to dashboard first to establish session if needed
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');

    // Go to Help Center
    await page.goto('/help');
    await expect(page.locator('text=Help Center').first()).toBeVisible();

    // Verify search tooltip is present on hover
    const searchInput = page.locator('input[placeholder="Search for help articles and videos..."]');
    await searchInput.hover();
    // Wait for the tooltip registry text to appear (usually "Search our knowledge base for help articles.")
    await expect(page.locator('text=Search our knowledge base for help articles.').first()).toBeVisible();

    // Verify Interactive Tour buttons
    const tourBtn = page.locator('button', { hasText: 'Tour: Set up your store' });
    await expect(tourBtn).toBeVisible();

    // Click to start walkthrough
    await tourBtn.click();

    // Validate the walkthrough overlay or bubble is visible
    await expect(page.locator('text=Activate your AI Support Agent').first()).toBeVisible();
    await expect(page.locator('text=Search here for any topic.').first()).toBeVisible();
  });

  test('Persona: Advanced User verifies Swagger UI loads in Next.js', async ({ page }) => {
    await page.goto('/api-docs');
    await expect(page.locator('text=Advanced:').first()).toBeVisible();
    await expect(page.locator('text=This section is for developers directly integrating with our APIs').first()).toBeVisible();
    // Wait for Swagger UI mock or text
    await expect(page.locator('text=API Reference').first()).toBeVisible();
  });
});
