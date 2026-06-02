import { test, expect } from './fixtures';
import { memberPage } from './fixtures';

test.describe('Pricing & Billing Portal', () => {
    memberPage('should allow user to navigate to pricing and see plans', async ({ page }) => {
        await page.goto('/pricing');
        await expect(page.locator('text=Pricing Plans')).toBeVisible();
        await expect(page.locator('text=Starter')).toBeVisible();
    });

    memberPage('should allow user to navigate to my plan and cancel', async ({ page }) => {
        await page.goto('/plan');
        await expect(page.locator('text=My Plan')).toBeVisible();

        // Mock the window.confirm to return true for cancellation
        page.on('dialog', dialog => dialog.accept());

        const cancelPromise = page.waitForResponse(response => response.url().includes('/api/billing/cancel'));
        await page.locator('text=Cancel Subscription').click();
        const response = await cancelPromise;
        expect(response.status()).toBe(200);
    });

    memberPage('should allow user to upgrade to Starter', async ({ page }) => {
        await page.goto('/pricing');

        // The backend automatically provides a fallback mock URL when there is no Stripe client.
        // We will just click the button and expect navigation to checkout.stripe.com/pay/mock_Starter

        const [request] = await Promise.all([
            page.waitForRequest(req => req.url().includes('checkout.stripe.com/pay/mock_Starter') || req.url().includes('checkout.stripe.com/c/pay/')),
            page.locator('button:has-text("Upgrade to Starter via Stripe")').click()
        ]);
        expect(request.url()).toContain('checkout.stripe.com');
    });
});
