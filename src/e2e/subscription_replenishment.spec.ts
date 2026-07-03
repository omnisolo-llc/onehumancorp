import { test, expect, adminPage } from './fixtures';


test.describe('Subscription Replenishment Feature', () => {
  test('should verify subscription replenishment flow', async ({ page, context }) => {
    // 1. Setup admin session
    page = await adminPage(page, context);

    // 2. Dashboard should have the Subscription Replenishment card
    const subscriptionCard = page.locator('#subscription-replenishment-section');
    await expect(subscriptionCard).toBeVisible();

    // 3. Navigate to subscription generator
    await page.locator('a[href="subscription-generator.html"]').click();

    // 4. Assert heading is visible
    await expect(page.locator('h1:has-text("Subscribe & Save Campaign")')).toBeVisible();

    // 5. Fill out the form
    await page.fill('input#product-name', 'Artisan Honey');
    await page.fill('input#discount', '20');
    await page.selectOption('select#frequency', 'monthly');

    // 6. Click generate AI campaign
    const generateBtn = page.locator('button:has-text("Generate AI Campaign")');
    await expect(generateBtn).toBeEnabled();
    await generateBtn.click();

    // 7. Wait for preview box to appear and verify contents
    const previewBox = page.locator('#preview-box');
    await expect(previewBox).toBeVisible();
    await expect(page.locator('pre#preview-text')).toContainText('Artisan Honey');
    await expect(page.locator('pre#preview-text')).toContainText('20%');

    // 8. Click send campaign
    const sendBtn = page.locator('button:has-text("Send Campaign")');
    await expect(sendBtn).toBeEnabled();
    await sendBtn.click();

    // 9. Verify success message
    await expect(page.locator('#success-msg')).toBeVisible({ timeout: 5000 });
  });
});