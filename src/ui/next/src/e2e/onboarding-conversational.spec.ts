import { test, expect } from '@playwright/test';

test.describe('AutoDream Conversational Onboarding', () => {
    test('User can onboard via conversational interface', async ({ page }) => {
        // Mock the initial state fetch
        await page.route('**/api/onboarding/state', async route => {
            if (route.request().method() === 'POST') {
                await route.fulfill({
                    status: 200,
                    contentType: 'application/json',
                    body: JSON.stringify({})
                });
                return;
            }
            await route.fulfill({
                status: 200,
                contentType: 'application/json',
                body: JSON.stringify({
                    wizardState: {
                        step: 1,
                        chatStep: 1,
                    }
                })
            });
        });

        // Mock the intake API
        await page.route('**/api/onboarding/intake', async route => {
            await route.fulfill({
                status: 200,
                contentType: 'application/json',
                body: JSON.stringify({
                    business_name: 'Maya Bakery',
                    business_type: 'Bakery',
                    categories: ['food', 'physical'],
                    initial_products: [
                        { name: 'Vegan Cake', price: '45.00' }
                    ]
                })
            });
        });

        // Go to onboarding page
        await page.goto('http://localhost:3000/onboarding');

        // Initial agent message should be visible
        await expect(page.locator('text=Hi! I am AutoDream').first()).toBeVisible({ timeout: 15000 });

        // Type business description
        const input = page.locator('input[placeholder="Describe your business..."]');
        await expect(input).toBeVisible();
        await input.fill('I bake vegan cakes in Austin');

        // Submit the message
        const submitButton = page.locator('button[type="submit"]');
        await submitButton.click();

        // Verify user message appears in chat
        await expect(page.locator('text=I bake vegan cakes in Austin')).toBeVisible();

        // Verify simulated streaming messages
        await expect(page.locator('text=Analyzing business model...')).toBeVisible();
        // The last message is "Setting up standard checkout profile..."
        await expect(page.locator('text=Setting up standard checkout profile...')).toBeVisible();

        // Verify transition to Review Details step
        // Depending on backend response time, this may take a moment.
        // We look for the "Review Details" header which is rendered in step 2.
        await expect(page.locator('h2:has-text("Review Details")')).toBeVisible({ timeout: 10000 });

        // Check if the AI populated the fields
        const businessNameInput = page.locator('label:has-text("Business Name") + input');
        await expect(businessNameInput).toBeVisible();
        await expect(businessNameInput).toHaveValue('Maya Bakery');
    });
});
