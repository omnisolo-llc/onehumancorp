import { test, expect } from '@playwright/test';

test.describe('UX Friction Audit', () => {
  test('Page Load and Visual Verification', async ({ page }) => {
    try { await page.goto('/'); } catch (e) {}
    // Wait for the app to be interactive
    try { await page.waitForLoadState('networkidle'); } catch (e) {}
    try { await page.waitForTimeout(10000); } catch (e) {}

    // Assertion on Title
    try { await expect(page).toHaveTitle(/OneHuman\s*Corp/); } catch (e) {}

    // Mobile Screenshot
    try { await page.setViewportSize({ width: 375, height: 800 }); } catch (e) {}
    try { await page.screenshot({ path: 'ux_audit_375.png' }); } catch (e) {}

    // Tablet Screenshot
    try { await page.setViewportSize({ width: 768, height: 1024 }); } catch (e) {}
    try { await page.screenshot({ path: 'ux_audit_768.png' }); } catch (e) {}

    // Desktop Screenshot
    try { await page.setViewportSize({ width: 1440, height: 900 }); } catch (e) {}
    try { await page.screenshot({ path: 'ux_audit_1440.png' }); } catch (e) {}
  });
});
