import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Viral Secret Menu Generator Loop - Extended', () => {
  test('should generate a viral secret menu link and have correct initial state', async ({ page }) => {
    await adminPage(page, async () => {
      await page.goto('/dashboard');
      await page.click('a#viral-secret-menu-link');
      await expect(page.locator('h1', { hasText: 'Viral Secret Menu Generator 🤫' })).toBeVisible();

      // Check initial state
      await expect(page.locator('input#itemName')).toHaveValue('');
      await expect(page.locator('input#itemDesc')).toHaveValue('');
      await expect(page.locator('input#accessCode')).toHaveValue('');
      await expect(page.locator('input#sharesReq')).toHaveValue('');
    });
  });

  test('preview iframe updates on input change', async ({ page }) => {
    await adminPage(page, async () => {
      await page.goto('/viral-secret-menu-generator');

      await page.fill('input#itemName', 'Test Item');

      const iframeElement = page.locator('iframe#previewFrame');
      await expect(iframeElement).toBeVisible();
      const src = await iframeElement.getAttribute('src');
      expect(src).toContain('item_name=Test%20Item');
    });
  });

  test('share link text updates on input change', async ({ page }) => {
    await adminPage(page, async () => {
      await page.goto('/viral-secret-menu-generator');

      await page.fill('input#accessCode', 'CODE123');

      const linkText = await page.locator('#shareLink').innerText();
      expect(linkText).toContain('access_code=CODE123');
    });
  });

  test('copy button returns to original state', async ({ page }) => {
    await adminPage(page, async () => {
      await page.goto('/viral-secret-menu-generator');

      await page.click('button#copyBtn');
      await expect(page.locator('button#copyBtn')).toHaveText('Copied!', { timeout: 10000 });
      // The original state returns after 2 seconds based on code
      await expect(page.locator('button#copyBtn')).toHaveText('Copy Link', { timeout: 5000 });
    });
  });

  test('iframe renders correctly with parameters', async ({ page, request }) => {
    await adminPage(page, async () => {
      // We can also hit the iframe route directly to verify the static render logic
      const response = await request.get('/api/v1/growth/secret-menu/embed?item_name=Burger&shares_req=5');
      expect(response.status()).toBe(200);

      const text = await response.text();
      expect(text).toContain('Burger');
      expect(text).toContain('5');
      expect(text).toContain('shares completed to unlock');
    });
  });
});
