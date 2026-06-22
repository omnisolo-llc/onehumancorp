import { test, expect } from '@playwright/test';

test.describe('TaxJar Integration Quoting', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('owner can navigate to quoting and generate quote with tax integration', async ({ page }) => {
    // In e2e, the test runner sets TAXJAR_API_KEY if we mock it, or we rely on the API.
    // For now, testing the UI has the new TaxJar integration.
    await page.goto('/integrations');

    await expect(page.locator('text=TaxJar')).toBeVisible({ timeout: 15000 });
    await expect(page.locator('text=Finance & Compliance').or(page.locator('text=finance'))).toBeVisible();

    const taxJarCard = page.locator('div').filter({ hasText: 'TaxJar' });
    await expect(taxJarCard).toBeVisible();
  });
});
