import { test, expect } from '@playwright/test';
import { e2e_login } from '../support/login';

test.describe('POS Inventory & Double Booking Prevention', () => {
    test.beforeEach(async ({ page }) => {
        await e2e_login(page);
    });

    test('should prevent double booking of limited stock via concurrent tap to pay and online checkout', async ({ browser, page }) => {
        // Step 1: Open POS in a separate context to represent the physical store terminal
        const posContext = await browser.newContext();
        const posPage = await posContext.newPage();
        await e2e_login(posPage);

        await posPage.goto('/pos/walkup');
        await posPage.waitForSelector('text=POS Mode', { timeout: 10000 }).catch(() => {});

        // Add item to cart
        const productLocators = await posPage.locator('.cursor-pointer').all();
        if (productLocators.length > 0) {
            await productLocators[0].click();
        } else {
            console.log('No products found, skipping item click');
        }

        // We can't fully mock Tap to Pay in E2E since it relies on Stripe Terminal
        // But we can trigger the reserve intent endpoint directly to simulate it
        const res = await posPage.request.post('/api/v1/checkout/session', {
            data: {
                tenant_id: 'test_tenant',
                type: 'IN_PERSON',
                amount_cents: 1000,
                cart_payload: {
                    items: [
                        {
                            product: {
                                id: 'e2e_test_limited_product'
                            },
                            quantity: 1
                        }
                    ]
                }
            }
        });

        // Ensure session creation (and lock) is successful
        expect(res.ok()).toBeTruthy();

        // Step 2: In the original page, act as an online customer
        await page.goto('/storefront');
        // This simulates checking out the same item online
        const onlineRes = await page.request.post('/api/v1/checkout/session', {
            data: {
                tenant_id: 'test_tenant',
                type: 'ONLINE',
                amount_cents: 1000,
                cart_payload: {
                    items: [
                        {
                            product: {
                                id: 'e2e_test_limited_product'
                            },
                            quantity: 1
                        }
                    ]
                }
            }
        });

        // Assert that the online customer gets blocked because the item is locked by POS
        // We expect a 409 Conflict due to "Item is currently being checked out by another customer."
        expect(onlineRes.status()).toBe(409);
        const data = await onlineRes.json();
        expect(data.error_message).toContain('Item is currently being checked out');

        await posContext.close();
    });
});
