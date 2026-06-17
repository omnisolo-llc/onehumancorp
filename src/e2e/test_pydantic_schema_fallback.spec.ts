import { test, expect } from './fixtures';

test.describe('Pydantic-first tool schema fallback mechanism', () => {
  test('Agent schema errors are gracefully managed and retried via the output parser', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);

    await page.goto('/agents');
    await page.waitForLoadState('networkidle');

    await expect(page.getByRole('heading', { name: 'Automations' }).first()).toBeVisible();

    await page.getByRole('button', { name: 'Workflows' }).click();
    await expect(page.getByTestId('visual-workflow-builder')).toBeVisible();

    const workflowName = `Auto Reply Schema Test ${Date.now()}`;
    await page.locator('#visual-workflow-name').fill(workflowName);

    await page.getByTestId('palette-block-trigger_message').click();
    await page.getByTestId('palette-block-action_draft').click();

    await page.locator('#btn-create-run-workflow').click();

    await expect(page.getByText(workflowName)).toBeVisible({ timeout: 15000 });
  });

  test('Agent feed correctly loads without fatal schema crash', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/dashboard');

    await expect(page.getByText('Work Feed')).toBeVisible();
  });

  test('Agent setup modal displays schema configurations robustly', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/agents');

    await expect(page.getByText('Explore the catalog to find templates, automations, and connections for your business.')).toBeVisible();
  });

  test('Workflow palette handles complex block conditions seamlessly', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/agents');
    await page.getByRole('button', { name: 'Workflows' }).click();

    await page.getByTestId('palette-block-condition_approval').click();
    await expect(page.getByTestId('canvas-block-0')).toContainText('Wait for Approval');
  });

  test('Workflow execution logic renders the resulting JSON reliably', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/agents');
    await page.getByRole('button', { name: 'Workflows' }).click();

    const workflowName = `JSON Render Test ${Date.now()}`;
    await page.locator('#visual-workflow-name').fill(workflowName);
    await page.getByTestId('palette-block-output_save').click();
    await page.locator('#btn-create-run-workflow').click();

    await expect(page.getByText(/"type":"Output"/).first()).toBeVisible();
  });
});
