import { test, expect } from '@playwright/test';

test.describe('Agent Jobs DB Sync Parity CUJ', () => {
  // Test 1: Simulating Task Creation to Verify No Timeout Failures
  test('verify owner can create a task successfully and UI reflects correct state', async ({ page }) => {
    await page.goto('/tasks');
    await expect(page.locator('text=Tasks')).toBeVisible();

    await page.getByRole('button', { name: 'New Task' }).click();
    await page.getByLabel('Title').fill('Task Parity E2E Test');
    await page.getByRole('button', { name: 'Save' }).click();

    // Verify task is successfully created
    await expect(page.locator('text=Task Parity E2E Test')).toBeVisible({ timeout: 10000 });
  });

  // Test 2: Simulating Empty Form Submission and Empty String handling
  test('verify empty task title handles null correctly', async ({ page }) => {
    await page.goto('/tasks');
    await expect(page.locator('text=Tasks')).toBeVisible();

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

    await expect(page.locator('text=Edited Task')).toBeVisible({ timeout: 10000 });
  });

  // Test 4: Delete Task (Database Delete Action)
  test('verify owner can delete a task and handle degradation gracefully', async ({ page }) => {
    await page.goto('/tasks');

    await page.getByRole('button', { name: 'New Task' }).click();
    await page.getByLabel('Title').fill('Delete Me Task');
    await page.getByRole('button', { name: 'Save' }).click();

    await page.locator('text=Delete Me Task').click();
    await page.getByRole('button', { name: 'Delete' }).click();

    // UI should reflect successful removal
    await expect(page.locator('text=Delete Me Task')).not.toBeVisible();
  });

  // Test 5: Verify task list rendering
  test('verify task list loads reliably from DB under load', async ({ page }) => {
    await page.goto('/tasks');

    for (let i = 0; i < 3; i++) {
        await page.getByRole('button', { name: 'New Task' }).click();
        await page.getByLabel('Title').fill(`Task Stress ${i}`);
        await page.getByRole('button', { name: 'Save' }).click();
    }

    await page.goto('/'); // force reload
    await page.goto('/tasks');

    for (let i = 0; i < 3; i++) {
        await expect(page.locator(`text=Task Stress ${i}`)).toBeVisible({ timeout: 10000 });
    }
  });
});
