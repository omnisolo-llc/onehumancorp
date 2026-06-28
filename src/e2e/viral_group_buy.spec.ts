import { test, expect } from './fixtures';

test.describe('Viral Group Buy Generator E2E', () => {
    test('should allow owner to customize group buy offer, generate link, and user to join', async ({ memberPage, context }) => {
        test.setTimeout(90000);

        // 1. Owner navigates to the group buy generator
        await memberPage.goto('/ui/group-buy-generator.html');
        await expect(memberPage.locator('h1', { hasText: 'Viral Group Buy Generator' })).toBeVisible({ timeout: 15000 });

        // 2. Owner fills out the offer details
        const productInput = memberPage.locator('#product-name');
        await productInput.fill('Playwright Masterclass');

        const regularPriceInput = memberPage.locator('#regular-price');
        await regularPriceInput.fill('100');

        const groupPriceInput = memberPage.locator('#group-price');
        await groupPriceInput.fill('50');

        const sizeInput = memberPage.locator('#group-size');
        await sizeInput.fill('2'); // We just need 1 more person to hit 2

        // 3. Click generate link
        const generateBtn = memberPage.locator('#generate-btn');
        await generateBtn.click();

        // Wait for result area to be visible
        const resultArea = memberPage.locator('#result-area');
        await expect(resultArea).toBeVisible();

        // 4. Copy the link
        const linkInput = memberPage.locator('#generated-url');
        const generatedUrl = await linkInput.inputValue();

        expect(generatedUrl).toContain('product=Playwright+Masterclass');
        expect(generatedUrl).toContain('reg=100');
        expect(generatedUrl).toContain('grp=50');
        expect(generatedUrl).toContain('size=2');

        // 5. Open consumer page (Join link) in a new context
        const publicPage = await context.newPage();
        await publicPage.goto(generatedUrl);

        // Verify consumer UI
        await expect(publicPage.locator('#display-product')).toHaveText('Playwright Masterclass');
        await expect(publicPage.locator('#display-reg-price')).toHaveText('$100');
        await expect(publicPage.locator('#display-group-price')).toHaveText('$50');
        await expect(publicPage.locator('#savings-badge')).toHaveText('SAVE 50%');

        // Progress should show 1 / 2 joined initially
        await expect(publicPage.locator('#spots-filled')).toHaveText('1');
        await expect(publicPage.locator('#spots-total')).toHaveText('2');

        // 6. Consumer enters email to join
        const emailInput = publicPage.locator('#email-input');
        await emailInput.fill('consumer@example.com');

        const joinBtn = publicPage.locator('#join-btn');
        await expect(joinBtn).toHaveText('Lock in $50 Price');
        await joinBtn.click();

        // 7. Verify the share section appears and deal unlocks (since size is 2 and we added 1)
        const shareSection = publicPage.locator('#share-section');
        await expect(shareSection).toBeVisible();
        await expect(shareSection.locator('h3')).toHaveText('Deal Unlocked! 🎉', { timeout: 5000 });

        // Verify footer viral loop is present
        const footerLink = publicPage.locator('#footer-branding');
        await expect(footerLink).toBeVisible();
        await expect(footerLink).toHaveAttribute('href', /setup.html\?source=groupbuy_footer/);

        await publicPage.close();
    });
});
