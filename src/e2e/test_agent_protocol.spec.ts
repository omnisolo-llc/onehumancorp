import { test, expect } from './fixtures';
import { runtimeAcceptance, runtimeGateReason } from './generation-acceptance';

const protocolPath = '/api/v1/agents/protocol';

test('unconfigured Agent Protocol creation retains the input without claiming a task', async ({ page, unlimitedAdminUser, loginAs }) => {
  test.skip(runtimeAcceptance, 'This contract requires an unconfigured workspace runtime.');
  await loginAs(page, unlimitedAdminUser);
  await page.goto('/agent-protocol');
  await expect(page.getByRole('heading', { name: 'Agent Protocol UI', exact: true })).toBeVisible();
  const taskInput = 'Test Agent Protocol Task';
  await page.getByPlaceholder('New Task Input...').fill(taskInput);
  const pending = page.waitForResponse(response => new URL(response.url()).pathname === protocolPath
    && response.request().method() === 'POST');
  await page.getByRole('button', { name: 'Create', exact: true }).click();
  const response = await pending;
  expect(response.request().postDataJSON()).toEqual({ method: 'ap_create_task', params: { input: taskInput } });
  expect(response.status()).toBe(503);
  expect(await response.json()).toEqual({ error: 'Agent runtime is not configured; no work was dispatched' });
  await expect(page.getByPlaceholder('New Task Input...')).toHaveValue(taskInput);
  await expect(page.getByText('Failed to create task', { exact: true })).toBeVisible();
  await expect(page.getByRole('region', { name: 'Unconfirmed task creation' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Create', exact: true })).toBeDisabled();
  await expect(page.locator('li').filter({ hasText: taskInput })).toHaveCount(0);
  await expect(page.getByRole('button', { name: 'Execute Step' })).toHaveCount(0);
  await expect(page.getByRole('button', { name: 'Restore Checkpoint' })).toHaveCount(0);
});

test('Agent Protocol creates a recorded task through an authorized runtime @runtime-acceptance', async ({ page, unlimitedAdminUser, loginAs }) => {
  test.skip(!runtimeAcceptance, runtimeGateReason);
  await loginAs(page, unlimitedAdminUser);
  await page.goto('/agent-protocol');
  await expect(page.getByRole('heading', { name: 'Agent Protocol UI', exact: true })).toBeVisible();
  const taskInput = `Test Agent Protocol Task ${crypto.randomUUID()}`;
  await page.getByPlaceholder('New Task Input...').fill(taskInput);
  const pending = page.waitForResponse(response => new URL(response.url()).pathname === protocolPath
    && response.request().method() === 'POST' && response.request().postDataJSON()?.method === 'ap_create_task');
  await page.getByRole('button', { name: 'Create', exact: true }).click();
  const response = await pending;
  expect(response.status()).toBe(200);
  const task = await response.json();
  expect(task.error).toBeUndefined();
  expect(task.task_id).toEqual(expect.any(String));
  expect(task.task_id.trim()).not.toBe('');
  expect(task.input).toBe(taskInput);
  const row = page.locator('li').filter({ hasText: task.task_id });
  await expect(row).toHaveCount(1);
  await expect(row).toContainText(taskInput);
  await expect(page.getByPlaceholder('New Task Input...')).toBeEmpty();
});
