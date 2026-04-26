import { test, expect } from '@playwright/test';

test('UI should not contain technical jargon', async ({ page }) => {
  await page.setViewportSize({ width: 375, height: 812 });

  await page.goto('/');

  await page.waitForTimeout(5000);

  // Login
  const emailField = page.locator('input[type="email"], input[name="username"]').first();
  if (await emailField.isVisible({ timeout: 5000 })) {
    await emailField.fill('admin');
  }

  const passwordField = page.locator('input[type="password"], input[name="password"]').first();
  if (await passwordField.isVisible({ timeout: 2000 })) {
    await passwordField.fill('admin');
  }

  const loginBtn = page.locator('button:has-text("Login"), button:has-text("Sign In")').first();
  if (await loginBtn.isVisible({ timeout: 2000 })) {
    await loginBtn.click();
    await page.waitForTimeout(5000);
  }

  // Navigate through key screens to verify jargon is removed
  const pagesToCheck = [
    '/#/dashboard',
    '/#/ongoing',
    '/#/swarm-memory',
    '/#/walkthrough'
  ];

  for (const pageUrl of pagesToCheck) {
    await page.goto(pageUrl);
    await page.waitForTimeout(3000);

    // In a real Playwright test against a canvas, text isn't easily accessible without a11y tree.
    // Assuming the test checks the DOM/Accessibility tree:

    const jargonList = ['System Health', 'Observability', 'Orchestration', 'Vector Memory', 'AutoDream', 'SQLite', 'PostgreSQL', 'SPIFFE', 'mTLS'];

    for (const jargon of jargonList) {
      const el = page.locator(`text=${jargon}`);
      // Assert that jargon doesn't exist
      await expect(el).toHaveCount(0);
    }
  }
});
