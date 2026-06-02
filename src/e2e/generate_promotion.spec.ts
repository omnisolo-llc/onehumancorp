import { test, expect } from './fixtures';

test.describe('Generate Promotion Growth Loop', () => {
    test('User can generate a promotion from the dashboard', async ({ adminPage: page }) => {
        // We use the adminPage fixture setup to navigate to the dashboard
        // after login (the fixture logs us in and goes to /dashboard).
        // For simplicity we will log in and then navigate to the dashboard.
        await page.goto('/dashboard');

        // Wait for the dashboard to load
        await expect(page.locator('h1').filter({ hasText: 'Overview' })).toBeVisible({ timeout: 10000 });

        // Scroll down to the Growth & Promotions section if needed
        const generatePromoButton = page.getByRole('button', { name: 'Generate Promotion' });


        // Click the Generate Promotion button
        await generatePromoButton.click();

        // The modal should appear with "AI Promotion Generator"
        await expect(page.getByRole('heading', { name: 'AI Promotion Generator' })).toBeVisible();

        // Wait for generation to complete and the textarea to be populated
        // The generate button sets isGeneratingPromo = true and shows "Generating the perfect message..."
        // When it finishes, it shows the textarea with the generated message.
        const textarea = page.locator('textarea');
        await expect(textarea).toBeVisible();

        // The message should contain the fallback or generated text
        await expect(textarea).toHaveValue(/Shop now and get 10% off: https:\/\/ohc.store\/shop\//);

        // Copy the promo link
        const copyButton = page.getByRole('button', { name: 'Copy Promo Link' });
        await copyButton.click();

        // Ensure the button changes to Copied!
        await expect(page.getByRole('button', { name: 'Copied!' })).toBeVisible();
    });
});
