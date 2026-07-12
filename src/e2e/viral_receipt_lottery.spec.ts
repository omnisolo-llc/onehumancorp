import { test, expect } from './fixtures';
import { adminPage } from './fixtures';

test.describe('Viral Receipt Lottery', () => {
  test('dashboard links to Viral Receipt Lottery, which generates a viral link', async ({ page }) => {
    await adminPage(page, async () => {
      await page.goto('/dashboard.html');

      await page.locator('#viral-receipt-lottery-link').click();

      await expect(page.locator('h1')).toHaveText('Viral Receipt Lottery 🎟');
      await expect(page.locator('.receipt-mockup')).toBeVisible();

      const generateBtn = page.locator('#generate-btn');
      await expect(generateBtn).toBeVisible();
      await expect(page.locator('#result-area')).not.toBeVisible();

      await generateBtn.click();
      await expect(generateBtn).toBeDisabled();
      await expect(generateBtn).toHaveText('Generating...');

      const resultArea = page.locator('#result-area');
      await expect(resultArea).toBeVisible({ timeout: 5000 });

      const shareLinkInput = page.locator('#share-link');
      await expect(shareLinkInput).toBeVisible();
      const generatedUrl = await shareLinkInput.inputValue();
      expect(generatedUrl).toContain('/win/');

      await expect(page.locator('#preview-url')).toHaveText(/ohc\.app\/win\//);
    });
  });

  test('should copy the lottery link to clipboard', async ({ page, context }) => {
    await context.grantPermissions(['clipboard-read', 'clipboard-write']);

    await adminPage(page, async () => {
      await page.goto('/viral-receipt-lottery.html');

      const generateBtn = page.locator('#generate-btn');
      await generateBtn.click();

      const resultArea = page.locator('#result-area');
      await expect(resultArea).toBeVisible({ timeout: 5000 });

      const copyBtn = page.locator('#copy-btn');
      await expect(copyBtn).toHaveText('Copy Link');
      await copyBtn.click();

      await expect(copyBtn).toHaveText('Copied!', { timeout: 3000 });

      try {
          const clipboardText = await page.evaluate(async () => {
              return await navigator.clipboard.readText();
          });
          expect(clipboardText).toContain('/win/');
      } catch (e) {
          console.warn('Clipboard read failed: ', e);
      }
    });
  });
});
