import { test, expect } from './fixtures';

test('Business owner interacts with AI Agent Departments', async ({ page }) => {
  const id = `agent-dept-${Date.now()}`;
  await page.addInitScript((tenantId) => {
    localStorage.setItem('tenant_id', tenantId);
    localStorage.setItem('user_id', tenantId);
  }, id);

  // Navigating to the team page
  await page.goto('/team');

  // Assert TeamPage loads successfully
  await expect(page.getByRole('heading', { name: 'Your Team' })).toBeVisible();

  // Wait for the mock API response or the empty state to render
  // As this is a generic test without actual pending approvals mocked initially, we expect either department cards to load or an empty state.

  // Click on 'The Manager' department card
  await page.getByRole('button', { name: /The Manager/i }).click();

  // Assert ApprovalInbox opens for the department
  await expect(page.getByRole('heading', { name: 'The Manager' })).toBeVisible();
  await expect(page.getByText('Approval Inbox')).toBeVisible();

  // Check empty state
  await expect(page.getByText('All Caught Up!')).toBeVisible();

  // Navigate back to the team page
  await page.locator('button.w-11.h-11').click();

  // Assert we are back to TeamPage
  await expect(page.getByRole('heading', { name: 'Your Team' })).toBeVisible();
});
