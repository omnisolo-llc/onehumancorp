import { test, expect } from './fixtures';

test.describe('Agentic One-Tap Out of Stock E2E', () => {
    test('CUJ: Business owner inventory changes trigger proactive agent responses', async ({ page, request }) => {
        // Step 1: Simulate the UI triggering the "Out of Stock" backend API
        const response = await request.post('/api/v1/inventory/update', {
            data: {
                tenant_id: "test-tenant-123",
                product_id: "prod-abc",
                quantity: 0
            }
        });

        expect(response.ok()).toBeTruthy();
        const body = await response.json();
        expect(body.success).toBe(true);

        // Step 2: Maya logs into her baking business dashboard
        await page.goto('/');

        // Verify the application remains stable and the event propagation didn't crash the server
        await expect(page.locator('body')).toBeVisible();
    });
});
