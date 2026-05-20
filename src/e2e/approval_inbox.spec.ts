import { test, expect } from './fixtures';

test('AI Team Dashboard and Approval Inbox', async ({ page, request }) => {
  // Mock the API for testing the UI specifically
  await page.route('/api/agents/approvals', async (route) => {
    const json = {
      pending_approvals: [
        {
          id: 'e2e-approval-mock-1',
          tenant_id: 'mock-tenant',
          department: 'CustomerSuccess',
          description: 'Draft email for review: Maya ordered a vegan cake',
          status: 'Pending',
          action_risk: 'High'
        },
        {
          id: 'e2e-approval-mock-2',
          tenant_id: 'mock-tenant',
          department: 'Marketing',
          description: 'Draft Instagram Post: New vegan cakes available!',
          status: 'Pending',
          action_risk: 'Low'
        }
      ]
    };
    await route.fulfill({ json });
  });

  await page.route('/api/agents/approvals/*', async (route) => {
    await route.fulfill({ json: { success: true } });
  });

  // 1. User opens the app, authenticates and navigates to the Team Dashboard
  await page.goto('/');

  // Login via UI (from global-setup login structure)
  // Assuming the user is already logged in via global-setup.ts
  await page.goto('/team');

  // Assert Team Dashboard elements (375px mobile-first)
  await expect(page.locator('text=The Ambassador')).toBeVisible();
  await expect(page.locator('text=The Promoter')).toBeVisible();

  // "The Ambassador" has pending approvals indicator (e.g., a badge)
  const ambassadorCard = page.locator('text=The Ambassador').locator('..');
  // Badge is a span with text '1' now
  await expect(ambassadorCard.locator('span', { hasText: '1' }).first()).toBeVisible();

  // Verify approval inbox view for The Ambassador in the Action Feed
  await expect(page.locator('text=Draft email for review: Maya ordered a vegan cake')).toBeVisible();

  // 2. User filters to "The Ambassador" department
  await ambassadorCard.click();

  // 3. User clicks Edit, changes description, and approves
  const editBtn = page.locator('button', { hasText: 'Edit' }).first();
  await editBtn.click();

  const textarea = page.locator('textarea');
  await textarea.fill('Draft email for review: Maya ordered a delicious vegan cake');

  const saveAndApproveBtn = page.locator('button', { hasText: 'Save & Approve' }).first();
  await saveAndApproveBtn.click();

  // Wait for the action to be processed (mocking the UI removal)
  await expect(page.locator('text=Draft email for review: Maya ordered a vegan cake')).not.toBeVisible();
});
