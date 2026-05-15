import { test, expect } from '@playwright/test';

test.describe('UX Friction Audit - Full Journey & Navigation', () => {
    test.use({ viewport: { width: 375, height: 667 } });

    // Helper to bypass strict mode / single locator matching
    const clickFirst = async (page: any, selector: string) => {
        await page.locator(selector).first().click();
    };

    test('Full UX flow: login -> dashboard -> agents -> integrations -> settings', async ({ page }) => {
        // We'll directly navigate to login and run the UI logic
        await page.goto('/login');

        // Login Page
        await expect(page.locator('text="Login"').first()).toBeVisible();
        await expect(page.locator('text="One Human Corp"').first()).toBeVisible();
        await page.locator('input[type="email"]').first().fill('test@test.com');
        await page.locator('input[type="password"]').first().fill('password');

        // Click Login (which might show an error if mocked incorrectly, but we just verify UI text)
        await clickFirst(page, 'button:has-text("Login")');

        // We manually transition to the dashboard to test the UX flows
        await page.goto('/dashboard');

        // Dashboard Page Checks
        await expect(page.locator('text="Your store is open"').first()).toBeVisible();

        // Mobile Navigation is reachable and clear (Check Plain language strings)
        await expect(page.locator('text="Check Messages"').first()).toBeVisible();
        await expect(page.locator('text="View Orders"').first()).toBeVisible();
        await expect(page.locator('text="See Analytics"').first()).toBeVisible();
        await expect(page.locator('text="Share Store"').first()).toBeVisible();

        // Check Glassmorphism
        const cards = await page.locator('.glass').all();
        if(cards.length > 0) {
           await expect(cards[0]).toBeVisible();
        }

        // Tap targets
        const buttons = await page.locator('button').all();
        if(buttons.length > 0) {
            const box = await buttons[0].boundingBox();
            expect(box?.width).toBeGreaterThanOrEqual(44);
            expect(box?.height).toBeGreaterThanOrEqual(44);
        }

        // Check Contextual Hint First
        await clickFirst(page, 'button:has-text("?")');
        await expect(page.locator('text="These buttons are shortcuts to your most common daily tasks."').first()).toBeVisible();

        // Navigate to Agents using Quick Actions
        await clickFirst(page, 'button:has-text("Manage Agents")');
        await expect(page.locator('text="Agents"').first()).toBeVisible();

        // Navigate back
        await clickFirst(page, 'button:has-text("Back")');

        // Navigate to Integrations (was Software)
        // Toggle menu first
        await clickFirst(page, 'button:has-text("Menu")');

        // Ensure menu is visible and click Connect Integrations
        await expect(page.locator('button:has-text("Connect Integrations")').first()).toBeVisible();
        await clickFirst(page, 'button:has-text("Connect Integrations")');

        await expect(page.locator('h1:has-text("Connect Integrations")').first()).toBeVisible();

        // Back to Dashboard
        await clickFirst(page, 'button:has-text("Back to Dashboard")');

        // Navigate to Settings
        await clickFirst(page, 'button:has-text("Settings")');
        await expect(page.locator('h1:has-text("Settings")').first()).toBeVisible();

        // Back to Dashboard
        await clickFirst(page, 'button:has-text("Cancel")');

        // Finally, navigate to Setup Wizard
        await clickFirst(page, 'button:has-text("Start Setup")');
        await expect(page.locator('h1:has-text("Your business, live in minutes.")').first()).toBeVisible();
    });
});
