import { test, expect } from '@playwright/test';

test.describe('Onboarding Glassmorphism UI Audit', () => {

  test('onboarding container matches OHC glassmorphism light mode spec', async ({ page }) => {
    await page.goto('http://127.0.0.1:18789/api/ui/setup.html');
    const container = page.locator('.container');
    await expect(container).toBeVisible();

    await page.emulateMedia({ colorScheme: 'light' });
    await page.waitForTimeout(500);

    const bgColor = await container.evaluate((el) => window.getComputedStyle(el).backgroundColor);
    expect(bgColor).toBe('rgba(255, 255, 255, 0.65)');

    const backdropFilter = await container.evaluate((el) => window.getComputedStyle(el).backdropFilter);
    expect(backdropFilter).toContain('blur(30px)');
    expect(backdropFilter).toMatch(/saturate\((210%|2\.1)\)/);

    const border = await container.evaluate((el) => window.getComputedStyle(el).border);
    expect(border).toContain('1px solid rgba(255, 255, 255, 0.4)');

    const borderRadius = await container.evaluate((el) => window.getComputedStyle(el).borderRadius);
    expect(borderRadius).toBe('16px');
  });

  test('onboarding container matches OHC glassmorphism dark mode spec', async ({ page }) => {
    await page.goto('http://127.0.0.1:18789/api/ui/setup.html');
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
    await page.goto('http://127.0.0.1:18789/api/ui/setup.html');

    // Start wizard to reach an input
    await page.getByText('Start My Business').click();

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
