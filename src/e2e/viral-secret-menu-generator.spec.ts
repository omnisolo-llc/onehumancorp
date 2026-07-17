import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Viral Secret Menu Generator Loop', () => {
  test('should generate a viral secret menu link', async ({ page }) => {
    // Navigate to dashboard
    await adminPage(page, async () => {
      await page.goto('/dashboard.html');

      // Click the new Viral Secret Menu Generator link
      await page.click('a#viral-secret-menu-link');

      // Wait for the secret menu generator page to load
      await expect(page.locator('h1', { hasText: 'Viral Secret Menu Generator 🤫' })).toBeVisible();

      // Fill in the form
      await page.fill('input#itemName', 'Double Smash Burger');
      await page.fill('input#itemDesc', 'Extra cheese, extra smash.');
      await page.fill('input#accessCode', 'SMASHX2');
      await page.fill('input#sharesReq', '4');

      // Check that the preview updates
      await expect(page.locator('#previewTitle')).toHaveText('Double Smash Burger');
      await expect(page.locator('#previewDesc')).toHaveText('Extra cheese, extra smash.');
      await expect(page.locator('#previewCode')).toHaveText('SMASHX2');
      await expect(page.locator('#shareCountText')).toHaveText('4');
      await expect(page.locator('#previewShares')).toHaveText('4');
      await expect(page.locator('#previewSharesSub')).toHaveText('4');
      await expect(page.locator('#previewSharesTotal')).toHaveText('4');

      // Test the copy link functionality
      await page.click('button#copyBtn');
      await expect(page.locator('button#copyBtn')).toHaveText('Copied!');
    });
  });
});
