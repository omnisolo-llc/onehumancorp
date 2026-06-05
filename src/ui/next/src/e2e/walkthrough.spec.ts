import { test, expect } from '@playwright/test';

test.describe('Interactive Walkthrough', () => {
    test('starts walkthrough, navigates through steps, and completes', async ({ page }) => {
        // Load the dashboard with `test_walkthrough=true` query param to bypass E2E skip
        await page.goto('/dashboard?test_walkthrough=true');

        // Locate the Start Tour button
        const startTourButton = page.locator('button', { hasText: 'Start Tour' });
        await expect(startTourButton).toBeVisible();

        // Click it to start
        await startTourButton.click();

        // Verify the first step bubble appears
        // Step 1: "Business Analytics"
        const bubble = page.locator('div.fixed.z-\\[1000\\]');
        await expect(bubble).toBeVisible();
        await expect(bubble.locator('h3', { hasText: 'Business Analytics' })).toBeVisible();
        await expect(bubble.locator('span', { hasText: 'Step 1 of 2' })).toBeVisible();

        // Click next
        const nextButton = bubble.locator('button', { hasText: 'Next' });
        await nextButton.click();

        // Verify the second step bubble appears
        // Step 2: "Operations Map"
        await expect(bubble.locator('h3', { hasText: 'Operations Map' })).toBeVisible();
        await expect(bubble.locator('span', { hasText: 'Step 2 of 2' })).toBeVisible();

        // Click finish
        const finishButton = bubble.locator('button', { hasText: 'Finish' });
        await finishButton.click();

        // Verify the bubble is gone
        await expect(bubble).not.toBeVisible();
    });
});
