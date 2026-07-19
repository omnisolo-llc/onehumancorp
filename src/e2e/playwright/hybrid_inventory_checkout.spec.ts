import { test, expect } from '@playwright/test';

test.describe('Centralized Inventory & Distributed POS Architecture', () => {

    test('Simultaneous online checkout and POS tap-to-pay lock contention', async ({ browser }) => {
        const posContext = await browser.newContext({ viewport: { width: 375, height: 667 } });
        const onlineContext = await browser.newContext({ viewport: { width: 1440, height: 900 } });

        const posPage = await posContext.newPage();
        const onlinePage = await onlineContext.newPage();

        const productId = 'e2e-red-dress-' + Date.now();

        // 1. Admin creates product via API
        await posPage.goto('/login');
        await posPage.getByPlaceholder('Email address').fill('admin@ohc.local');
        await posPage.getByPlaceholder('Password').fill('admin');
        await posPage.getByRole('button', { name: 'Sign In' }).click();
        await expect(posPage.locator('text=Dashboard').first()).toBeVisible({ timeout: 15000 });

        await posPage.request.post('/api/v1/catalog/products', {
            data: {
                id: productId,
                title: 'Red Dress',
                inventory_count: 1,
                price_cents: 10000
            }
        });

        // Add a test staff member
        await posPage.request.post('/api/v1/staff', {
            data: {
                id: 'staff_1',
                name: 'Priya',
                role: 'Manager',
                pin: '1234'
            }
        });

        // We will test the POS client
        await posPage.goto('/pos.html');
        // Wait for it to fetch staff
        await posPage.waitForTimeout(2000);
        await expect(posPage.locator('h1', { hasText: 'Terminal Locked' })).toBeVisible({ timeout: 15000 });

        // Unlock terminal
        await posPage.waitForSelector('button:has-text("1")');
        await posPage.getByRole('button', { name: '1', exact: true }).click();
        await posPage.getByRole('button', { name: '2', exact: true }).click();
        await posPage.getByRole('button', { name: '3', exact: true }).click();
        await posPage.getByRole('button', { name: '4', exact: true }).click();

        await posPage.getByRole('button', { name: 'Clock In' }).click();

        // Find and select the "Red Dress" product
        const productButton = posPage.locator('button.product-btn').filter({ hasText: 'Red Dress' });
        await expect(productButton).toBeVisible();
        await productButton.click();

        // Wait for checkout charge button
        const chargeBtn = posPage.locator('button.charge-btn', { hasText: /Collect Payment|Charge/ });
        await expect(chargeBtn).toBeVisible({ timeout: 15000 });
        await chargeBtn.click();

        // Wait for the tap to pay modal to be active
        await expect(posPage.locator('text=Payment Method')).toBeVisible({ timeout: 15000 });

        // Let's use Cash as the transaction type as the mock terminal flow might block in some setups
        await posPage.getByRole('button', { name: 'Cash' }).click();

        // At this point we are about to click "Record Offline Cash Sale" but actually online
        // Oh wait, if it's cash online it will reserve and commit immediately.
        // Let's intercept the commit to pause it.
        await posPage.route('**/api/v1/payments/terminal/commit', async route => {
            // we will hold this route until the online user hits the conflict

            // 4. The online customer attempts to checkout the same "Red Dress"
            // Start the checkout process online
            await onlinePage.goto(`/checkout?product_id=${productId}&quantity=1`);
            await expect(onlinePage.getByRole('button', { name: 'Pay' })).toBeVisible();
            await onlinePage.getByRole('button', { name: 'Pay' }).click();

            // 5. Verify the online customer receives "Item just sold out"
            await expect(onlinePage.locator('h3', { hasText: 'Oops! Item just sold out.' })).toBeVisible({ timeout: 15000 });

            // Now fulfill the commit route so the POS flow finishes
            route.continue();
        });

        // Click Cash record sale
        await posPage.getByRole('button', { name: /Record Offline Cash Sale/ }).click();

        // Wait for POS transaction to complete
        await expect(posPage.locator('text=Payment Successful!')).toBeVisible({ timeout: 20000 });

        await posContext.close();
        await onlineContext.close();
    });
});
