import { test, expect } from '@playwright/test';

test.describe('Finance Dashboard Micro-Financing CUJ', () => {
    test.beforeEach(async ({ page }) => {
        // Authenticate by setting token in localStorage
        await page.goto('/');
        await page.evaluate(() => {
            localStorage.setItem('token', 'test-token');
            localStorage.setItem('tenant', 'test-tenant');
        });
    });

    test('Should display predictive cashflow alert and allow 1-tap accept', async ({ page }) => {
        // Step 1: Navigate to the dashboard (simulating home page login context)
        await page.goto('/dashboard');

        // Step 2: Verify the Predictive Cashflow Alert is visible in the UI
        const alertCard = page.locator('text="Predictive Cashflow Alert"');
        await expect(alertCard).toBeVisible();

        // Step 3: Verify the text mentions the shortfall amount (mocked at $500)
        await expect(page.locator('p', { hasText: 'it looks like you\'ll need $500 for supplies next week' })).toBeVisible();

        // Step 4: Click the "Advanced Details" toggle and verify APR text
        // First we wait for the section to be fully ready
        await page.waitForTimeout(500);
        await page.locator('div', { hasText: 'Advanced Details' }).last().locator('button').click();
        await expect(page.locator('text=/15.2%/')).toBeVisible();

        // Step 5: Click the 1-Tap Accept button
        const acceptButton = page.locator('button', { hasText: '1-Tap Accept (500)' });
        await expect(acceptButton).toBeVisible();

        // Note: The UI mocks the acceptance API response and updates state.
        // We will intercept the API route to guarantee success response during the test.
        await page.route('/api/finance/microloan/accept', async route => {
            await route.fulfill({ status: 200, json: { success: true, message: "Loan accepted" } });
        });

        await acceptButton.click();

        // Step 6: Verify the UI transitions to the "Funds Disbursed" success state
        await expect(page.locator('text=/Funds Disbursed!/')).toBeVisible();
        await expect(page.locator('p', { hasText: '$500 has been added to your Treasury wallet' })).toBeVisible();
    });
});
