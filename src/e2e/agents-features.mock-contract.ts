import { test, expect } from '@playwright/test';

test.describe('OHC Expert Center Comprehensive E2E Flows', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the agents dashboard page
    await page.goto('/agents');
  });

  test('should support summoning experts, selecting modes/models, and starting tasks', async ({ page }) => {
    // Verify catalog title
    await expect(page.getByRole('heading', { name: 'Expert Center' })).toBeVisible();

    // 1. Click Featured Scenarios cards in carousel
    const contentCreationCard = page.getByText('Content Creation');
    await expect(contentCreationCard).toBeVisible();
    await contentCreationCard.click();

    const investmentAnalysisCard = page.getByText('Investment Analysis');
    await expect(investmentAnalysisCard).toBeVisible();
    await investmentAnalysisCard.click();

    // 2. Search catalog filtering
    const searchInput = page.getByPlaceholder('Search experts, roles, skills, connectors...');
    await expect(searchInput).toBeVisible();
    await searchInput.fill('Promoter');
    
    // Verify filtered card is visible
    const promoterCard = page.getByTestId('expert-card-the-promoter');
    await expect(promoterCard).toBeVisible();

    // 3. Click Most Used Scenarios buttons
    const growthStrategistBtn = page.getByRole('button', { name: 'Growth Strategist' });
    await expect(growthStrategistBtn).toBeVisible();
    await growthStrategistBtn.click();

    // Verify ready message
    await expect(page.getByText('Growth Strategist is ready')).toBeVisible();

    // 4. Test Task Composer inputs and mode selection
    const askMode = page.getByRole('button', { name: 'Ask' });
    await expect(askMode).toBeVisible();
    await askMode.click();

    const planMode = page.getByRole('button', { name: 'Plan' });
    await expect(planMode).toBeVisible();
    await planMode.click();

    const craftMode = page.getByRole('button', { name: 'Craft' });
    await expect(craftMode).toBeVisible();
    await craftMode.click();

    // Toggle custom provider, constraints, outputs, models
    await page.getByLabel('Task prompt').fill('Execute weekly review report');
    await page.getByLabel('Context references').fill('@sales @finance');
    await page.getByLabel('Attachments').fill('balance.csv');
    await page.getByLabel('Custom provider').fill('https://api.openai.com/v1');
    await page.getByLabel('Work directory').fill('/workspace/finance-dir');
    await page.getByLabel('Output format').selectOption('Spreadsheet');
    await page.getByLabel('Task constraints').fill('Verify margins > 20%');

    // Model Selector dropdown check
    const modelSelector = page.locator('span:has-text("Model") + select');
    await expect(modelSelector).toBeVisible();
    await modelSelector.selectOption('Local Ollama');

    // Mock the backend API responses for hiring
    await page.route('/api/v1/agents/hire', async route => {
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

    // Click find, search, upload, create, bulk buttons inside Skills
    await page.getByRole('button', { name: 'Find skill' }).click();
    await page.getByRole('button', { name: 'Search installed skills' }).click();
    await page.getByRole('button', { name: 'Upload local skill' }).click();
    await page.getByRole('button', { name: 'Create skill from prompt' }).click();
    await page.getByRole('button', { name: 'Disable skill' }).click();
    await page.getByRole('button', { name: 'Uninstall skill' }).click();
    await page.getByRole('button', { name: 'Bulk uninstall' }).click();

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

    // Verify upgrade pro details link
    const upgradeLink = page.getByRole('link', { name: 'Upgrade to Pro' });
    await expect(upgradeLink).toBeVisible();

    // Click X share to unlock
    await page.getByRole('button', { name: 'Share on X to get 7 Days Free' }).click();

    // Paywall modal should close
    await expect(page.getByRole('heading', { name: 'Upgrade to Pro' })).not.toBeVisible();
  });

  test('should support automations rules, memory entries, remote control, and results outputs', async ({ page }) => {
    // Navigating Automations
    await page.getByRole('button', { name: 'Automations' }).click();
    await expect(page.getByText('Scheduled Tasks')).toBeVisible();
    await expect(page.getByText('Weekly business review')).toBeVisible();
    await expect(page.getByText('Weekly stats execution')).toBeVisible();

    // Navigating Memory panel
    await page.getByRole('button', { name: 'Memory' }).click();
    await expect(page.getByRole('heading', { name: 'Consolidated Memory' })).toBeVisible();
    await expect(page.getByText('Brand voice prefers practical, friendly copy.')).toBeVisible();

    // Navigating Remote Control
    await page.getByRole('button', { name: 'Remote control' }).click();
    await expect(page.getByRole('heading', { name: 'Remote control' })).toBeVisible();
    await expect(page.getByText('/summon Growth Strategist')).toBeVisible();

    // Navigating Data Management
    await page.getByRole('button', { name: 'Data management' }).click();
    await expect(page.getByRole('heading', { name: 'Data Management' })).toBeVisible();
    await expect(page.getByText('Shared files')).toBeVisible();
    await expect(page.getByText('Unshare queue')).toBeVisible();

    // Navigating Results panel
    await page.getByRole('button', { name: 'Results' }).click();
    await expect(page.getByRole('heading', { name: 'Results' })).toBeVisible();
    
    // Result tabs
    await page.getByRole('button', { name: 'Artifacts' }).first().click();
    await page.getByRole('button', { name: 'All files' }).first().click();
    await page.getByRole('button', { name: 'Diffs' }).first().click();
    await page.getByRole('button', { name: 'Preview' }).first().click();

    // Action buttons inside results panel
    await page.getByRole('button', { name: 'Share result' }).first().click();
    await page.getByRole('button', { name: 'Download file' }).first().click();
    await page.getByRole('button', { name: 'Copy to workspace' }).first().click();
    await page.getByRole('button', { name: 'Archive task' }).first().click();
    await page.getByRole('button', { name: 'Unarchive' }).first().click();
  });

  test('should show Needs Approval and Feed panels', async ({ page }) => {
    // Navigate to Feed
    await page.getByRole('button', { name: 'Activity Feed' }).click();
    await expect(page.getByRole('heading', { name: 'Activity Feed' })).toBeVisible();

    // Navigate to Approvals
    await page.getByRole('button', { name: /Needs Approval/i }).click();
    await expect(page.getByRole('heading', { name: 'Needs Approval' })).toBeVisible();

    // Test clicking Needs Approval action buttons
    // Since mock list initially has items, click Approve & Send
    await page.route('/api/v1/agents/approvals/*', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ success: true }),
      });
    });
    
    // Click Approve & Send
    const approveBtn = page.getByRole('button', { name: 'Approve & Send' }).first();
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();
  });
});
