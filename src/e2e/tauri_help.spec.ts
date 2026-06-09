import { test, expect } from '@playwright/test';
import * as path from 'path';
import * as fs from 'fs';

test.describe('Help Center and Contextual Help (Tauri UI)', () => {
  test.beforeEach(async ({ page }) => {
    const workspaceRoot = process.env.TEST_WORKSPACE
        ? path.join(process.env.TEST_SRCDIR!, process.env.TEST_WORKSPACE)
        : process.cwd();
    const tauriUiDir = path.join(workspaceRoot, 'src/ui/tauri/src/ui');

    await page.route('/ui/*', async route => {
      const url = new URL(route.request().url());
      const filename = path.basename(url.pathname);
      const filePath = path.join(tauriUiDir, filename);
      if (fs.existsSync(filePath)) {
        const ext = path.extname(filename);
        let contentType = 'text/html';
        if (ext === '.js') contentType = 'application/javascript';
        if (ext === '.css') contentType = 'text/css';

        const content = fs.readFileSync(filePath, 'utf-8');
        await route.fulfill({ contentType, body: content });
      } else {
        await route.continue();
      }
    });
  });

  test('Persona: Business Owner uses help center and chat', async ({ page }) => {
    // Navigate to local Tauri UI HTML
    await page.goto('/ui/dashboard.html');

    // Wait for page to load fully
    await page.waitForLoadState('networkidle');

    // Check if HelpChat component is accessible
    const chatButton = page.locator('button[aria-label="Open help chat"]');
    await expect(chatButton).toBeVisible();
    await chatButton.click();
    await expect(page.locator('text=Ask AI Help').first()).toBeVisible();

    const input = page.locator('input[placeholder="Ask me anything..."]');
    await input.fill('How do I accept credit cards?');
    await page.locator('button[aria-label="Send message"]').click();

    await expect(page.locator('text=How do I accept credit cards?').first()).toBeVisible();
    await expect(page.locator('text=I am your AI Help Agent!').first()).toBeVisible();
    await expect(page.locator('text=Read the full article').first()).toBeVisible();

    await page.locator('button[aria-label="Close help chat"]').click();

    // Go to /help
    await page.goto('/ui/help.html');
    await expect(page.locator('text=Help Center').first()).toBeVisible();
    await expect(page.locator('text=Getting Started').first()).toBeVisible();

    await page.fill('input[placeholder="Search for help articles and videos..."]', 'paid');
    await expect(page.locator('text=Getting Paid').first()).toBeVisible();
  });

  test('Persona: Business Owner views the Changelog', async ({ page }) => {
    await page.goto('/ui/changelog.html');
    await expect(page.locator('text=Release Notes & Changelog').first()).toBeVisible();
    await expect(page.locator('text=Version 1.0 (Latest)').first()).toBeVisible();
    await expect(page.locator('text=New Features').first()).toBeVisible();
  });

  test('Persona: Developer views the API documentation', async ({ page }) => {
    await page.goto('/ui/api-docs.html');
    await expect(page.locator('text=Advanced:').first()).toBeVisible();
    await expect(page.locator('text=OHC Advanced API Reference').first()).toBeVisible();
  });

  test('Persona: Business Owner interacts with a Tooltip', async ({ page }) => {
    await page.goto('/ui/dashboard.html');
    const shareLink = page.locator('button#generate-link-btn');
    await expect(shareLink).toBeVisible();
    await shareLink.hover();
    await expect(page.locator('text=Click here to share access with a team member.').first()).toBeVisible();
  });
});
