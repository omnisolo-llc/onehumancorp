import { test, expect } from './fixtures';

test.describe('Visual/low-code orchestration', () => {
  test('user can visually construct a workflow by connecting blocks and execute it', async ({ page, unlimitedAdminUser, loginAs }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/agents');
    await page.getByRole('button', { name: 'Workflows' }).click();

    // The visual builder should be visible
    await expect(page.getByTestId('visual-workflow-builder')).toBeVisible();

    const workflowName = `Visual Auto-Reply ${Date.now()}`;
    await page.locator('#visual-workflow-name').fill(workflowName);

    // 1. Add Trigger Block
    await page.getByTestId('palette-block-trigger_message').click();
    await expect(page.getByTestId('canvas-block-0')).toContainText('Inbound Message');

    // 2. Add Action Block
    await page.getByTestId('palette-block-action_analyze').click();
    await expect(page.getByTestId('canvas-block-1')).toContainText('Analyze Sentiment');

    // 3. Add Condition Block
    await page.getByTestId('palette-block-condition_approval').click();
    await expect(page.getByTestId('canvas-block-2')).toContainText('Wait for Approval');

    // 4. Add Output Block
    await page.getByTestId('palette-block-output_send').click();
    await expect(page.getByTestId('canvas-block-3')).toContainText('Send Message');

    // Verify Remove block logic
    await page.getByTestId('palette-block-action_draft').click();
    await expect(page.getByTestId('canvas-block-4')).toContainText('Draft Reply');

    // Remove the 4th block (index 4)
    await page.getByTestId('canvas-block-4').getByRole('button', { name: 'Remove block' }).click();
    await expect(page.getByTestId('canvas-block-4')).not.toBeVisible();

    // Create & Run
    await page.locator('#btn-create-run-workflow').click();

    // Verify it was created and is visible in the active workflows list
    await expect(page.getByText(workflowName)).toBeVisible();

    // Verify the compiled JSON payload output exists in the workflow view.
    await expect(page.getByText(/"entrypoint":"trigger_message_\d+"/).first()).toBeVisible();
    await expect(page.getByText(/"type":"Action"/).first()).toBeVisible();
    await expect(page.getByText(/"label":"Wait for Approval"/).first()).toBeVisible();
  });

  test('visual workflow builder prevents saving without name or blocks', async ({ page, unlimitedAdminUser, loginAs }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/agents');
    await page.getByRole('button', { name: 'Workflows' }).click();

    await expect(page.locator('#btn-create-run-workflow')).toBeDisabled();

    await page.locator('#visual-workflow-name').fill('Test Workflow');
    await expect(page.locator('#btn-create-run-workflow')).toBeDisabled();

    await page.getByTestId('palette-block-trigger_message').click();
    await expect(page.locator('#btn-create-run-workflow')).toBeEnabled();
  });

  test('visual workflow blocks correctly display their types visually', async ({ page, unlimitedAdminUser, loginAs }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/agents');
    await page.getByRole('button', { name: 'Workflows' }).click();

    await page.getByTestId('palette-block-trigger_message').click();
    await page.getByTestId('palette-block-action_research').click();
    await page.getByTestId('palette-block-condition_approval').click();
    await page.getByTestId('palette-block-output_save').click();

    // Verify correct type labels appear on the canvas blocks
    await expect(page.getByTestId('canvas-block-0').locator('span').first()).toHaveText(/trigger/i);
    await expect(page.getByTestId('canvas-block-1').locator('span').first()).toHaveText(/action/i);
    await expect(page.getByTestId('canvas-block-2').locator('span').first()).toHaveText(/condition/i);
    await expect(page.getByTestId('canvas-block-3').locator('span').first()).toHaveText(/output/i);
  });

  test('visual builder canvas is empty initially', async ({ page, unlimitedAdminUser, loginAs }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/agents');
    await page.getByRole('button', { name: 'Workflows' }).click();

    await expect(page.getByText('Click blocks on the left to add them to your workflow')).toBeVisible();
    await expect(page.getByTestId('canvas-block-0')).not.toBeVisible();
  });

  test('removing all blocks disables save and shows empty state', async ({ page, unlimitedAdminUser, loginAs }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/agents');
    await page.getByRole('button', { name: 'Workflows' }).click();

    await page.locator('#visual-workflow-name').fill('Temp Workflow');

    await page.getByTestId('palette-block-trigger_message').click();
    await expect(page.locator('#btn-create-run-workflow')).toBeEnabled();

    await page.getByTestId('canvas-block-0').getByRole('button', { name: 'Remove block' }).click();

    await expect(page.getByText('Click blocks on the left to add them to your workflow')).toBeVisible();
    await expect(page.locator('#btn-create-run-workflow')).toBeDisabled();
  });
});
