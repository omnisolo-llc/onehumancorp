import { test, expect } from './fixtures';

test.describe('Upsell Recommendation Widget', () => {
  test('displays AI upsell insight and allows generation', async ({ page, loginAs, adminUser }) => {
    await page.goto('http://localhost:3000/dashboard');

    // Wait for the Upsell Recommendation widget title
    const widgetTitle = page.locator('text=AI Upsell Insight');
    await expect(widgetTitle).toBeVisible({ timeout: 15000 });

    // Verify recommendation text
    const recommendationText = page.locator('text=Customers frequently ask for faster delivery. Add a \'Priority Processing\' tier for $15.');
    await expect(recommendationText).toBeVisible();

    // Verify Generate button
    const generateBtn = page.getByRole('button', { name: /Generate Upsell Campaign/i });
    await expect(generateBtn).toBeVisible();

    // Click the generate button
    await generateBtn.click();

    // Verify it changed to generated state
    await expect(page.locator('text=Campaign Generated!')).toBeVisible();
  });
});
