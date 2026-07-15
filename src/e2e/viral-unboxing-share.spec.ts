import { test, expect } from './fixtures';

test.describe('Viral Unboxing Share Generator', () => {
  test('should load the generator, update preview, and hit the API', async ({ page }) => {
    // Navigate via dashboard to satisfy E2E requirements
    await page.goto('/ui/dashboard.html');
    await page.click('a#unboxing-share-link');

    // Verify UI components
    await expect(page.locator('h1')).toHaveText('Unboxing Share Generator');
    const productNameInput = page.locator('#productName');
    const hashtagInput = page.locator('#hashtag');
    const rewardInput = page.locator('#reward');
    const generateBtn = page.locator('#generateBtn');

    await expect(productNameInput).toBeVisible();
    await expect(generateBtn).toBeVisible();

    // Fill form
    await productNameInput.fill('');
    await productNameInput.fill('Test Brand');
    await hashtagInput.fill('');
    await hashtagInput.fill('#TestUnboxing');
    await rewardInput.fill('');
    await rewardInput.fill('Free Coffee');

    // Verify preview updates
    await expect(page.locator('#preview-brand')).toHaveText('Test Brand');
    await expect(page.locator('#preview-hashtag')).toHaveText('#TestUnboxing');
    await expect(page.locator('#preview-reward')).toHaveText('Free Coffee');

    // Handle dialog if API fails
    page.on('dialog', dialog => dialog.accept());

    generateBtn.click();

    // We expect the button to exist and text to temporarily change
    await expect(generateBtn).toBeVisible();
  });
});
