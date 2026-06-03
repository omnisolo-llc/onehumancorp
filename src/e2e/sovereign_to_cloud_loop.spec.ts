import { test, expect } from '@playwright/test';

test.describe('Sovereign-to-Cloud Loop', () => {
    test('User can share a specific agentic output (Case Study) via Cloud Bridge in Approval Inbox', async ({ page }) => {
        // Mock the API response to include a case_study
        await page.route('/api/agents/approvals', async route => {
          const json = {
            pending_approvals: [
              {
                id: 'req_123',
                tenant_id: 'test-tenant',
                department: 'marketing',
                description: 'Case Study Draft | Payload: {"feature_type": "case_study", "service_name": "Web Design", "media_url": "https://example.com/image.jpg", "generated_description": "Great project."}',
                status: 'pending',
                action_risk: 'low',
                payload: {
                  feature_type: "case_study",
                  service_name: "Web Design",
                  media_url: "https://example.com/image.jpg",
                  generated_description: "Great project."
                }
              }
            ]
          };
          await route.fulfill({ json });
        });

        // Go to the Team page
        await page.goto('http://localhost:3000/team');

        // Wait for departments to load
        await page.waitForSelector('text=The Promoter'); // Marketing department

        // Click on the Marketing department (The Promoter)
        const promoterCard = page.locator('button', { hasText: 'The Promoter' }).first();
        await expect(promoterCard).toBeVisible();
        await promoterCard.click();

        // Now we should be in ApprovalInbox. Check if "Share via Cloud Bridge" is visible.
        const shareButton = page.locator('button', { hasText: 'Share via Cloud Bridge' });
        await expect(shareButton).toBeVisible();

        // Click it to open the modal
        await shareButton.click();

        // Check if the specific asset modal is open (since ApprovalInbox replaced the generic team modal, it is the first or only one)
        const modalTitle = page.locator('h2', { hasText: 'Cloud Bridge Invite' }).first();
        await expect(modalTitle).toBeVisible();

        // Check if the copy link button is visible
        const copyButton = page.locator('button', { hasText: 'Copy Link' }).first();
        await expect(copyButton).toBeVisible();

        // The input should contain the asset id
        const input = page.locator('input#cloud-bridge-invite-link').first();
        await expect(input).toBeVisible();
        await expect(input).toHaveValue(/asset=req_123/);
    });
});
