import { test, expect } from '@playwright/test';

test.describe('Inventory Restock Action Feed', () => {
    test('Persona: Boutique owner sees 1-tap restock action for predictive inventory', async ({ page }) => {
        await page.goto('http://localhost:3000/dashboard');

        // Add mock local storage to act as authenticated
        await page.evaluate(() => {
            localStorage.setItem('tenant_id', 'tenant1');
            localStorage.setItem('has_pro', 'true');
        });

        // The dashboard triggers API requests to /api/agents/approvals
        // We will intercept this and inject our predictive restock action
        await page.route('/api/agents/approvals', async route => {
            const json = {
                pending_approvals: [
                    {
                        id: 'test-restock-approval-1',
                        tenant_id: 'tenant1',
                        department: 'operations',
                        description: 'Inventory for Fast Selling Item is low (50 remaining). Average daily sales: 10.0. Will run out in 5.0 days.',
                        status: 'PendingApproval',
                        action_risk: 'DraftForReview',
                        payload: {
                            feature_type: "restock_action",
                            target_product_id: "prod_high_vel",
                            target_product_name: "Fast Selling Item",
                            restock_qty: 300
                        }
                    }
                ]
            };
            await route.fulfill({ json });
        });

        // Intercept the restock API call
        await page.route('/api/agents/restock', async route => {
            await route.fulfill({ json: { success: true } });
        });

        await page.goto('http://localhost:3000/dashboard');

        // Verify the restock card is rendered beautifully
        await expect(page.locator('text=Operations Department')).toBeVisible();
        await expect(page.locator('text=Approve & Restock')).toBeVisible();

        // Click the approve button
        await page.click('text=Approve & Restock');

        // Verify the card disappears from the action required list
        await expect(page.locator('text=Approve & Restock')).not.toBeVisible();
    });
});
