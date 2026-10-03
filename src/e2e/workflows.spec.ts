import { test, expect } from './fixtures';
import { generationAcceptance, generationGateReason } from './generation-acceptance';

test.describe('Agent Workflows', () => {
  test('unconfigured text analysis retains the workflow instead of inventing CLI execution', async ({ page }) => {
    test.skip(generationAcceptance, 'This contract requires unconfigured text analysis.');
    const submitted: string[] = [];
    page.on('request', request => {
      if (request.method() === 'POST' && ['/api/v1/agents/hire', '/api/v1/agents/workflows'].includes(new URL(request.url()).pathname)) submitted.push(request.url());
    });
    await page.goto('/agents');
    await expect(page.getByText('Text analysis is not configured. No task can be submitted.', { exact: true })).toBeVisible();
    await page.getByRole('button', { name: 'Workflows', exact: true }).click();
    const workflowName = `Branch review ${Date.now()}`;
    const task = 'Review this supplied branch summary for security regressions.';
    await page.locator('#workflow-name').fill(workflowName);
    await page.locator('#workflow-task').fill(task);
    await page.getByRole('button', { name: 'Create & Run Workflow' }).first().click();
    await expect(page.getByText('Task acceptance was not confirmed. Review the composer status before trying again.', { exact: true })).toBeVisible();
    await expect(page.locator('#workflow-name')).toHaveValue(workflowName);
    await expect(page.locator('#workflow-task')).toHaveValue(task);
    await expect(page.getByRole('heading', { name: workflowName, exact: true })).toHaveCount(0);
    await expect(page.getByText('Backend CLI', { exact: true })).toHaveCount(0);
    expect(submitted).toEqual([]);
  });

  test('a configured provider accepts and persists the supplied workflow task @provider-acceptance', async ({ page }) => {
    test.skip(!generationAcceptance, generationGateReason);
    const workflowName = `Branch review ${Date.now()}`;
    const task = 'Review this supplied summary: the change adds authenticated tenant-scoped database reads. Identify potential security regressions.';
    await page.goto('/agents');
    await expect(page.getByRole('button', { name: 'Start task', exact: true })).toBeEnabled();
    await page.getByRole('button', { name: 'Workflows', exact: true }).click();
    await page.locator('#workflow-name').fill(workflowName);
    await page.locator('#workflow-task').fill(task);
    const accepted = page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/agents/hire' && response.request().method() === 'POST');
    await page.getByRole('button', { name: 'Create & Run Workflow' }).first().click();
    const response = await accepted;
    expect(response.status()).toBe(201);
    const receipt = await response.json();
    expect(receipt.status).toBe('queued');
    expect(receipt.workflow_id).toMatch(/^[a-f0-9-]{36}$/);
    const record = await page.request.get(`/api/v1/agents/workflows/${receipt.workflow_id}`);
    expect(record.status()).toBe(200);
    expect((await record.json()).workflow).toMatchObject({ id: receipt.workflow_id, name: workflowName, task });
    await expect(page.getByRole('heading', { name: workflowName, exact: true })).toBeVisible();
  });
});
