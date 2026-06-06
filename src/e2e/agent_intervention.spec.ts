import { test, expect } from './fixtures';

test.describe('Agent Human-in-the-Loop Intervention', () => {
  test('should show intervention panel when agent is paused and resume on input', async ({ page }) => {
    // 1. Mock the API responses
    await page.route('/api/agents/approvals*', async (route) => {
      const json = {
        pending_approvals: [
          {
            id: 'app_1',
            tenant_id: 'e2e-tenant',
            department: 'OPERATIONS',
            description: 'Authentication required for the shipping provider. Please provide your API key.',
            status: 'USER_INTERVENTION_REQUIRED',
            action_risk: 'HIGH',
            payload: {
              is_intervention: true,
              task_id: 'task_123',
              tool_call_id: 'call_456'
            }
          }
        ]
      };
      await route.fulfill({ json });
    });

    await page.route('/api/agents/approvals/resolve', async (route) => {
      await route.fulfill({ json: { success: true } });
    });

    // 2. Go to dashboard
    await page.goto('/dashboard');

    // 3. Click the proposal that requires intervention
    await expect(page.getByText('Authentication required for the shipping provider')).toBeVisible();
    await page.getByRole('button', { name: 'Approve' }).click();

    // 4. Verify Intervention Panel appears
    await expect(page.getByText('Human Intervention Needed')).toBeVisible();
    await expect(page.getByText('Reason for Pause')).toBeVisible();
    await expect(page.getByText('Authentication required for the shipping provider')).toBeVisible();

    // 5. Provide input and submit
    const textarea = page.getByPlaceholder('Provide information or instructions');
    await textarea.fill('my-api-key-789');
    await page.getByRole('button', { name: 'Send to Agent' }).click();

    // 6. Verify panel closes and item is removed
    await expect(page.getByText('Human Intervention Needed')).not.toBeVisible();
    await expect(page.getByText('Authentication required for the shipping provider')).not.toBeVisible();
    await expect(page.getByText('All caught up!')).toBeVisible();
  });
});
