import { test, expect } from './fixtures';

test.describe('Automated Review Campaigns Flow', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('user can open automated review campaigns and generate a draft', async ({ page }) => {
    // Navigate to the Review Campaigns screen
    await page.getByRole('button', { name: 'Review Campaigns ⭐️' }).click();

    await expect(page.getByRole('heading', { name: 'Automated Review Campaigns ⭐️' })).toBeVisible();

    // Fill the inputs
    await page.locator('#review-product').fill('Signature Coffee Blend');
    await page.locator('#review-audience').selectOption('loyal');

    // Generate Campaign
    await page.getByRole('button', { name: 'Drafting with AI...' }).click();

    // Verify the result
    const resultCard = page.locator('#review-result');
    await expect(resultCard).toBeVisible();

    const resultText = await resultCard.textContent();
    expect(resultText).toContain('Signature Coffee Blend');
    expect(resultText).toContain('How are you loving your Signature Coffee Blend?');

    // Send the campaign
    const sendBtn = page.locator('#review-send-btn');
    await expect(sendBtn).toContainText('Send to Audience (12 Customers)');
    await sendBtn.click();

    await expect(page.locator('#review-sent-msg')).toBeVisible();
    await expect(page.locator('#review-sent-msg')).toContainText('Campaign sent to 12 customers!');
  });
});
