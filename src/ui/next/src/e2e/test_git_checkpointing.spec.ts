import { test, expect } from '../../../../e2e/fixtures';
import { runtimeAcceptance, runtimeGateReason } from '../../../../e2e/generation-acceptance';

const protocolPath = '/api/v1/agents/protocol';

test.describe('Agent Protocol Git Checkpointing', () => {
  test('unconfigured runtime does not present unread task or checkpoint history as empty', async ({ page, unlimitedAdminUser, loginAs }) => {
    test.skip(runtimeAcceptance, 'This contract requires an unconfigured workspace runtime.');
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/agent-protocol');
    await expect(page.getByRole('heading', { name: 'Agent Protocol UI', exact: true })).toBeVisible();
    await expect(page.getByText('No tasks found.', { exact: true })).toHaveCount(0);
    const pending = page.waitForResponse(response => {
      const url = new URL(response.url());
      return url.pathname === protocolPath && url.searchParams.get('method') === 'ap_list_tasks'
        && response.request().method() === 'GET';
    });
    await page.getByRole('button', { name: 'Load workspace runtime tasks' }).click();
    const response = await pending;
    expect(response.status()).toBe(503);
    expect(await response.json()).toEqual({ error: 'Agent runtime is not configured; no work was dispatched' });
    await expect(page.getByRole('alert').filter({ hasText: 'Workspace runtime task history could not be verified.' })).toHaveText('Workspace runtime task history could not be verified.');
    await expect(page.getByText('No tasks found.', { exact: true })).toHaveCount(0);
    await expect(page.getByText('No checkpoints saved.', { exact: true })).toHaveCount(0);
    await expect(page.getByRole('button', { name: 'Restore Checkpoint' })).toHaveCount(0);
  });

  test('creates a task, executes a step, and restores an actual checkpoint @runtime-acceptance', async ({ page, unlimitedAdminUser, loginAs }) => {
    test.skip(!runtimeAcceptance, runtimeGateReason);
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/agent-protocol');
    await expect(page.getByRole('heading', { name: 'Agent Protocol UI', exact: true })).toBeVisible();
    const taskInput = `Real E2E Checkpointing Test ${crypto.randomUUID()}`;
    await page.getByPlaceholder('New Task Input...').fill(taskInput);
    const created = page.waitForResponse(response => new URL(response.url()).pathname === protocolPath
      && response.request().method() === 'POST' && response.request().postDataJSON()?.method === 'ap_create_task');
    await page.getByRole('button', { name: 'Create', exact: true }).click();
    const creation = await created;
    expect(creation.status()).toBe(200);
    const task = await creation.json();
    expect(task.error).toBeUndefined();
    expect(task.task_id).toEqual(expect.any(String));
    expect(task.task_id.trim()).not.toBe('');
    expect(task.input).toBe(taskInput);
    const taskItem = page.locator('li').filter({ hasText: task.task_id });
    await expect(taskItem).toHaveCount(1);
    await expect(taskItem).toContainText(taskInput);
    await taskItem.click();

    await page.getByPlaceholder('Optional Step Input...').fill('Please execute step to generate a checkpoint');
    const executed = page.waitForResponse(response => new URL(response.url()).pathname === protocolPath
      && response.request().method() === 'POST' && response.request().postDataJSON()?.method === 'ap_execute_step');
    await page.getByRole('button', { name: 'Execute Step' }).click();
    const execution = await executed;
    expect(execution.status()).toBe(200);
    const step = await execution.json();
    expect(step.error).toBeUndefined();
    expect(step).toMatchObject({ task_id: task.task_id, status: 'completed' });
    expect(step.output).toEqual(expect.any(String));
    expect(step.output.trim()).not.toBe('');
    await expect(page.getByText('completed', { exact: true })).toBeVisible();
    await expect(page.getByText(step.output, { exact: true })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'State Checkpoints' })).toBeVisible();

    // A configured acceptance run must actually produce and restore a checkpoint.
    // An empty list or disabled checkpointer cannot satisfy this positive gate.
    const restore = page.getByRole('button', { name: 'Restore Checkpoint' }).first();
    await expect(restore).toBeEnabled({ timeout: 30_000 });
    await expect(page.getByText('No checkpoints saved.', { exact: true })).toHaveCount(0);
    const restored = page.waitForResponse(response => new URL(response.url()).pathname === protocolPath
      && response.request().method() === 'POST' && response.request().postDataJSON()?.method === 'ap_restore_checkpoint');
    await restore.click();
    const restoration = await restored;
    const submitted = restoration.request().postDataJSON();
    expect(submitted.params.task_id).toBe(task.task_id);
    expect(submitted.params.checkpoint_id).toEqual(expect.any(String));
    expect(submitted.params.checkpoint_id.trim()).not.toBe('');
    expect(restoration.status()).toBe(200);
    expect(await restoration.json()).toMatchObject({ success: true });
    await expect(page.locator('div.bg-red-100')).toHaveCount(0);
  });
});
