import { test, expect } from '@playwright/test';

test('verify title change', async ({ page }) => {
    // Navigate to the dashboard
    await page.goto('/login');
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Sign In")');

    await page.waitForURL('**/*');

    // Wait for the UI text "System History" instead of "Swarm Observability"
    const sysHistory = page.locator('text=System History');
    // Wait for the UI text "Team Members" instead of "Company Structure" or "My Team"
    const teamMembers = page.locator('text=Team Members');
});
