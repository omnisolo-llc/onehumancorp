import { test, expect } from './fixtures';

test.describe('Canvas Storefront Builder E2E', () => {
  test('should render storefront builder blocks natively', async ({ page }) => {
    await page.goto('/storefront-builder');

    // Check if storefront builder blocks exist
    await expect(page.locator('.builder-block').first()).toBeVisible();

    // Check for some known text based on SmartBlock rendering in page.tsx
    await expect(page.getByText('Get in Touch')).toBeVisible();
    await expect(page.getByText('Our Services')).toBeVisible();
  });
});
