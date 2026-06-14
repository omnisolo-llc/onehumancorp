import { test, expect } from '@playwright/test';

test.describe('Assistant Workstation', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the assistant route
    await page.goto('/assistant');
  });

  test('should load the workstation shell correctly', async ({ page }) => {
    // Check main components
    await expect(page.locator('text=Assistant Workspace')).toBeVisible();
    await expect(page.locator('text=No tasks yet.')).toBeVisible();
    await expect(page.locator('button:has-text("Send")')).toBeVisible();
  });

  test('should allow creating a new task and interacting with it', async ({ page }) => {
    // Enter a prompt
    const promptInput = page.locator('textarea[placeholder="Type a message or prompt..."]');
    await promptInput.fill('Please help me organize my bakery orders.');

    // Click send
    await page.locator('button:has-text("Send")').click();

    // Verify task is created in the list
    await expect(page.locator('text=Please help me organize my...').first()).toBeVisible();

    // Check that the chat area updates
    await expect(page.locator('text=Please help me organize my bakery orders.').first()).toBeVisible();
  });

  test('should show correct permission profile warnings', async ({ page }) => {
    // Attempt to change permission profile
    const profileSelect = page.locator('select');
    await profileSelect.selectOption('Full Access');

    // Warning should show up
    await expect(page.locator('text=Requires elevated privileges and explicit approval for sensitive actions.')).toBeVisible();
  });

  test('should have working tabs in the results panel', async ({ page }) => {
    await page.locator('button:has-text("Artifacts")').click();
    await expect(page.locator('text=No artifacts generated yet.')).toBeVisible();

    await page.locator('button:has-text("All Files")').click();
    await expect(page.locator('text=No files in the workspace yet.')).toBeVisible();

    await page.locator('button:has-text("Changes")').click();
    await expect(page.locator('text=No pending changes.')).toBeVisible();

    await page.locator('button:has-text("Preview")').click();
    await expect(page.locator('text=No preview available.')).toBeVisible();
  });
});
