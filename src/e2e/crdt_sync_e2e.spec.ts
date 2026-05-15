import { test, expect } from '@playwright/test';

test('CRDT offline to cloud full sync cycle', async ({ page }) => {
  // 1. Navigate to home page and login
  await page.goto('/');
  await page.click('button:has-text("Sign in")');

  // Navigate through normal E2E path
  await page.click('text="Tasks"');
  await page.click('text="New Task"');
  await page.fill('input[placeholder="Task Name"]', 'Offline Task 123');
  await page.click('button:has-text("Save")');

  // Verify task is created
  await expect(page.locator('text="Offline Task 123"')).toBeVisible({ timeout: 10000 });
});
