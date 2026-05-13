import { test, expect } from '@playwright/test';

test('Order placement triggers Operations and Customer Success AI agents chained workflow', async ({ page }) => {
    // Navigate to the home page
    await page.goto('/');

    // Login
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Login")');

    // Wait for the Dashboard
    await expect(page.locator('text="Your business, live in minutes."')).toBeVisible();

    // Trigger the flow
    await page.click('button:has-text("Test Order")');

    // Wait for Operations agent activity (AutoExecute action)
    await expect(page.locator('text="Operations processed OrderReceived"')).toBeVisible({ timeout: 5000 });

    // Wait for Customer Success agent draft activity (DraftForReview)
    await expect(page.locator('text="Send personalized thank you & shipping ETA"')).toBeVisible({ timeout: 5000 });

    // Verify that the Draft-for-Review approval card surfaces
    await expect(page.locator('text="Tasks for You to Approve"')).toBeVisible({ timeout: 5000 });
    await expect(page.locator('text="Send personalized thank you & shipping ETA"')).toBeVisible({ timeout: 5000 });
    await expect(page.locator('text="Approve & Send"')).toBeVisible({ timeout: 5000 });
});
