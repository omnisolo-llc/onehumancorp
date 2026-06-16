import { test, expect } from '@playwright/test';

test.describe('OHC Expert Center E2E Flows', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the agents dashboard page
    await page.goto('/agents');
  });

  test('should support summoning experts, selecting modes/models, and starting tasks', async ({ page }) => {
    // Verify catalog title
    await expect(page.getByRole('heading', { name: 'Expert Center' })).toBeVisible();

    // Summon Growth Strategist
    const growthCard = page.getByTestId('expert-card-growth-strategist');
    await expect(growthCard).toBeVisible();
    await growthCard.getByRole('button', { name: /Summon/i }).click();

    // Verify ready message
    await expect(page.getByText('Growth Strategist is ready')).toBeVisible();

    // Fill in task composer details
    await page.getByLabel('Task prompt').fill('Launch marketing promo plan');
    await page.getByLabel('Context references').fill('@sales @marketing');
    await page.getByLabel('Attachments').fill('promo.csv');
    await page.getByLabel('Custom provider').fill('https://api.myllm.com');
    await page.getByLabel('Work directory').fill('/workspace/promo-dir');
    await page.getByLabel('Output format').selectOption('Spreadsheet');
    await page.getByLabel('Task constraints').fill('Max budget $200');

    // Mock the backend API responses
    await page.route('/api/agents/hire', async route => {
      await route.fulfill({
        status: 201,
        contentType: 'application/json',
        body: JSON.stringify({
          id: 'test-wf-id',
          status: 'running',
          workflow_id: 'workflow-promo-e2e',
        }),
      });
    });

    // Submit task
    await page.getByRole('button', { name: 'Start task' }).click();

    // Verify workflow result box is updated
    await expect(page.getByText('workflow-promo-e2e')).toBeVisible();
  });

  test('should navigate tabs and toggle skill/connector grid items', async ({ page }) => {
    // Go to Skills tab
    await page.getByRole('button', { name: 'Skills' }).click();
    await expect(page.getByRole('heading', { name: 'Skill Market' })).toBeVisible();

    // Verify skill market grid toggles
    const skillBtn = page.getByRole('button', { name: /Web Research Enabled/i });
    await expect(skillBtn).toBeVisible();
    await skillBtn.click();
    await expect(page.getByRole('button', { name: /Web Research Installed/i })).toBeVisible();

    // Go to Connectors tab
    await page.getByRole('button', { name: 'Connectors' }).click();
    await expect(page.getByRole('heading', { name: 'Connector Center' })).toBeVisible();

    const connectorBtn = page.getByRole('button', { name: /Stripe Selected/i });
    await expect(connectorBtn).toBeVisible();
    await connectorBtn.click();
    await expect(page.getByRole('button', { name: /Stripe Connected/i })).toBeVisible();
  });

  test('should open paywall dialog and toggle pro mode upgrade', async ({ page }) => {
    // Try to toggle Pro Mode in the header
    const proSwitch = page.getByLabel('Toggle Pro Mode');
    await expect(proSwitch).toBeVisible();
    await proSwitch.click();

    // Paywall modal should appear
    await expect(page.getByRole('heading', { name: 'Upgrade to Pro' })).toBeVisible();

    // Click X share to unlock
    await page.getByRole('button', { name: 'Share on X to get 7 Days Free' }).click();

    // Paywall modal should close
    await expect(page.getByRole('heading', { name: 'Upgrade to Pro' })).not.toBeVisible();
  });

  test('should display automated rules and timelines in automations', async ({ page }) => {
    await page.getByRole('button', { name: 'Automations' }).click();
    await expect(page.getByText('Scheduled Tasks')).toBeVisible();
    await expect(page.getByText('Weekly business review')).toBeVisible();
    await expect(page.getByText('Weekly stats execution')).toBeVisible();
  });

  test('should show Needs Approval and Feed panels', async ({ page }) => {
    // Navigate to Feed
    await page.getByRole('button', { name: 'Activity Feed' }).click();
    await expect(page.getByRole('heading', { name: 'Activity Feed' })).toBeVisible();

    // Navigate to Approvals
    await page.getByRole('button', { name: /Needs Approval/i }).click();
    await expect(page.getByRole('heading', { name: 'Needs Approval' })).toBeVisible();
  });
});
