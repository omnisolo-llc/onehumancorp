import { test, expect } from '@playwright/test';

test('verify title change', async ({ page }) => {
    // E2E test to verify Swarm Observability is removed
    await page.goto('/');

    // Login
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Sign In")');

    // Wait for Dashboard to load
    await page.waitForSelector('text=Dashboard');

    // Verify "Agent Actions Today" is visible
    await expect(page.locator('text=Agent Actions Today')).toBeVisible();

    // Verify "Swarm Observability" is NOT visible
    await expect(page.locator('text=Swarm Observability')).toHaveCount(0);

    // Verify "My Team" is visible
    await expect(page.locator('text=My Team')).toBeVisible();

    // Verify "Company Structure" is NOT visible
    await expect(page.locator('text=Company Structure')).toHaveCount(0);
});
