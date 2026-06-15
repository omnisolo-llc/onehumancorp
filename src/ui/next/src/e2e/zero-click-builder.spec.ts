import { test, expect } from '@playwright/test';

test.describe('Zero Click Builder E2E', () => {
  test('User can use Zero Click Builder to launch a storefront quickly', async ({ page }) => {
    await page.goto('/zero-click-builder');

    // Verify the page title
    await expect(page.locator('h1')).toContainText('Zero-Click Business Generator');

    // Describe the business
    await page.fill('textarea[id="prompt"]', 'I sell amazing custom organic cakes in Portland.');

    // Click Generate
    await page.getByRole('button', { name: 'Generate My Business' }).click();

    // Wait for the storefront to be generated
    await expect(page.locator('h2')).toContainText('Your business is live!', { timeout: 30000 });

    // Verify the storefront overview data
    await expect(page.locator('dt:has-text("Business Name") + dd')).toBeVisible();
    await expect(page.locator('dt:has-text("Store URL") + dd')).toBeVisible();

    const productsCount = await page.locator('dt:has-text("Products Generated") + dd').innerText();
    expect(parseInt(productsCount)).toBeGreaterThan(0);

    await expect(page.locator('dt:has-text("AI Agents Active") + dd')).toContainText('Sales, Ops, Marketing');

    // Test the Go to Dashboard button
    await page.getByRole('button', { name: 'Go to Dashboard' }).click();
    await expect(page).toHaveURL(/\/dashboard/);
  });

  test('Zero Click Builder button handles empty input gracefully', async ({ page }) => {
    await page.goto('/zero-click-builder');

    const generateBtn = page.getByRole('button', { name: 'Generate My Business' });

    // Initially disabled
    await expect(generateBtn).toBeDisabled();

    // Add text -> enabled
    await page.fill('textarea[id="prompt"]', 'Something');
    await expect(generateBtn).toBeEnabled();

    // Clear text -> disabled again
    await page.fill('textarea[id="prompt"]', '');
    await expect(generateBtn).toBeDisabled();
  });

  test('Zero Click Builder simulates generation steps', async ({ page }) => {
    await page.goto('/zero-click-builder');
    await page.fill('textarea[id="prompt"]', 'I sell bikes');
    await page.getByRole('button', { name: 'Generate My Business' }).click();

    // Wait for at least one step to show up
    await expect(page.locator('h3', { hasText: 'Analyzing your business...' })).toBeVisible({ timeout: 5000 });

    // Eventually it reaches "Your business is live!"
    await expect(page.locator('h2')).toContainText('Your business is live!', { timeout: 30000 });
  });

  test('Zero Click Builder generates mock data correctly', async ({ page }) => {
    await page.goto('/zero-click-builder');
    await page.fill('textarea[id="prompt"]', 'I sell toy cars');
    await page.getByRole('button', { name: 'Generate My Business' }).click();

    await expect(page.locator('h2')).toContainText('Your business is live!', { timeout: 30000 });

    // Verify default fallback structure or LLM structure depending on the agent output
    await expect(page.locator('dt:has-text("Products Generated") + dd')).not.toBeEmpty();
  });

  test('Zero Click Builder correctly routes from the main builder page via Instant Build button', async ({ page }) => {
    // This is optional if website-builder has a route to zero-click.
    // The issue description implies the main page might link to this.
    // Testing navigation from website-builder
    await page.goto('/website-builder');
    await page.getByRole('button', { name: 'Instant Build' }).click();

    // It should navigate or activate the Instant Build flow
    await expect(page.locator('h1')).toContainText('Tell us about your business');
  });
});
