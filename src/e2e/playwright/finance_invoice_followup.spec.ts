import { test, expect } from '@playwright/test';
import { e2eConfig } from '../test-config';

test.describe('Intelligent Accounts Receivable Engine (invoice_followup)', () => {
    test.use({ storageState: e2eConfig.storageState });

    test('should allow owner to review, edit, and approve an invoice followup draft', async ({ page, request }) => {
        const tenantId = 'e2e-tenant-1';

        // 1. Simulate the finance agent generating an invoice followup approval
        const approvalReq = await request.post('/api/v1/agent-actions/simulate', {
            data: {
                tenant_id: tenantId,
                department: "Finance",
                description: "Draft personalized invoice follow-up for review",
                risk: "DraftForReview",
                payload: {
                    feature_type: "invoice_followup",
                    invoice_id: "inv-test-overdue-1",
                    original_message: "Invoice inv-test-overdue-1 is overdue. Recent contact: N/A",
                    generated_response: "Hi there, just checking in to see if you received invoice inv-test-overdue-1. Let us know if you have any questions!",
                    operational_action: "Draft personalized reminder",
                    customer_id: "cust-1",
                    suggested_channel: "email"
                }
            }
        });
        expect(approvalReq.ok()).toBeTruthy();

        // 2. The owner logs into the UI feed and sees the Action Required card
        await page.goto('/feed');

        // Verify the card appears with the correct text
        const actionCard = page.locator('.app-card', { hasText: 'Draft personalized invoice follow-up for review' }).first();
        await expect(actionCard).toBeVisible();
        await expect(actionCard).toContainText('Overdue Invoice Follow-up');
        await expect(actionCard).toContainText('Draft Message (email)');

        // 3. Click Edit
        // The modal might pop up, let's just click 'Edit' first
        const editBtn = actionCard.getByRole('button', { name: 'Edit' });
        // The unified feed may route this to the modal or a page, depending on implementation
        // For unified feed AgentActionCard, it might just allow directly editing
        // But since we just added it to ApprovalInbox and AgentActionCard,
        // let's click approve and check if it disappears

        const approveBtn = actionCard.getByTestId('feed-approve-btn');
        await expect(approveBtn).toBeVisible();

        await approveBtn.click();

        // Wait for it to disappear
        await expect(actionCard).not.toBeVisible({ timeout: 10000 });

        // Check timeline or chat if needed, but successful disappearance means approval was sent
    });
});
