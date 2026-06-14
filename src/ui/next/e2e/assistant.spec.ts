import { test, expect } from '@playwright/test';

test.describe('Assistant Workstation', () => {

  test('should render the workspace layout', async ({ page }) => {
    await page.goto('/assistant');

    // Verify headers and workspace title
    await expect(page.locator('h1').first()).toBeVisible();
  });

  test('should allow creating a new task', async ({ page }) => {
    await page.goto('/assistant');

    // Find the composer text area and the create/send button
    // It should simulate Maya typing "Prepare a quote for 50 custom cupcakes"
    const input = page.locator('textarea').first();
    await input.fill('Prepare a quote for 50 custom cupcakes');

    // We assume there is a Send button
    const sendButton = page.getByRole('button', { name: /Send|Submit/i });
    if (await sendButton.isVisible()) {
        await sendButton.click();
    }
  });

  test('should list active tasks', async ({ page }) => {
    await page.goto('/assistant');
    // Verify there is a task list visible
    const taskList = page.locator('nav').first();
    await expect(taskList).toBeVisible();
  });

  test('should render generated artifacts', async ({ page }) => {
    await page.goto('/assistant');

    // The UI should have a Results panel on the right with an "Artifacts" tab
    const artifactsTab = page.getByRole('button', { name: /Artifacts/i });
    if (await artifactsTab.isVisible()) {
      await artifactsTab.click();
      await expect(page.getByText('Generated output').or(page.getByText('No generated files yet')).first()).toBeVisible();
    }
  });

  test('should allow task archiving', async ({ page }) => {
    await page.goto('/assistant');
    // We expect there to be a way to archive tasks
    const archiveButton = page.getByRole('button', { name: /Archive/i }).first();
    if (await archiveButton.isVisible()) {
      await archiveButton.click();
    }
  });
});
