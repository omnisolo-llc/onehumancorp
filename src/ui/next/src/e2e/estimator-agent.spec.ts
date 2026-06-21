import { test, expect } from '@playwright/test';

test.describe('The Estimator Agent - Mobile Feed to Customer Deposit', () => {
    test('owner approves quote, customer receives and pays deposit', async ({ page, request }) => {

        // Ensure there is some draft proposal in the DB so it renders
        // Mock the backend API directly via page route
        await page.route('/api/agent-feed', async route => {
            const json = {
                items: [
                    {
                        id: 'test-proposal-feed-item',
                        lifecycle_state: 'PENDING_APPROVAL',
                        proposed_action: {
                            action_type: 'Draft Proposal',
                            service_name: 'Custom Repair',
                            total_amount_cents: 20000,
                            required_deposit_cents: 10000
                        },
                        message: 'Carlos, new repair request from John. I drafted a quote for $200 with a $100 deposit based on your standard rates.'
                    }
                ]
            };
            await route.fulfill({ json });
        });

        // --- 1. Owner Mobile Feed Flow (375px) ---
        await page.setViewportSize({ width: 375, height: 812 }); // iPhone X dimensions
        await page.goto('/feed?tenant=carlos-handyman');

        // Wait for the feed to load
        await page.waitForTimeout(500);

        // Wait for a quote draft action card
        const feedCard = page.locator('.ohc-feed-card', { hasText: 'Carlos, new repair request from John' }).first();
        await expect(feedCard).toBeVisible();

        // Tap the card to open the translucent modal
        await feedCard.click();

        // Verify the Apple-style modal contents
        const modal = page.locator('.ohc-modal');
        await expect(modal).toBeVisible();
        await expect(modal.locator('text=Deposit Required')).toBeVisible();

        // Tap "Approve & Send"
        const approveBtn = modal.locator('button', { hasText: 'Approve & Send' });
        await approveBtn.click();

        // Verify the card changes state or disappears
        await expect(modal).toBeHidden();


        // --- 2. Customer Web View Flow ---
        // Simulate customer opening the generated proposal link
        await page.goto('/proposals/customer-view?id=test-proposal-uuid');

        // Verify responsive layout
        await expect(page.locator('h1', { hasText: 'Your Quote' })).toBeVisible();
        await expect(page.locator('text=Deposit Due')).toBeVisible();

        // Customer clicks "Pay Deposit"
        const payBtn = page.locator('button', { hasText: 'Pay Deposit' });
        await expect(payBtn).toBeVisible();

        page.on('dialog', async dialog => {
            expect(dialog.message()).toContain('Redirecting to Stripe for deposit payment...');
            await dialog.accept();
        });

        await payBtn.click();
    });
});
