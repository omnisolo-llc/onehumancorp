import { test, expect } from './fixtures';

test.describe('Agent Harness Security E2E', () => {
  test.beforeEach(async ({ page }) => {
    // Start from home page and navigate to diagnostics
    await page.goto('/');
    await page.click('nav a:has-text("Setup")'); // Using Setup as a proxy for diagnostics if direct link is missing
    await page.goto('/diagnostics');
  });

  test('should display sandbox security diagnostics section', async ({ page }) => {
    const section = page.locator('#sandbox-security-diagnostics');
    await expect(section).toBeVisible();
    await expect(section.locator('h2')).toContainText('Sandbox Security');
  });

  test('should show correct AST validation status', async ({ page }) => {
    const status = page.locator('#ast-validation-status');
    await expect(status).toContainText('AST Validation: Enhanced');
    await expect(status).toContainText('Blocking subshells, redirections');
  });

  test('should show correct Linux sandbox status', async ({ page }) => {
    const status = page.locator('#bwrap-status');
    await expect(status).toContainText('Linux Sandbox: Bubblewrap enabled');
  });

  test('should handle AST validation test trigger', async ({ page }) => {
    await page.click('#test-ast-btn');
    const result = page.locator('#diagnostics-result');
    await expect(result).toContainText('AST validation test: Access denied');
  });

  test('should handle Sandbox isolation test trigger', async ({ page }) => {
    await page.click('#test-sandbox-btn');
    const result = page.locator('#diagnostics-result');
    await expect(result).toContainText('Sandbox violation recorded');
  });
});
