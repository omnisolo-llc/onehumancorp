import { test, expect } from './fixtures';

test.describe('Customer Loyalty Program Configurator E2E', () => {

    test('Dashboard contains link to Loyalty program', async ({ page }) => {
        await page.goto('/dashboard');
        const loyaltyLink = page.locator('a[href="/loyalty-program"]');
        await expect(loyaltyLink).toBeVisible();
        await expect(loyaltyLink).toContainText('Loyalty');
    });

    test('Can configure a Punch Card program', async ({ page }) => {
        await page.goto('/loyalty-program');

        // Wait for page to fully load the settings from API
        await page.waitForTimeout(2000);

        // Verify title
        await expect(page.locator('h1', { hasText: 'Customer Loyalty' })).toBeVisible();

        // Enable program
        const enableCheckbox = page.locator('input[type="checkbox"]');
        await enableCheckbox.click({ force: true });

        // Select Punch Card (default, but let's click it to be sure)
        await page.getByRole('button', { name: 'Punch Card' }).click();

        // Set threshold
        const thresholdInput = page.locator('input[type="number"]').first();
        await thresholdInput.fill('8');

        // Set description
        const descInput = page.getByPlaceholder('e.g., 50% off your next item');
        await descInput.fill('Free Croissant');

        // Save
        await page.getByRole('button', { name: 'Save Program Settings' }).click();

        // Verify Save Message
        await expect(page.getByRole('button', { name: 'Saved successfully!' })).toBeVisible();
    });

    test('Live preview updates for Punch Card correctly', async ({ page }) => {
        await page.goto('/loyalty-program');

        await page.waitForTimeout(2000);

        // Enable
        await page.locator('input[type="checkbox"]').click({ force: true });

        // Change values
        await page.locator('input[type="number"]').first().fill('6');
        await page.getByPlaceholder('e.g., 50% off your next item').fill('Free Muffin');

        // Check preview
        await expect(page.locator('p', { hasText: 'Buy 6, get a reward!' })).toBeVisible();
        await expect(page.locator('h3', { hasText: 'Free Muffin' })).toBeVisible();
    });

    test('Can configure a Points System program', async ({ page }) => {
        await page.goto('/loyalty-program');

        await page.waitForTimeout(2000);

        // Enable program
        await page.locator('input[type="checkbox"]').click({ force: true });

        // Select Points System
        await page.getByRole('button', { name: 'Points System' }).click();

        // Set threshold
        const inputs = page.locator('input[type="number"]');
        await inputs.nth(0).fill('500'); // Threshold

        // Set points per dollar
        await inputs.nth(1).fill('2');

        // Set description
        await page.getByPlaceholder('e.g., 50% off your next item').fill('$10 Store Credit');

        // Save
        await page.getByRole('button', { name: 'Save Program Settings' }).click();
        await expect(page.getByRole('button', { name: 'Saved successfully!' })).toBeVisible();

        // Check preview
        await expect(page.locator('h3', { hasText: '350' })).toBeVisible(); // Mocked points balance
        await expect(page.locator('p', { hasText: '150 more points to unlock:' })).toBeVisible();
        await expect(page.locator('p', { hasText: '$10 Store Credit' }).last()).toBeVisible();
    });

    test('Share Program button is visible when enabled', async ({ page }) => {
        await page.goto('/loyalty-program');

        await page.waitForTimeout(2000);

        // Should not be visible initially if disabled
        const shareBtn = page.getByRole('button', { name: 'Share Program to Customers' });
        await expect(shareBtn).toBeHidden();

        // Enable
        await page.locator('input[type="checkbox"]').click({ force: true });

        // Should be visible now
        await expect(shareBtn).toBeVisible();
    });
});
