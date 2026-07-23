import { test, expect } from '@playwright/test';
import { setupMockContract } from './playwright/utils/mock_contract_setup';
import { login } from './playwright/utils/auth_helper';

test.describe('AI Unified Review & Reputation Management System', () => {
    test('Simulate Google Business Review webhook and verify Ambassador mitigation draft', async ({ page, request }) => {
        const tenantId = 'e2e-tenant-reputation-management';
        const customerName = 'Angry Customer';

        // 1. Setup mock data
        await setupMockContract(tenantId);

        // 2. Login
        await login(page, tenantId);

        // 3. Send Google Business Review Webhook (1-star)
        const webhookResponse = await request.post('/api/v1/local_seo/webhook', {
            data: {
                tenant_id: tenantId,
                review_id: 'review-123',
                reviewer_name: customerName,
                star_rating: 1,
                comment: 'This was the worst experience ever. Completely unacceptable.'
            }
        });
        expect(webhookResponse.ok()).toBeTruthy();

        // 4. Navigate to Agent Feed / The Ambassador
        const ambassadorDept = page.locator('button', { hasText: 'The Ambassador' });
        await expect(ambassadorDept).toBeVisible({ timeout: 15000 });
        await ambassadorDept.click();

        // 5. Verify the drafted reply card is present
        const dmCard = page.locator('[data-testid="ambassador-reply-card"]').first();
        await expect(dmCard).toBeVisible({ timeout: 15000 });

        // Check for presence of the negative review text
        await expect(page.locator('text=' + customerName)).toBeVisible();
        await expect(dmCard).toContainText('Google Business Review');
        await expect(dmCard).toContainText('1 Stars');

        // Check if there is an apologize/mitigate draft
        const draftTextarea = page.getByTestId('edit-ambassador-draft');
        await expect(draftTextarea).toBeVisible();
        const draftText = await draftTextarea.inputValue();
        expect(draftText.toLowerCase()).toContain('apolog'); // "apologize" or "apologies"

        // 6. Approve the draft
        const approveBtn = page.getByTestId('save-send-ambassador-reply').or(page.getByRole('button', { name: 'Approve & Send' })).first();
        await approveBtn.click();

        // 7. Verify the card disappears (approved)
        await expect(dmCard).not.toBeVisible({ timeout: 15000 });
    });
});
