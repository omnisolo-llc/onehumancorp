import { test, expect } from './fixtures';

test.describe('Viral Storefront Footer Loop', () => {
  test('should display the new Powered by OHC CTA on the storefront preview', async ({ page }) => {
    await page.goto('/storefront-builder');

    // Make sure we're seeing the page
    await expect(page.getByText('Preview Mode')).toBeVisible();

    // Verify the new footer CTA text
    await expect(page.getByText('Powered by OHC: Launch your free AI store in 30s')).toBeVisible();

    // Make sure it's a link (actually the parent `<a>` element has the href)
    const ctaLink = page.locator('a[href*="ohc://join?ref="]');
    await expect(ctaLink).toBeVisible();
  });
});
