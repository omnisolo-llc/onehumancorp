import { test, expect } from '@playwright/test';

test.describe('Jarvis Assistant Workstation', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the assistant route.
    await page.goto('/assistant');
  });

  test('should allow creating a new task', async ({ page }) => {
    // Verify the Assistant Workstation shell loaded
    const shell = page.locator('[data-testid="assistant-shell"]');
    await expect(shell).toBeVisible();

    // Verify task prompt input exists
    const input = page.locator('textarea[placeholder*="Ask Assistant"]');
    await expect(input).toBeVisible();

    // Fill a prompt and submit
    await input.fill('Draft an investor update for this week.');
    await page.locator('button', { hasText: 'Run' }).first().click();

    // Wait for the UI to show some loading or task created state
    await expect(page.locator('button', { hasText: 'Draft an investor update for this week.' }).first()).toBeVisible({ timeout: 5000 });
  });

  test('should display generated artifacts in Results panel', async ({ page }) => {
    // Navigate to the assistant route
    await page.goto('/assistant');

    // Switch to Artifacts tab
    const artifactsTab = page.locator('button', { hasText: 'Artifacts' });
    await expect(artifactsTab).toBeVisible();
    await artifactsTab.click();

    // Verify the tab changed and the artifact list is visible
    await expect(page.locator('.resultList, [class*="resultList"]')).toBeVisible();
  });
});
