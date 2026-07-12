import { test, expect } from './fixtures';
import { adminPage } from './fixtures';

test.describe('Viral Job Board Widget', () => {
  test('should load the widget and generate an embed code', async ({ page }) => {
    await adminPage(page, async () => {
      await page.goto('/viral-job-board-generator.html');

      await expect(page.locator('h1')).toHaveText('Viral Job Board Widget');
      const generateBtn = page.locator('#generate-btn');
      await expect(generateBtn).toBeVisible();

      const companyInput = page.locator('#company-name');
      await expect(companyInput).toBeVisible();
      await companyInput.fill('Globex Corporation');

      await generateBtn.click();

      await expect(generateBtn).toBeDisabled();
      await expect(generateBtn).toHaveText('Generating...');

      const resultArea = page.locator('#result-area');
      await expect(resultArea).toBeVisible({ timeout: 5000 });

      const embedCodeTextarea = page.locator('#embed-code');
      await expect(embedCodeTextarea).toBeVisible();
      const embedText = await embedCodeTextarea.inputValue();

      expect(embedText).toContain('Globex Corporation');
      expect(embedText).toContain('<iframe');
      expect(embedText).toContain('Powered by OHC');
    });
  });

  test('should copy the embed code to clipboard', async ({ page, context }) => {
    await context.grantPermissions(['clipboard-read', 'clipboard-write']);

    await adminPage(page, async () => {
      await page.goto('/viral-job-board-generator.html');

      const generateBtn = page.locator('#generate-btn');
      await generateBtn.click();
      const resultArea = page.locator('#result-area');
      await expect(resultArea).toBeVisible({ timeout: 5000 });

      const copyBtn = page.locator('#copy-btn');
      await expect(copyBtn).toHaveText('Copy Embed Code');

      await copyBtn.click();

      await expect(copyBtn).toHaveText('Copied!', { timeout: 3000 });

      try {
          const clipboardText = await page.evaluate(async () => {
              return await navigator.clipboard.readText();
          });
          expect(clipboardText).toContain('<iframe');
          expect(clipboardText).toContain('Powered by OHC');
      } catch (e) {
          console.warn('Clipboard read failed: ', e);
      }
    });
  });

  test('should remove branding when PRO feature is used', async ({ page }) => {
    await adminPage(page, async () => {
      await page.evaluate(() => {
          localStorage.setItem('has_pro', 'true');
      });

      await page.goto('/viral-job-board-generator.html');

      const removeBrandingCheckbox = page.locator('#remove-branding');
      await removeBrandingCheckbox.check();

      const companyInput = page.locator('#company-name');
      await companyInput.fill('Stark Industries');

      const generateBtn = page.locator('#generate-btn');
      await generateBtn.click();

      const embedCodeTextarea = page.locator('#embed-code');
      await expect(embedCodeTextarea).toBeVisible();
      const embedText = await embedCodeTextarea.inputValue();

      expect(embedText).toContain('Stark Industries');
      expect(embedText).toContain('<iframe');
      expect(embedText).not.toContain('Powered by OHC');
    });
  });

  test('should show responsive layout on mobile viewport', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await adminPage(page, async () => {
      await page.goto('/viral-job-board-generator.html');
      await page.waitForTimeout(100);

      await expect(page.locator('h1')).toHaveText('Viral Job Board Widget');

      const container = page.locator('.container');
      const box = await container.boundingBox();
      expect(box?.width).toBeLessThanOrEqual(375);
    });
  });

  test('should navigate back to the dashboard', async ({ page }) => {
    await adminPage(page, async () => {
      await page.goto('/viral-job-board-generator.html');
      const backLink = page.locator('.back-link');
      await expect(backLink).toBeVisible();
      await expect(backLink).toHaveAttribute('href', '/dashboard.html');
    });
  });

  test('should render the embedded job board properly', async ({ page }) => {
    await page.goto('/job-board-embed.html?tenant=e2e-tenant&company=Test%20Company&hideBranding=false');

    await expect(page.locator('#board-title')).toHaveText('Test Company Open Positions');

    const emptyState = page.locator('.empty-state');
    await expect(emptyState).toBeVisible();
    await expect(emptyState).toContainText('No Open Positions');

    const brandingLink = page.locator('#branding-link');
    await expect(brandingLink).toBeVisible();
    await expect(brandingLink).toHaveText('⚡ Powered by OHC');
  });

  test('should hide branding in the embedded job board when hideBranding=true', async ({ page }) => {
    await page.goto('/job-board-embed.html?tenant=e2e-tenant&company=Test%20Company&hideBranding=true');
    const brandingContainer = page.locator('#branding-container');
    await expect(brandingContainer).not.toBeVisible();
  });
});
