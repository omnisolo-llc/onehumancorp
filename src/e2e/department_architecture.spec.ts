import { test, expect } from '@playwright/test';

test.describe('Department Architecture UI', () => {
  test.beforeEach(async ({ page }) => {
    // Mock the fetch call to return sample pending approvals
    await page.route('/api/agents/approvals', async (route) => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          pending_approvals: [
            {
              id: 'approval-1',
              department: 'customer_success',
              description: 'Drafted quote for vegan cake.',
            },
            {
              id: 'approval-2',
              department: 'marketing',
              description: 'Suggested Instagram post for weekend sale.',
            }
          ],
        }),
      });
    });

    // Mock the post call to approve an action
    await page.route('**/api/agents/approvals/*', async (route) => {
      if (route.request().method() === 'POST') {
        await route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify({ success: true }),
        });
      } else {
        await route.continue();
      }
    });

    // Handle alert dialogs that pop up on approval/rejection
    page.on('dialog', dialog => dialog.accept());

    await page.goto('/');
  });

  test('should display Staff Updates section with drafted actions', async ({ page }) => {
    const staffUpdatesSection = page.locator('#staff-updates-section');
    await expect(staffUpdatesSection).toBeVisible();
    await expect(staffUpdatesSection.locator('text=Staff Updates')).toBeVisible();

    // Wait for the simulated fetch response to render
    await expect(page.locator('text=Drafted quote for vegan cake.')).toBeVisible();
    await expect(page.locator('text=The Ambassador')).toBeVisible();

    await expect(page.locator('text=Suggested Instagram post for weekend sale.')).toBeVisible();
    await expect(page.locator('text=The Promoter')).toBeVisible();
  });

  test('should allow approving an action', async ({ page }) => {
    const card = page.locator('.card', { hasText: 'Drafted quote for vegan cake.' });
    await expect(card).toBeVisible();
    const approveBtn = card.getByRole('button', { name: 'Approve' });
    await expect(approveBtn).toBeVisible();

    // Click approve
    await approveBtn.click();

    // Since we mock the API response and automatically accept the alert, the page should just stay on the same UI
    // We verify the button was interactable
  });

  test('should allow editing/rejecting an action', async ({ page }) => {
    const card = page.locator('.card', { hasText: 'Suggested Instagram post for weekend sale.' });
    await expect(card).toBeVisible();
    const editRejectBtn = card.getByRole('button', { name: 'Edit / Reject' });
    await expect(editRejectBtn).toBeVisible();

    // Click edit/reject
    await editRejectBtn.click();
  });

  test('should display department tabs and auto-pilot toggles on the Agents screen', async ({ page }) => {
    // Navigate to agents screen
    await page.getByRole('button', { name: 'My Agents' }).first().click();

    const agentsScreen = page.locator('#agents-screen');
    await expect(agentsScreen).toBeVisible();
    await expect(agentsScreen.getByRole('heading', { name: 'Staff (Agents)' })).toBeVisible();

    // Check for departments
    await expect(agentsScreen.locator('text=📦 The Manager')).toBeVisible();
    await expect(agentsScreen.locator('text=📣 The Promoter')).toBeVisible();
    await expect(agentsScreen.locator('text=🤝 The Salesperson')).toBeVisible();
    await expect(agentsScreen.locator('text=❤️ The Ambassador')).toBeVisible();
    await expect(agentsScreen.locator('text=💰 The Accountant')).toBeVisible();
    await expect(agentsScreen.locator('text=⚖️ The Protector')).toBeVisible();
    await expect(agentsScreen.locator('text=📈 The Advisor')).toBeVisible();

    // Check for auto-pilot toggles
    const toggles = agentsScreen.getByLabel(/Auto-pilot/);
    expect(await toggles.count()).toBeGreaterThanOrEqual(7);
  });
});
