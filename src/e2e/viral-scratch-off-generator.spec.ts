import { test, expect } from '@playwright/test';

test.describe('Viral Scratch-Off Generator', () => {
  // We can just visit the generator UI page and test basic functionality
  test('generator UI loads and creates embed code', async ({ page }) => {
    // Navigate to the generator
    await page.goto('/viral-scratch-off-generator.html');

    // Check elements
    await expect(page.locator('h1')).toContainText('Scratch-Off Generator');
    await expect(page.locator('#offer-text')).toBeVisible();
    await expect(page.locator('#generate-btn')).toBeVisible();

    // Type a custom offer
    await page.fill('#offer-text', '30% OFF OHC');
    await page.click('input[value="dark"]');

    // Click Generate
    await page.click('#generate-btn');

    // Preview section should appear
    await expect(page.locator('#preview-section')).toBeVisible();

    // The embed code should contain the correct URL parameters
    const code = await page.locator('#embed-code').textContent();
    expect(code).toContain('type=scratch_off');
    expect(code).toContain('theme=dark');
    expect(code).toContain('offer=30%25+OFF+OHC'); // url encoded
  });
});
