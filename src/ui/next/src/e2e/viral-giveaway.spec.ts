import { test, expect } from '@playwright/test';

test.describe('Giveaway Generator Growth Loop', () => {
    test('generator page renders correctly, generates link, and public page works with footer', async ({ page }) => {
        // 1. Set some initial local storage state to act as a logged-in user
        await page.goto('/dashboard');
        await page.evaluate(() => {
            localStorage.setItem('tenant', 'e2e-bakery');
            localStorage.setItem('has_pro', 'true');
        });

        // 2. Go to the Giveaway Generator page
        await page.goto('/giveaway');

        // Check the page header
        await expect(page.locator('h1', { hasText: 'Viral Giveaway Generator' })).toBeVisible();

        // 3. Configure the giveaway
        const titleInput = page.getByPlaceholder('e.g. Win a free custom cake!');
        await titleInput.fill('Awesome E2E Cake Giveaway');

        const descriptionInput = page.getByPlaceholder('e.g. Enter your email for a chance to win...');
        await descriptionInput.fill('The best automated cakes in town giveaway.');

        // 4. Generate the link
        const generateButton = page.locator('button', { hasText: 'Generate Giveaway Link' });
        await generateButton.click();

        // 5. Verify the link is generated
        await expect(page.locator('h3', { hasText: 'Link Ready!' })).toBeVisible();
        const linkInput = page.locator('input[readonly]');
        await expect(linkInput).toBeVisible();
        const generatedLink = await linkInput.inputValue();
        expect(generatedLink).toContain('/giveaway/enter?tenant=e2e-bakery');

        // Wait a moment for the state
        await page.waitForTimeout(500);

        // 6. Navigate to the generated public page
        await page.goto(`/giveaway/enter?tenant=e2e-bakery&title=Awesome%20E2E%20Cake%20Giveaway&description=The%20best%20automated%20cakes%20in%20town%20giveaway.`);

        // Verify the public page loaded the saved data
        await expect(page.locator('h1', { hasText: 'Awesome E2E Cake Giveaway' })).toBeVisible();
        await expect(page.locator('p', { hasText: 'The best automated cakes in town giveaway.' })).toBeVisible();

        // Verify the viral footer exists on the public page
        const publicFooterLink = page.locator('a', { hasText: '⚡ Powered by OHC' });
        await expect(publicFooterLink).toBeVisible();
        await expect(publicFooterLink).toHaveAttribute('href', '/api/v1/growth/referrals/click?target=/onboarding&ref=e2e-bakery');
    });

    test('Dashboard contains link to Giveaway generator', async ({ page }) => {
        await page.goto('/dashboard');

        // Find the link to create a giveaway
        const giveawayButton = page.locator('a[href="/giveaway"]');
        await expect(giveawayButton).toBeVisible();
        await expect(giveawayButton).toContainText('Viral Giveaway Generator');
    });
});
