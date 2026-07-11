import { expect, test } from '@playwright/test';

test.describe('The Estimator Agent - Mobile Feed to Customer Deposit', () => {
    test('owner approves quote, customer receives and pays deposit', async ({ page, request }) => {
        test.setTimeout(180000);

        await page.goto('/login');
        await page.getByPlaceholder('Email or Username').fill('test@example.com');
        await page.getByPlaceholder('Password').fill('password123');
        await page.getByRole('button', { name: 'Log In' }).click();
        await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

        const tenantId = await page.evaluate(() => localStorage.getItem('tenant_id') || 'default');

        // Create a real quote in the backend via API
        const quoteRes = await request.post(`/api/v1/quotes`, {
            headers: {
                'x-tenant-id': tenantId,
            },
            data: {
                tenant_id: tenantId,
                customer_id: '00000000-0000-0000-0000-000000000001',
                total_amount_cents: 15000,
                required_deposit_cents: 5000,
                line_items: [
                    { description: "Custom Service", unit_price_cents: 15000, quantity: 1, is_optional: false }
                ]
            }
        });

        expect(quoteRes.ok()).toBeTruthy();
        const quoteData = await quoteRes.json();
        const quoteId = quoteData.id;

        // Add an agent feed item to trigger the UI
        await request.post(`/api/triage/create?tenant_id=${encodeURIComponent(tenantId)}`, {
            data: {
                customer_id: '00000000-0000-0000-0000-000000000001',
                source: 'email',
                priority: 'high',
                context: 'Carlos, new repair request from John. I drafted a quote for $150 with a $50 deposit based on your standard rates.',
                action_type: 'Draft Quote',
                action_payload: JSON.stringify({
                    quote_id: quoteId,
                    total_amount_cents: 15000,
                    required_deposit_cents: 5000
                })
            }
        });

        // --- 1. Owner Mobile Feed Flow (375px) ---
        await page.setViewportSize({ width: 375, height: 812 }); // iPhone X dimensions
        await page.goto('/feed');

        // Wait for the feed to load
        await page.waitForTimeout(1000);

        // Find the feed item
        const reviewBtn = page.getByTestId('feed-approve-btn').first();
        await expect(reviewBtn).toBeVisible({ timeout: 15000 });

        // Go to review
        await reviewBtn.click();

        // --- 2. Quote Review Flow ---
        await expect(page.locator('text=Review Estimate')).toBeVisible({ timeout: 15000 });

        const approveQuoteBtn = page.locator('button', { hasText: 'Approve & Send to Customer' });
        await expect(approveQuoteBtn).toBeVisible();
        await approveQuoteBtn.click();

        await expect(page.locator('text=Stripe Payment Link')).toBeVisible({ timeout: 15000 });

        // --- 3. Customer Web View Flow ---
        // Go to the customer view
        await page.goto(`/ui/quote.html?id=${quoteId}`);

        // Verify responsive layout
        await expect(page.locator('h1', { hasText: 'Quote Details' })).toBeVisible({ timeout: 15000 });
        await expect(page.locator('text=Deposit')).toBeVisible();

        // Customer clicks "Pay Deposit"
        const payBtn = page.locator('button', { hasText: 'Pay Deposit with Pay' });
        await expect(payBtn).toBeVisible();

        page.on('dialog', async dialog => {
            expect(dialog.message()).toContain('Redirecting to Stripe for deposit payment...');
            await dialog.accept();
        });

        await payBtn.click();
    });
});
