import { test, expect } from '@playwright/test';

test.describe('Hybrid Landing Page E2E', () => {
  test('User navigates to Hybrid Landing page and starts cloud trial', async ({ page }) => {
    await page.goto('/hybrid-landing');

    // Verify main header
    await expect(page.locator('h1')).toHaveText(/Your Business.\s*Your AI. Your Rules./);

    // Verify cards content
    await expect(page.locator('text=Local Sovereignty')).toBeVisible();
    await expect(page.locator('text=Zero Data Leakage:')).toBeVisible();

    await expect(page.locator('text=Cloud Convenience')).toBeVisible();
    await expect(page.locator('text=Team Collaboration:')).toBeVisible();

    // Verify buttons
    await expect(page.locator('button:has-text("Download Desktop")')).toBeVisible();
    const startWebTrial = page.locator('a:has-text("Start Web Trial")');
    await expect(startWebTrial).toBeVisible();

    // Navigate to dashboard
    await startWebTrial.click();
    await expect(page).toHaveURL(/.*\/dashboard|.*\/onboarding/);
  });
});
