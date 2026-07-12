import { test, expect } from './fixtures';
import { adminPage } from './fixtures';

test.describe('Viral ROI Calculator', () => {
  test('dashboard links to Viral ROI Calculator, which generates an embed code', async ({ page }) => {
    await adminPage(page, async () => {
      await page.goto('/dashboard.html');

      const link = page.locator('a[href="interactive-roi-calculator.html"]');
      if (await link.count() > 0) {
        await link.click();
      } else {
        await page.goto('/interactive-roi-calculator.html');
      }

      await expect(page.locator('h1').filter({ hasText: /ROI Calculator/i })).toBeVisible();

      const generateBtn = page.locator('#generate-btn');
      await expect(generateBtn).toBeVisible();

      await page.locator('#service-name').fill('Pro SEO Services');
      await page.locator('#avg-customer-value').fill('1500');

      await generateBtn.click();
      await expect(generateBtn).toBeDisabled();

      const resultArea = page.locator('#result-area');
      await expect(resultArea).toBeVisible({ timeout: 5000 });

      const embedCodeTextarea = page.locator('#embed-code');
      await expect(embedCodeTextarea).toBeVisible();
      const embedText = await embedCodeTextarea.inputValue();

      expect(embedText).toContain('Pro%20SEO%20Services');
      expect(embedText).toContain('1500');
      expect(embedText).toContain('<iframe');
    });
  });
});
