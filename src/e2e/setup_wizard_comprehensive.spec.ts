import { test, expect } from './fixtures';

test.describe('Business Setup Wizard Comprehensive Flow', () => {
  test('uses the new AI instant generation flow', async ({ page }) => {
    // Intercept generate API call
    await page.route('**/api/v1/builder/generate', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
            pages: [{
                blocks: [
                    { block_type: 'HeroBlock', content: { headline: "Alex's Art", subtitle: "Creative services." } }
                ]
            }]
        })
      });
    });

    await page.goto('/login');
    // Start Business Setup from Login screen
    await page.getByRole('button', { name: 'Start Business Setup' }).click();

    await expect(page.getByRole('heading', { name: 'Tell us about your business' })).toBeVisible();
    await page.getByPlaceholder('e.g. I bake custom vegan cakes').fill('I do creative portraits as Alex Art');

    // Generate Business
    await page.getByRole('button', { name: /Generate My Business/ }).click();

    // Verify it reached generation step and subsequently the builder preview
    await expect(page.getByText('Designing your storefront...')).toBeVisible();

    // After generation is mocked, we expect the storefront-builder-screen to be shown
    await expect(page.getByRole('heading', { name: 'Edit Website' })).toBeVisible({ timeout: 15000 });
  });
});
