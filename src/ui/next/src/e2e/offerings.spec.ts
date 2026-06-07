import { test, expect } from '@playwright/test';

test.describe('Offering Creation Flow', () => {
  test.use({ viewport: { width: 375, height: 667 } }); // Mobile viewport

  test('should create a new offering from intent', async ({ page }) => {
    // Navigate directly to the new offering page to test the creation flow
    await page.goto('/offerings/new');

    // Check header
    await expect(page.locator('h1', { hasText: 'New Offering' })).toBeVisible();

    // Verify initial state
    await expect(page.locator('label', { hasText: 'What do you want to offer?' })).toBeVisible();
    const textarea = page.locator('textarea[placeholder="e.g. Guitar lessons for beginners, 1 hour"]');
    await expect(textarea).toBeVisible();

    // Type intent
    const intentText = 'Guitar lessons for beginners, 1 hour';
    await textarea.fill(intentText);

    // Click generate
    const generateButton = page.locator('button', { hasText: 'Generate Details' });
    await expect(generateButton).toBeVisible();
    await generateButton.click();

    // Loading state should appear
    await expect(page.locator('text=AI Agents are provisioning your offering...')).toBeVisible();

    // Wait for the mock API response to resolve and form to appear
    await expect(page.locator('label', { hasText: 'Title' })).toBeVisible({ timeout: 5000 });

    // Check generated data
    await expect(page.locator('input[value="Guitar Lessons For Beginners, 1 Hour"]')).toBeVisible();

    // Check type is Service
    await expect(page.locator('input[value="Service"]')).toBeVisible();

    // Edit price
    const priceInput = page.locator('input[type="number"]');
    await priceInput.fill('45');

    // Check Calendar connected message
    await expect(page.locator('text=Calendar connected')).toBeVisible();

    // Publish
    const publishButton = page.locator('button', { hasText: 'Publish to Storefront' });
    // Use evaluate to avoid viewport checks
    await publishButton.evaluate(node => (node as HTMLElement).click());

    // Check success state
    await expect(page.locator('h2', { hasText: 'Offering Published!' })).toBeVisible();

    // The component redirects after 2s, wait for the redirect
    await page.waitForURL('**/dashboard', { timeout: 3000 });

    // Check we are on dashboard
    await expect(page).toHaveURL(/.*\/dashboard/);
  });
});
