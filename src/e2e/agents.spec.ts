import { test, expect } from '@playwright/test';

test.describe('AI Agent Configuration E2E', () => {
  // Use mobile viewport for mobile-first requirement
  test.use({ viewport: { width: 375, height: 667 } });

  test('Configure agent and tune prompt', async ({ page }) => {
    await page.goto('/');

    // Bypass login by setting local storage directly, or just login
    await page.fill('#login-email', 'test@example.com');
    await page.click('button:has-text("Sign In")');
    await expect(page.locator('#dashboard-screen')).toBeVisible();

    // Go to agents screen
    await page.click('a:has-text("Agents")');
    await expect(page.locator('#agents-screen')).toBeVisible();

    // Select an agent
    await page.click('text=Customer Support');
    await expect(page.locator('#agent-config-screen')).toBeVisible();

    // Click tune this agent
    await page.click('button:has-text("Tune this agent")');
    await expect(page.locator('#prompt-tuning-screen')).toBeVisible();

    // Save tuning
    await page.click('button:has-text("Save Tuning")');
    await expect(page.locator('#agent-config-screen')).toBeVisible();

    // Activate agent
    await page.click('button:has-text("Activate")');
    await expect(page.locator('#agents-screen')).toBeVisible();
  });
});
