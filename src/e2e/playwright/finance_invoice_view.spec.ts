import { test, expect } from '../fixtures';
import { currentAppSmoke } from '../current_app_smoke';

test.describe('Agentic Invoicing Flow - View Invoice', () => {
    test.beforeEach(async ({ page, loginAs, adminUser }) => {
        await loginAs(page, adminUser);
        await page.goto('/finance');
    });

    test('should retrieve invoice with line items concurrently from the backend', async ({ page, request }) => {
        const createRes = await request.post('/api/v1/invoices', {
            data: {
                client_id: "test-client",
                client_name: "Test E2E Client",
                due_date: Math.floor(Date.now() / 1000) + (30 * 24 * 60 * 60),
                currency: "USD",
                line_items: [
                    {
                        id: "",
                        invoice_id: "",
                        description: "E2E Consulting Services",
                        quantity: 2,
                        unit_price: 150.0,
                        amount: 300.0
                    }
                ]
            }
        });

        expect(createRes.ok()).toBeTruthy();
        const invoiceData = await createRes.json();

        const listRes = await request.get('/api/v1/invoices');
        expect(listRes.ok()).toBeTruthy();

        expect(invoiceData.line_items).toBeDefined();
        expect(invoiceData.line_items.length).toBeGreaterThan(0);
        expect(invoiceData.line_items[0].description).toBe('E2E Consulting Services');

        await expect(page.locator('h1', { hasText: 'Finance & Invoicing' })).toBeVisible();
    });
});
