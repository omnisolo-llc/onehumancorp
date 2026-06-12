import { test, expect } from './fixtures';
import * as path from 'path';
import * as fs from 'fs';

test.describe('Glassmorphism UI Audit', () => {
  let tauriUiDir: string;

  test.beforeAll(() => {
    const workspaceRoot = process.env.TEST_WORKSPACE
        ? path.join(process.env.TEST_SRCDIR!, process.env.TEST_WORKSPACE)
        : process.cwd();
    tauriUiDir = path.join(workspaceRoot, 'src/ui/tauri/src/ui');
  });

  test.beforeEach(async ({ page }) => {
    await page.route('**/*.html', async route => {
        const url = new URL(route.request().url());
        const filename = path.basename(url.pathname);
        const filepath = path.join(tauriUiDir, filename);
        if (fs.existsSync(filepath)) {
            const content = fs.readFileSync(filepath, 'utf-8');
            await route.fulfill({ contentType: 'text/html', body: content });
        } else {
            await route.continue();
        }
    });
  });

  test('Verify setup page uses 16px border radius', async ({ page }) => {
    await page.goto('http://mock/setup.html');
    await page.waitForLoadState('networkidle');
    const container = page.locator('.container.glassmorphism').first();
    const borderRadius = await container.evaluate((el) => {
      return window.getComputedStyle(el).borderRadius;
    });
    expect(borderRadius).toBe('16px');
  });

  test('Verify input elements use 8px border radius', async ({ page }) => {
    await page.goto('http://mock/setup.html');
    await page.waitForLoadState('networkidle');
    const input = page.locator('input[type="text"]').first();
    const borderRadius = await input.evaluate((el) => {
      return window.getComputedStyle(el).borderRadius;
    });
    expect(borderRadius).toBe('8px');
  });

  test('Verify dashboard buttons use 8px border radius', async ({ page }) => {
    await page.goto('http://mock/dashboard.html');
    await page.waitForLoadState('networkidle');
    const button = page.locator('button').first();
    const borderRadius = await button.evaluate((el) => {
      return window.getComputedStyle(el).borderRadius;
    });
    expect(borderRadius).toBe('8px');
  });

  test('Verify POS buttons use 8px border radius', async ({ page }) => {
    await page.goto('http://mock/pos.html');
    await page.waitForLoadState('networkidle');
    const button = page.locator('.charge-btn').first();
    const borderRadius = await button.evaluate((el) => {
      return window.getComputedStyle(el).borderRadius;
    });
    expect(borderRadius).toBe('8px');
  });

  test('Verify Quote page containers use 16px border radius', async ({ page }) => {
    await page.goto('http://mock/quote.html');
    await page.waitForLoadState('networkidle');
    const container = page.locator('.glass-card').first();
    const borderRadius = await container.evaluate((el) => {
      return window.getComputedStyle(el).borderRadius;
    });
    expect(borderRadius).toBe('16px');
  });
});
