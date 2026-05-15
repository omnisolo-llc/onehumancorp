import { test, expect } from '@playwright/test';

test.describe('Echo UX Improvements', () => {
    test.beforeEach(async ({ page }) => {
        // Mock the initial state and load the app.
        await page.goto('/');

        // Ensure we are on the login screen, then sign in with UI clicks
        // The HTML actually defaults to signup or login screen but looks like we need to click "Have an account? Sign In" first
        // based on the HTML in lib.rs
        const loginScreen = page.locator('#login-screen');
        if (!(await loginScreen.isVisible())) {
            // It might be on signup screen, switch to login
            const switchToLogin = page.locator('button:has-text("Have an account? Sign In")');
            if (await switchToLogin.isVisible()) {
                await switchToLogin.click();
            }
        }

        // If login screen doesn't exist or isn't shown initially we handle signup
        const signupScreen = page.locator('#signup-screen');
        if (await signupScreen.isVisible()) {
             await page.getByPlaceholder('Email or Username').fill('test@test.com');
             await page.getByPlaceholder('Password').fill('password');
             await page.getByRole('button', { name: 'Sign Up' }).click();
        } else {
             // In case there is a login flow
             await page.evaluate(() => {
                (window as any).showScreen('dashboard-screen');
             });
        }

        // Let's just make sure we do the UI flow. Let me check the HTML for actual login button behavior.
        // Actually, the HTML provided shows a signup screen that calls `handleSignup(this)`.
    });

    test('mobile navigation is visible on mobile viewport', async ({ page }) => {
        await page.setViewportSize({ width: 375, height: 667 });

        // Wait for bottom nav to be visible
        const mobileNav = page.locator('#mobile-nav');
        await expect(mobileNav).toBeVisible();
    });

    test('mobile navigation elements have correct labels', async ({ page }) => {
        await page.setViewportSize({ width: 375, height: 667 });

        const expectedLabels = ['Add Product', 'Orders', 'Messages', 'Analytics', 'Share Store'];

        for (const label of expectedLabels) {
            const btn = page.locator('#mobile-nav').getByRole('button', { name: label });
            await expect(btn).toBeVisible();
        }
    });

    test('mobile navigation elements meet minimum touch target size (44x44)', async ({ page }) => {
        await page.setViewportSize({ width: 375, height: 667 });

        const btn = page.locator('#mobile-nav').getByRole('button', { name: 'Orders' });
        const box = await btn.boundingBox();

        expect(box).not.toBeNull();
        expect(box!.width).toBeGreaterThanOrEqual(44);
        expect(box!.height).toBeGreaterThanOrEqual(44);
    });

    test('navigating via bottom nav switches screens', async ({ page }) => {
        await page.setViewportSize({ width: 375, height: 667 });

        // Click on Analytics
        const analyticsBtn = page.locator('#mobile-nav').getByRole('button', { name: 'Analytics' });
        await analyticsBtn.click();

        // Our mock analytics button just switches to the agents-screen for now (based on showScreen('agents-screen') in the HTML)
        const agentsScreen = page.locator('#agents-screen');
        await expect(agentsScreen).toBeVisible();
    });

    test('dashboard cards use glassmorphism styling', async ({ page }) => {
        await page.setViewportSize({ width: 1440, height: 900 });

        const card = page.locator('#dashboard-screen .card').first();
        await expect(card).toBeVisible();

        // Check computed styles for backdrop-filter
        const backdropFilter = await card.evaluate((el) => {
            const style = window.getComputedStyle(el);
            return style.backdropFilter || style.webkitBackdropFilter;
        });

        expect(backdropFilter).toContain('blur');
    });
});
