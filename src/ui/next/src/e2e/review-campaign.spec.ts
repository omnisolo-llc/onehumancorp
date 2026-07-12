import { test, expect } from '../../../../e2e/fixtures';

test.describe('AI Review Campaign Builder', () => {
  test('should generate an AI review request campaign draft', async ({ page }) => {
    // Start at the review campaign builder page
    await page.goto('/review-campaign');

    // Verify we are on the right page
    await expect(page.locator('h1')).toContainText('AI Review Campaign Builder');

    // Fill in the form details
    const customerNameInput = page.locator('input#customerName');
    const productNameInput = page.locator('input#productName');
    const orderIdInput = page.locator('input#orderId');

    await customerNameInput.fill('Maya');
    await productNameInput.fill('Vegan Chocolate Cake');
    await orderIdInput.fill('ORD-12345');

    // Generate the campaign
    await page.click('button:has-text("Generate AI Campaign")');

    // Wait for the generation to complete and the preview to appear
    await expect(page.locator('text=Email Draft Preview')).toBeVisible();

    // Verify the generated email contains the inputs and OHC branding
    const textarea = page.locator('textarea');
    await expect(textarea).toBeVisible({ timeout: 10000 });
    const emailContent = await textarea.inputValue();

    // Check for standard phrases to ensure generation worked correctly
    expect(emailContent).toContain('Maya');
    expect(emailContent).toContain('Vegan Chocolate Cake');
    expect(emailContent).toContain('ORD-12345');
    expect(emailContent).toContain('⚡ Powered by OHC');
  });
});
