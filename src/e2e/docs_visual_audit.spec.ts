import { test, expect } from '@playwright/test';

test('Generate visual screenshots for User Guide', async ({ page }) => {
  await page.goto('/');
  await page.waitForLoadState('domcontentloaded');
  await page.waitForTimeout(5000); // Give the UI time to settle

  // Mobile Screenshot
  await page.setViewportSize({ width: 375, height: 800 });
  await page.screenshot({ path: 'docs/app/ux_audit_375.png' });

  // Tablet Screenshot
  await page.setViewportSize({ width: 768, height: 1024 });
  await page.screenshot({ path: 'docs/app/ux_audit_768.png' });

  // Desktop Screenshot
  await page.setViewportSize({ width: 1440, height: 900 });
  await page.screenshot({ path: 'docs/app/ux_audit_1440.png' });
});
