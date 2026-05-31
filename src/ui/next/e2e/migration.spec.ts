import { test, expect } from '@playwright/test';
import { adminPage } from '../../../e2e/fixtures';

test.describe('Platform Migration Engine', () => {
  test('should allow merchant to migrate from legacy platform', async ({ page }) => {
    await page.goto('/onboarding');
    await expect(page.locator('text=Tell us about your business')).toBeVisible();
    await page.click('text=migrate an existing store');
    await expect(page.locator('text=Migrate your store')).toBeVisible();
    await expect(page.locator('text=Store URL')).toBeVisible();
    await page.fill('input[type="url"]', 'https://priyasboutique.myshopify.com');
    await page.click('button:has-text("Start Migration")');
    await expect(page.locator('text=✨ AI is packing up your shop...')).toBeVisible();
    await expect(page.locator('text=Review Details')).toBeVisible();
    await expect(page.locator('label:text("Business Name") + input')).toHaveValue("Priya's Boutique");
  });
});
