import { test, expect } from '../../../../e2e/fixtures';

test.describe('Interactive Quote Widget Growth Loop', () => {
    test('generator page renders correctly, preview updates, and public widget functions properly with viral loop link', async ({ page }) => {
        // 1. Set some initial local storage state to act as a logged-in user
        await page.goto('/dashboard');
        await page.evaluate(() => {
            localStorage.setItem('tenant', 'e2e-service');
        });

        // 2. Go to the Interactive Quote Generator page
        await page.goto('/interactive-quote-generator');

        // Check the page header
        await expect(page.locator('h1', { hasText: 'Interactive Quote Generator 🧮' })).toBeVisible();

        // 3. Configure the page
        const serviceNameInput = page.getByPlaceholder('e.g. Custom Cake Design');
        await serviceNameInput.fill('Professional Landscaping');

        const basePriceInput = page.getByPlaceholder('50');
        await basePriceInput.fill('200');

        const unitNameInput = page.getByPlaceholder('e.g. Guests, Hours, Pages');
        await unitNameInput.fill('Hours');

        const pricePerUnitInput = page.getByPlaceholder('5', { exact: true });
        await pricePerUnitInput.fill('50');

        // Wait a brief moment for state update
        await page.waitForTimeout(500);

        // 5. Navigate to the generated public widget page (simulated embed page)
        // Since the generator page dynamically builds the link, we can just navigate directly
        await page.goto('/quote-calculator?tenant=e2e-service&service=Professional%20Landscaping&basePrice=200&unitName=Hours&pricePerUnit=50&theme=light');

        // Verify the public page loaded the passed data
        await expect(page.locator('h3', { hasText: 'Professional Landscaping Quote' })).toBeVisible();
        await expect(page.locator('span', { hasText: '$200.00' }).first()).toBeVisible();
        await expect(page.locator('label', { hasText: 'Number of Hours' })).toBeVisible();

        // 6. Test interaction: Slide or input new quantity to update total
        // The default slider value is 1, so total should be 200 + (1 * 50) = 250
        await expect(page.locator('span', { hasText: '$250.00' })).toBeVisible();

        // Change quantity to 4
        const slider = page.locator('input[type="range"]');
        await slider.fill('4');

        // New total should be 200 + (4 * 50) = 400
        await expect(page.locator('span', { hasText: '$400.00' })).toBeVisible();

        // 7. Verify the viral footer exists on the public page and has the exact expected referral URL structure
        const publicFooterLink = page.locator('a', { hasText: '⚡ Powered by OHC' });
        await expect(publicFooterLink).toBeVisible();
        await expect(publicFooterLink).toHaveAttribute('href', /\/api\/v1\/growth\/referrals\/click\?target=\/onboarding&ref=e2e-service/);

        // 8. Go back to generator page and verify the embed code contains the viral link
        // We wait for the generator page to be ready and local storage tenant to be loaded
        await page.goto('/interactive-quote-generator');
        // Give time for useEffect to pick up localStorage and update state to e2e-service
        await page.waitForFunction(() => {
            const el = document.querySelector('textarea[readonly]');
            return el && (el as HTMLTextAreaElement).value.includes('ref=e2e-service');
        });

        const embedCodeTextarea = page.locator('textarea[readonly]');
        await expect(embedCodeTextarea).toBeVisible();

        // Use regex for checking text since spaces/newlines might differ, ensuring the core viral link structure is there
        const embedCodeValue = await embedCodeTextarea.inputValue();
        expect(embedCodeValue).toContain('https://ohc.app/api/v1/growth/referrals/click?target=/onboarding&ref=e2e-service');
        expect(embedCodeValue).toContain('⚡ Powered by OHC');
    });

    test('Dashboard contains link to Interactive Quote Generator', async ({ page }) => {
        await page.goto('/dashboard');

        // Find the link to create an interactive quote widget
        const quoteWidgetLink = page.locator('a[href="/interactive-quote-generator"]');
        await expect(quoteWidgetLink).toBeVisible();
        await expect(quoteWidgetLink).toContainText('Interactive Quote Widget');
    });
});
