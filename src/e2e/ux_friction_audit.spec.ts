import { test, expect } from './fixtures';

test.describe('UX Friction Audit', () => {
  test('Page Load and Visual Verification', async ({ page }) => {
    await page.goto('/');
    // Wait for the app to be interactive
    // await page.waitForLoadState('networkidle');
    await page.waitForTimeout(10000);

    // Assertion on Title
    // await expect(page).toHaveTitle(/OneHuman\s*Corp/);

    // Mobile Screenshot
    await page.setViewportSize({ width: 375, height: 800 });
    await page.screenshot({ path: 'ux_audit_375.png' });

    // Tablet Screenshot
    await page.setViewportSize({ width: 768, height: 1024 });
    await page.screenshot({ path: 'ux_audit_768.png' });

    // Desktop Screenshot
    await page.setViewportSize({ width: 1440, height: 900 });
    await page.screenshot({ path: 'ux_audit_1440.png' });
  });
});
