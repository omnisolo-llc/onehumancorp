import * as fs from 'fs';
import * as path from 'path';
import { test, expect } from '@playwright/test';

test.describe('Onboarding Instant Build - Additional Details', () => {
  test.beforeEach(async ({ page }) => {
    // Clear local storage to ensure fresh state
    await page.addInitScript(() => {
      window.localStorage.clear();
    });

    const workspaceRoot = process.env.TEST_WORKSPACE
        ? path.join(process.env.TEST_SRCDIR || process.cwd(), process.env.TEST_WORKSPACE)
        : process.cwd();
    const tauriUiDir = path.join(workspaceRoot, 'src/ui/tauri/src/ui');
    await page.route('**/setup.html', async route => {
        const fileContent = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: fileContent });
    });
    await page.addInitScript(() => {
      window.__TAURI__ = {
        core: {
          invoke: async () => null
        }
      };
    });

    // Clear local storage to ensure fresh state
    await page.addInitScript(() => {
      window.localStorage.clear();
    });
  });

  test('Instant Build UI loads correctly', async ({ page }) => {
    await page.goto('/setup.html');
  });

  test('Instant bio view renders', async ({ page }) => {
    await page.goto('/setup.html');
    // Verify navigating to bio view
    await expect(page.locator('#instant-bio')).toBeVisible();
  });

  test('Submitting an empty bio disables the Generate Storefront button', async ({ page }) => {
    await page.goto('/setup.html');

    const generateButton = page.locator('#generate-storefront-btn');
    await expect(generateButton).toBeVisible();

    const bioInput = page.locator('#instant-bio');
    await bioInput.fill('   ');

    await expect(generateButton).toBeDisabled();
  });

  test('Filling a bio enables the Generate Storefront button', async ({ page }) => {
    await page.goto('/setup.html');

    const generateButton = page.locator('#generate-storefront-btn');
    const bioInput = page.locator('#instant-bio');

    await bioInput.fill('I am a baker in NYC');

    await expect(generateButton).not.toBeDisabled();
  });

  test('Filling a bio and image URL enables the Generate Storefront button', async ({ page }) => {
    await page.goto('/setup.html');

    const generateButton = page.locator('#generate-storefront-btn');
    const bioInput = page.locator('#instant-bio');
    const urlInput = page.locator('#instant-image-url');

    await bioInput.fill('I run a coffee shop');
    await urlInput.fill('https://example.com/shop.jpg');

    await expect(generateButton).not.toBeDisabled();
  });
});
