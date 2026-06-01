import { test, expect } from '@playwright/test';

test.describe('Trust Loop Growth Loop', () => {
    test('Wall of Love API renders properly and contains viral loop footer', async ({ page }) => {
        // Go directly to the widget route to test it isolated.
        await page.goto('http://localhost:3000/api/v1/growth/storefront/wall-of-love?tenant=my-test-store');

        // Assert the title and core elements render
        await expect(page.locator('h2:has-text("Loved by Customers")')).toBeVisible();
        await expect(page.locator('text=Absolutely amazing product!')).toBeVisible();

        // Check the Viral Growth Loop footer
        const viralLink = page.locator('a[href^="https://ohc.store/join?ref="]');
        await expect(viralLink).toBeVisible();
        await expect(viralLink).toContainText('Powered by OHC - Create your own Wall of Love');

        // Verify tenant is correctly parameterized in the viral link
        const href = await viralLink.getAttribute('href');
        expect(href).toBe('https://ohc.store/join?ref=my-test-store');
    });

    test('Dashboard embeds the Wall of Love widget and displays copy button', async ({ page }) => {
        await page.goto('http://localhost:3000/dashboard');

        // Find Wall of Love Generator button and click it to open the modal
        await page.click('button:has-text("Generate Widget")');

        // Verify the modal appears
        await expect(page.locator('h2:has-text("Your Wall of Love")')).toBeVisible();

        // Verify the copy code button is present
        await expect(page.locator('button:has-text("Copy Code")')).toBeVisible();

        // Verify the textarea has the iframe src pointing to the API route
        const textArea = page.locator('textarea');
        const textAreaValue = await textArea.inputValue();
        expect(textAreaValue).toContain('<iframe src="https://ohc.app/api/v1/growth/storefront/wall-of-love');
    });

    test('Dashboard automated review request generation works', async ({ page }) => {
         // Dashboard has an automated review request loop.
         await page.goto('http://localhost:3000/dashboard');

         // In a real browser context, window might not have localStorage set for the tenant yet,
         // but we can just test the UI elements.
         // Click on the specific row's Draft Request button in "Turn Customers into Advocates" section
         const askReviewBtn = page.locator('div:has-text("Order #8922 - Delivered")').locator('..').locator('button');
         await askReviewBtn.click();

         // Modal should pop up showing the drafted review
         await expect(page.locator('h2:has-text("AI Review Request")')).toBeVisible();

         // We should see the textarea for the generated email and the send button
         await expect(page.locator('textarea')).toBeVisible();
         await expect(page.locator('button:has-text("Send Email")')).toBeVisible();
    });
});
