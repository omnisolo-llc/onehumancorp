import { test, expect } from './fixtures';
import { v4 as uuidv4 } from 'uuid';

test.describe('AI Agent Department Approval Flow', () => {
    test('Business owner approves an AI-generated draft in the queue triggered by an order', async ({ page, request }) => {
        // Business Persona: Maya (Home Baker)
        // She wants to check her AI agents' activity and approve a draft reply to a customer.

        // 1. Simulate external event "New Order"
        // This will route to Operations, which hands off to Customer Success to draft a thank you note.
        const orderId = uuidv4();
        const tenantId = 'e2e-tenant';

        // Wait, the webhook takes payload {"tenant_id", "message", "source"}
        const webhookResponse = await request.post('/api/agents/webhook', {
            data: {
                tenant_id: tenantId,
                message: "order_placed",
                source: "stripe",
                order_id: orderId
            }
        });

        expect(webhookResponse.ok()).toBeTruthy();

        // 2. Wait for the async orchestrator to finish creating the draft.
        // We'll give it a few seconds since it involves async mesh routing and AI budget checks.
        await page.waitForTimeout(5000);

        // 3. Navigate to agents dashboard
        await page.goto('/agents');

        // 4. Wait for the page to load
        await expect(page.getByRole('heading', { name: 'AI Departments' }).first()).toBeVisible();

        // 5. Switch to Needs Approval tab
        await page.getByRole('button', { name: 'Needs Approval' }).click();

        // 6. Verify that there is a pending approval draft
        // The webhook logic for "order_placed" -> Operations -> CS creates: "Send personalized thank you & shipping ETA"
        await expect(page.getByText('Send personalized thank you').first()).toBeVisible();
        await expect(page.getByText('customer_success').first()).toBeVisible();
        await expect(page.getByText('Draft For Review').first()).toBeVisible();

        // 7. Approve the draft
        const approveButton = page.getByRole('button', { name: 'Approve & Send' }).first();
        await approveButton.click();

        // 8. Verify that it was approved (the draft disappears)
        await expect(approveButton).toBeHidden();

        // 9. Check the activity feed to see the approved action
        await page.getByRole('button', { name: 'Activity Feed' }).click();
        await expect(page.getByText('Approved').first()).toBeVisible();
        await expect(page.getByText('Send personalized thank you').first()).toBeVisible();
        await expect(page.getByText('customer_success').first()).toBeVisible();
    });
});
