import { test, expect } from '@playwright/test';

test.describe('Draft Toast Messages', () => {
    test('Saving a draft in website builder shows a success toast', async ({ page }) => {
        // Go to website builder
        await page.goto('/website-builder');

        // Wait for the form to appear
        await expect(page.getByRole('heading', { name: '10-Minute Setup Wizard' })).toBeVisible();
        await page.getByRole('button', { name: 'Instant Build' }).click();

        // Type some text to trigger a draft save
        const input = page.getByPlaceholder('e.g. Coffee shop, Bakery, Consulting');
        await input.fill('Bakery');

        // Ensure that the Toast message "Draft Saved!" appears on screen
        await expect(page.getByText('Draft Saved!')).toBeVisible({ timeout: 5000 });
    });
});
