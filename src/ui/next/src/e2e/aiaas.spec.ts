import { test, expect } from '@playwright/test';

test.describe('AIaaS Core Capabilities (Issue #22669)', () => {

  test('should navigate to the AI Inbox and view workflows and personas', async ({ page }) => {
    // Navigate to the Team dashboard
    await page.goto('/team');
    await expect(page.getByRole('heading', { name: 'Your Team' })).toBeVisible();

    // Mock the backend API responses for the AI Inbox
    await page.route('/api/agents/aiaas/personas', async (route) => {
      const json = {
        personas: [
          {
            id: 'persona-e2e-1',
            name: 'E2E Persona',
            system_prompt: 'You are an E2E testing assistant.',
            capabilities: ['drafting', 'review']
          }
        ]
      };
      await route.fulfill({ json });
    });

    await page.route('/api/agents/aiaas/workflows', async (route) => {
      const json = {
        workflows: [
          {
            workflow_id: 'wf-e2e-1',
            persona_id: 'persona-e2e-1',
            trigger_event: 'test_event',
            status: 'active'
          }
        ]
      };
      await route.fulfill({ json });
    });

    // Click on the AI Inbox button
    await page.getByRole('button', { name: 'AI Inbox' }).click();

    // Verify we are on the AI Inbox page
    await expect(page.getByRole('heading', { name: 'AI Inbox' })).toBeVisible();

    // Verify Persona is rendered
    await expect(page.getByText('E2E Persona')).toBeVisible();
    await expect(page.getByText('You are an E2E testing assistant.')).toBeVisible();
    await expect(page.getByText('drafting')).toBeVisible();

    // Verify Workflow is rendered
    await expect(page.getByText('test_event')).toBeVisible();
    await expect(page.getByText('Persona: persona-e2e-1')).toBeVisible();
    await expect(page.getByText('active', { exact: true })).toBeVisible();
  });
});
