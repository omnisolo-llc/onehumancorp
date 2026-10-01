import { randomUUID } from 'node:crypto';
import { test, expect } from '@playwright/test';

test.describe('Agent Jobs DB Sync Parity CUJ', () => {
  // Test 1: Simulating Task Creation to Verify No Timeout Failures
  test('verify owner can create a task successfully and UI reflects correct state', async ({ page }) => {
    await page.goto('/tasks');
    await expect(page.locator('text=Tasks').first()).toBeVisible();

    await page.getByRole('button', { name: 'New Task' }).click();
    await page.getByLabel('Title').fill('Task Parity E2E Test');
    await page.getByRole('button', { name: 'Save' }).click();

    // Verify task is successfully created
    await expect(page.locator('text=Task Parity E2E Test')).toBeVisible({ timeout: 10000 });
  });

  // Test 2: Simulating Empty Form Submission and Empty String handling
  test('verify empty task title handles null correctly', async ({ page }) => {
    await page.goto('/tasks');
    await expect(page.locator('text=Tasks').first()).toBeVisible();

    await page.getByRole('button', { name: 'New Task' }).click();
    await page.getByRole('button', { name: 'Save' }).click();

    await expect(page.locator('text=Title is required')).toBeVisible({ timeout: 10000 });
  });

  // Test 3: Edit Task (Database Update Action)
  test('verify owner can edit a task and sync changes properly', async ({ page }) => {
    await page.goto('/tasks');

    await page.getByRole('button', { name: 'New Task' }).click();
    await page.getByLabel('Title').fill('Edit Me Task');
    await page.getByRole('button', { name: 'Save' }).click();

    await page.locator('text=Edit Me Task').click();
    await page.getByRole('button', { name: 'Edit' }).click();
    await page.getByLabel('Title').fill('Edited Task');
    await page.getByRole('button', { name: 'Save Changes' }).click();

    await expect(page.locator('#task-list').getByText('Edited Task', { exact: true })).toBeVisible({ timeout: 10000 });
    await page.reload();
    await expect(page.locator('#task-list').getByText('Edited Task', { exact: true })).toBeVisible();
  });

  // Test 4: Delete Task (Database Delete Action)
  test('verify owner can delete a task and handle degradation gracefully', async ({ page }) => {
    const title = `Delete Me Task ${randomUUID()}`;
    await page.goto('/tasks');

    await page.getByRole('button', { name: 'New Task' }).click();
    await page.getByLabel('Title').fill(title);
    await page.getByRole('button', { name: 'Save' }).click();

    await page.locator('#task-list').getByText(title, { exact: true }).click();
    const deletion = page.waitForResponse(response => new URL(response.url()).pathname.startsWith('/api/v1/staff/tasks/')
      && response.request().method() === 'DELETE');
    await page.getByRole('button', { name: 'Delete' }).click();

    const deleted = await deletion;
    expect(deleted.ok(), await deleted.text()).toBe(true);
    // Both the selected detail and the list row must be removed after persistence.
    await expect(page.getByText(title, { exact: true })).toHaveCount(0);
    await page.reload();
    await expect(page.getByText('Loading tasks...', { exact: true })).not.toBeVisible();
    await expect(page.locator('#task-list').getByText(title, { exact: true })).toHaveCount(0);
  });

  // Test 5: Verify task list rendering
  test('verify task list loads reliably from DB under load', async ({ page }) => {
    await page.goto('/tasks');

    for (let i = 0; i < 3; i++) {
        await page.getByRole('button', { name: 'New Task' }).click();
        await page.getByLabel('Title').fill(`Task Stress ${i}`);
        await page.getByRole('button', { name: 'Save' }).click();
        await expect(page.locator('#task-list').getByText(`Task Stress ${i}`, { exact: true })).toBeVisible();
    }

    await page.goto('/'); // force reload
    await page.goto('/tasks');

    for (let i = 0; i < 3; i++) {
        await expect(page.locator(`text=Task Stress ${i}`)).toBeVisible({ timeout: 10000 });
    }
  });
});
