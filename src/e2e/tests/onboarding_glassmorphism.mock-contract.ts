import { test, expect } from '@playwright/test';

test.describe('Onboarding Glassmorphism UI Audit', () => {

  test.beforeEach(async ({ page }) => {
    const fs = require('fs');
    const path = require('path');
    const tauriUiDir = path.join(process.cwd(), 'src/ui/tauri/src/ui');
    await page.route('**/setup.html', async route => {
        const content = fs.readFileSync(path.join(tauriUiDir, 'setup.html'), 'utf-8');
        await route.fulfill({ contentType: 'text/html', body: content });
    });
  });




  test('onboarding container matches OHC glassmorphism light mode spec', async ({ page }) => {
    await page.goto('http://mock/setup.html');
    const container = page.locator('.container');
    await expect(container).toBeVisible();

    await page.emulateMedia({ colorScheme: 'light' });
    await page.waitForTimeout(500);

    const bgColor = await container.evaluate((el) => window.getComputedStyle(el).backgroundColor);
    expect(bgColor).toMatch(/rgba\(\s*255\s*,\s*255\s*,\s*255\s*,\s*0\.65\s*\)|rgba\(\s*252\s*,\s*252\s*,\s*252\s*,\s*0\.65\s*\)/);

    const backdropFilter = await container.evaluate((el) => window.getComputedStyle(el).backdropFilter);
    expect(backdropFilter).toContain('blur(30px)');
    expect(backdropFilter).toMatch(/saturate\((210%|2\.1)\)/);

    const border = await container.evaluate((el) => window.getComputedStyle(el).border);
    expect(border).toContain('1px solid rgba(255, 255, 255, 0.4)');

    const borderRadius = await container.evaluate((el) => window.getComputedStyle(el).borderRadius);
    expect(borderRadius).toBe('16px');
  });

  test('onboarding container matches OHC glassmorphism dark mode spec', async ({ page }) => {
    await page.goto('http://mock/setup.html');
    const container = page.locator('.container');
    await expect(container).toBeVisible();

    await page.emulateMedia({ colorScheme: 'dark' });
    await page.waitForTimeout(500);

    const bgColor = await container.evaluate((el) => window.getComputedStyle(el).backgroundColor);
    expect(bgColor).toMatch(/rgba\(22,\s*22,\s*26,\s*0\.7\)/);

    const backdropFilter = await container.evaluate((el) => window.getComputedStyle(el).backdropFilter);
    expect(backdropFilter).toContain('blur(30px)');
    expect(backdropFilter).toMatch(/saturate\((210%|2\.1)\)/);

    const border = await container.evaluate((el) => window.getComputedStyle(el).border);
    // Dark mode border is 1px solid rgba(255, 255, 255, 0.1)
    expect(border).toContain('1px solid rgba(255, 255, 255, 0.1)');
  });

  test('onboarding inputs and buttons use 8px border radius', async ({ page }) => {
    await page.goto('http://mock/setup.html');

    // Start wizard to reach an input
    await page.getByText('Step-by-Step Setup').click();

    // Check an input
    const input = page.locator('.container input').first();
    const borderRadiusInput = await input.evaluate((el) => window.getComputedStyle(el).borderRadius);
    expect(borderRadiusInput).toBe('8px');

    // Check back/forward buttons or action buttons
    // The very first button might be the `setup-nav-button` which intentionally has a 999px border-radius,
    // so we skip that one and check the general wizard continuation buttons.
    const button = page.locator('.container button:not(.setup-nav-button)').first();
    const borderRadiusButton = await button.evaluate((el) => window.getComputedStyle(el).borderRadius);
    expect(borderRadiusButton).toBe('8px');
  });
});
