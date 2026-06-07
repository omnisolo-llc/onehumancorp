import { test, expect } from '@playwright/test';


test.describe('Offering Creation Flow', () => {
    test.beforeEach(async ({ page }) => {
        // Mock authentication to bypass actual login requirement
        await page.addInitScript(() => {
            localStorage.setItem('tenant', 'e2e-store');
            localStorage.setItem('user_id', 'e2e-user');
            localStorage.setItem('has_onboarded', 'true');
        });
    });

    test('Mobile-first unified offering creation via intent', async ({ page }) => {
        // Ensure standard mobile viewport size
        await page.setViewportSize({ width: 375, height: 812 });

        // 1. Start from Dashboard
        await page.goto('/dashboard');
        await expect(page.locator('h2', { hasText: 'Welcome back' })).toBeVisible();

        // 2. Click the new mobile FAB (Floating Action Button)
        const fab = page.locator('.fixed.bottom-24.right-6 a[href="/offerings/new"]');
        await expect(fab).toBeVisible();
        await fab.click();

        // 3. Verify we are on the Add Offering page
        await expect(page).toHaveURL(/\/offerings\/new/);
        await expect(page.locator('h1', { hasText: 'Add Offering' })).toBeVisible();

        // 4. Enter the intent and generate the offering draft
        const intentTextarea = page.locator('textarea[placeholder*="e.g. Guitar lessons"]');
        await intentTextarea.fill('Guitar lessons for beginners, 1 hour');

        // Click the generate/up arrow button
        await page.locator('button:has(span:has-text("↑"))').click();

        // 5. Verify the loading state briefly appears (optional due to speed, but we can check AI Drafted)
        // Wait for the AI Drafted form to appear
        const draftedBadge = page.locator('div:has-text("AI Drafted")').first();
        await expect(draftedBadge).toBeVisible({ timeout: 10000 });

        // 6. Verify the form fields were pre-filled correctly based on the mock backend
        await expect(page.locator('input[value="Guitar lessons for beginners, 1 "]')).toBeVisible();
        await expect(page.locator('textarea', { hasText: 'A fantastic service offering generated from your request' })).toBeVisible();
        await expect(page.locator('input[value="50"]')).toBeVisible();

        // 7. Edit the price
        const priceInput = page.locator('input[value="50"]');
        await priceInput.fill('45');

        // 8. Publish the offering
        const publishButton = page.locator('button', { hasText: 'Publish to Storefront' });
        await publishButton.click();

        // 9. Verify success state
        await expect(page.locator('h1', { hasText: 'Live and Ready!' })).toBeVisible();
        await expect(page.locator('a', { hasText: 'Return to Dashboard' })).toBeVisible();

        // 10. Return to dashboard
        await page.locator('a', { hasText: 'Return to Dashboard' }).click();
        await expect(page).toHaveURL(/\/dashboard/);
    });
});
