import { test, expect } from '@playwright/test';

test.describe('Viral Embeddable Offer Generator', () => {
  test('should load the generator, update preview, and hit the API', async ({ page }) => {
    await page.goto('file://' + process.cwd() + '/src/ui/tauri/src/ui/viral-embeddable-offer.html');

    // Verify UI components
    await expect(page.locator('h1')).toHaveText('Viral Embeddable Offer');
    const offerTitleInput = page.locator('#offerTitle');
    const offerDescInput = page.locator('#offerDesc');
    const generateBtn = page.locator('#generateBtn');

    await expect(offerTitleInput).toBeVisible();
    await expect(generateBtn).toBeVisible();

    // Fill form
    await offerTitleInput.fill('');
    await offerTitleInput.fill('Special VIP Deal');
    await offerDescInput.fill('');
    await offerDescInput.fill('Get 50% Off');

    // Verify preview updates
    await expect(page.locator('#previewTitle')).toHaveText('Special VIP Deal');
    await expect(page.locator('#previewDesc')).toHaveText('Get 50% Off');

    page.on('dialog', dialog => dialog.accept());

    await generateBtn.click();

    // Verify result area becomes visible
    const resultArea = page.locator('#result-area');
    await expect(resultArea).toBeVisible({ timeout: 5000 });

    const embedCode = page.locator('#embedCode');
    await expect(embedCode).toHaveValue(/Special VIP Deal/);
    await expect(embedCode).toHaveValue(/Get 50% Off/);
  });
});
