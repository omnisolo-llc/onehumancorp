import { test, expect } from '@playwright/test';

test.describe('Draft Toast Messages', () => {
    test('Saving a draft in onboarding shows a success toast', async ({ page }) => {
        // Go to onboarding wizard
        await page.goto('/onboarding');

        // Ensure "Let's Get Started" heading is visible
        await expect(page.getByRole('heading', { name: 'Let\'s Get Started' })).toBeVisible();

        // Wait for the inputs
        const businessNameInput = page.getByLabel('Business Name');
        await expect(businessNameInput).toBeVisible();

        // Fill out business name which should trigger draft auto-save
        await businessNameInput.fill('Test Business Name');

        // Ensure that the Toast message "Draft Saved!" appears on screen
        await expect(page.getByText('Draft Saved!')).toBeVisible({ timeout: 5000 });
    });
});
