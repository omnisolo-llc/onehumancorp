import { test, expect } from './fixtures';

test('Generate visual screenshots for User Guide', async ({ page }) => {
  await page.goto('/dashboard');
  await page.waitForLoadState('domcontentloaded');
  await page.waitForTimeout(5000); // Give the UI time to settle

  // Mobile Screenshot
  await page.viewportSize();
  await page.screenshot({ path: 'docs/app/ux_audit_375.png' });

  // Tablet Screenshot
  await page.viewportSize();
  await page.screenshot({ path: 'docs/app/ux_audit_768.png' });

  // Desktop Screenshot
  await page.viewportSize();
  await page.screenshot({ path: 'docs/app/ux_audit_1440.png' });
});
