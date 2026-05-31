import { expect } from '@playwright/test';
import { test } from './fixtures';

test.describe('Billing & Cost Dashboard CUJ', () => {
    test('User can view My Plan and Cost Dashboard and verify glassmorphic styling', async ({ adminUser, page }) => {
        // We use adminUser fixture which sets up network intercepts properly.

        // Go to dashboard
        await page.goto('/dashboard');

        // Wait for page to load
        await expect(page.locator('text=OneHuman Corp')).toBeVisible();

        // Navigate to My Plan
        await page.goto('/plan');

        // Verify Plan page loads
        await expect(page.locator('h1:has-text("My Plan")')).toBeVisible();
        await expect(page.locator('text=Your Current Usage')).toBeVisible();

        // Verify Glassmorphism
        const planSection = page.locator('section').first();
        await expect(planSection).toHaveCSS('backdrop-filter', 'blur(20px) saturate(200%)');

        // Navigate to Cost Dashboard
        await page.goto('/cost-dashboard');

        // Verify Cost Dashboard loads
        await expect(page.locator('h1:has-text("Business Advisory Dashboard")')).toBeVisible();
        await expect(page.locator('h2:has-text("Cost Transparency")')).toBeVisible();

        // Verify Glassmorphism on Cost Dashboard
        const costSection = page.locator('section').first();
        await expect(costSection).toHaveCSS('backdrop-filter', 'blur(20px) saturate(200%)');
    });
});
