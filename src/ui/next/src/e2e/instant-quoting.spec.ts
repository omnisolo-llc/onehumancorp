import { test, expect } from '@playwright/test';

test.describe('Instant Quoting E2E', () => {
    test('CUJ: customer visits widget and toggles dynamic pricing options without network lag', async ({ page }) => {
        // 1. Visit the widget as a customer requesting a quote
        await page.goto('/embed/widget?type=quote&tenant_id=demo');

        // 2. Wait for the page and sticky bar to load and display the base price
        await expect(page.getByText('Request a Quote')).toBeVisible();
        await expect(page.getByText('$50.00')).toBeVisible(); // Base price

        // 3. Interact with options and assert instant calculation
        // Select Medium Cake (+$25)
        await page.getByText('Medium (8")').click();
        await expect(page.getByText('$75.00')).toBeVisible(); // 50 + 25

        // Select Vegan (+15% of current price)
        await page.getByText('Vegan').click();
        await expect(page.getByText('$86.25')).toBeVisible(); // 75 * 1.15 = 86.25

        // Change delivery to Rush Delivery (+$40)
        await page.getByText('Rush Delivery (Today)').click();

        // Total = (50 + 25 + 40) * 1.15 = 115 * 1.15 = 132.25
        await expect(page.getByText('$132.25')).toBeVisible();

        // 4. Submit the quote
        // Fill out required generic details
        await page.fill('input[name="name"]', 'Carlos Tester');
        await page.fill('input[name="email"]', 'carlos@test.com');
        await page.fill('textarea[name="details"]', 'I need this for a party!');

        // Note: The E2E test will try to hit the backend `/api/v1/work-intake/submit` which will then try to hit the `/api/agents/webhook` in nextjs route.
        // We just need to make sure the submission goes through and we see the success screen.
        await page.getByRole('button', { name: 'Get My Quote' }).click();

        // 5. Verify Success
        await expect(page.getByText('Success!')).toBeVisible({ timeout: 10000 });
        await expect(page.getByText('Your request has been sent')).toBeVisible();
    });
});
