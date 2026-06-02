import { test, expect } from './fixtures';

test.describe('Pricing & Billing Portal', () => {
    test('should allow user to navigate to pricing and see plans', async ({ memberPage }) => {
        await memberPage.goto('/pricing');
        await expect(memberPage.locator('text=Pricing Plans')).toBeVisible();
        await expect(memberPage.locator('text=Starter')).toBeVisible();
    });

    test('should allow user to navigate to my plan and cancel', async ({ memberPage }) => {
        await memberPage.goto('/plan');
        await expect(memberPage.locator('text=My Plan')).toBeVisible();

        // Mock the window.confirm to return true for cancellation
        memberPage.on('dialog', dialog => dialog.accept());

        const cancelPromise = memberPage.waitForResponse(response => response.url().includes('/api/billing/cancel'));
        await memberPage.locator('text=Cancel Subscription').click();
        const response = await cancelPromise;
        expect(response.status()).toBe(200);
    });

    test('should allow user to upgrade to Starter', async ({ memberPage }) => {
        await memberPage.goto('/pricing');

        // The backend automatically provides a fallback mock URL when there is no Stripe client.
        // We will just click the button and expect navigation to checkout.stripe.com/pay/mock_Starter

        const [request] = await Promise.all([
            memberPage.waitForRequest(req => req.url().includes('checkout.stripe.com/pay/mock_Starter') || req.url().includes('checkout.stripe.com/c/pay/')),
            memberPage.locator('button:has-text("Upgrade to Starter via Stripe")').click()
        ]);
        expect(request.url()).toContain('checkout.stripe.com');
    });
});
