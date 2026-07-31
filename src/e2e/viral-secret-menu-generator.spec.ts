import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Viral Secret Menu Generator Loop', () => {
  test('should generate a viral secret menu link', async ({ page }) => {
    // Navigate to dashboard
    await adminPage(page, async () => {
      await page.goto('/dashboard');

      // Click the new Viral Secret Menu Generator link
      await page.click('a#viral-secret-menu-link');

      // Wait for the secret menu generator page to load
      await expect(page.locator('h1', { hasText: 'Viral Secret Menu Generator 🤫' })).toBeVisible();

      // Fill in the form
      await page.fill('input#itemName', 'Double Smash Burger');
      await page.fill('input#itemDesc', 'Extra cheese, extra smash.');
      await page.fill('input#accessCode', 'SMASHX2');
      await page.fill('input#sharesReq', '4');

      // Check that the preview iframe updates with correct parameters
      const iframeElement = page.locator('iframe#previewFrame');
      await expect(iframeElement).toBeVisible();

      const src = await iframeElement.getAttribute('src');
      expect(src).toContain('item_name=Double%20Smash%20Burger');
      expect(src).toContain('item_desc=Extra%20cheese%2C%20extra%20smash.');
      expect(src).toContain('access_code=SMASHX2');
      expect(src).toContain('shares_req=4');

      // Test the copy link functionality
      await page.click('button#copyBtn');
      await expect(page.locator('button#copyBtn')).toHaveText('Copied!', { timeout: 10000 });

      const linkText = await page.locator('#shareLink').innerText();
      expect(linkText).toContain('/api/v1/growth/secret-menu/embed');
    });
  });
});
