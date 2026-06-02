import { test, expect } from '@playwright/test';

test.describe('Customer Success Ambassador Engine', () => {
  // Use a simulated tenant
  const tenantId = 'e2e-tenant-csa';

  test.beforeEach(async ({ page }) => {
    // Setup simulated login
    await page.goto('/login');
    // For test simulation, directly setting localStorage or cookies if needed
    // Assuming simple pathing for this mock scenario
  });

  test('should display escalated messages in the Approval Inbox', async ({ page }) => {
    // Mocking an escalated message event
    await page.route('**/api/v1/team/approvals*', async (route) => {
      await route.fulfill({
        status: 200,
        json: {
          approvals: [
            {
              id: 'approval-1',
              tenant_id: tenantId,
              department: 'CustomerSuccess',
              description: 'Draft reply for review on instagram',
              status: 'PendingApproval',
              action_risk: 'HIGH',
              payload: {
                feature_type: 'ambassador_reply',
                platform: 'instagram',
                original_message: 'Can you make this cake vegan?',
                generated_response: 'Let me check if we can make a vegan version of that.',
                context_used: 'Vegan orders require 48h notice.',
                confidence_score: 70.0
              }
            }
          ]
        }
      });
    });

    await page.goto('/team');

    // Check if the component renders the draft
    await expect(page.locator('text=Customer Message (instagram)')).toBeVisible();
    await expect(page.locator('text="Can you make this cake vegan?"')).toBeVisible();
    await expect(page.locator('text=Let me check if we can make a vegan version of that.')).toBeVisible();

    // Verify touch targets exist
    await expect(page.locator('button:has-text("Approve & Send")').first()).toBeVisible();
    await expect(page.locator('button:has-text("Review")').first()).toBeVisible();
  });
});
