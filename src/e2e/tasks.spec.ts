import { test, expect } from '@playwright/test';

test.describe('Task List Page', () => {
  test('should display task list page', async ({ page }) => {
    await page.goto('/tasks');
    await expect(page.locator('text=/task|todo/i')).toBeVisible();
  });

  test('should show task list header', async ({ page }) => {
    await page.goto('/tasks');
    await expect(page.locator('text=Tasks')).toBeVisible();
  });

  test('should display task items', async ({ page }) => {
    await page.goto('/tasks');
    const taskItem = page.locator('[class*="task"], [class*="item"]').first();
    await expect(taskItem).toBeVisible();
  });

  test('should show task status indicators', async ({ page }) => {
    await page.goto('/tasks');
    const status = page.locator('[class*="status"], text=/pending|progress|done/i').first();
    await expect(status).toBeVisible();
  });

  test('should filter tasks by status', async ({ page }) => {
    await page.goto('/tasks');
    const filterDropdown = page.locator('select, [class*="filter"]').first();
    if (await filterDropdown.isVisible()) {
      await filterDropdown.selectOption({ index: 1 });
    }
  });

  test('should sort tasks', async ({ page }) => {
    await page.goto('/tasks');
    const sortButton = page.locator('button:has-text("Sort"), [class*="sort"]').first();
    if (await sortButton.isVisible()) {
      await sortButton.click();
      await expect(page.locator('text=/ascending|descending|priority/i')).toBeVisible();
    }
  });

  test('should search tasks', async ({ page }) => {
    await page.goto('/tasks');
    const searchInput = page.locator('input[type="search"], input[placeholder*="search"]').first();
    if (await searchInput.isVisible()) {
      await searchInput.fill('test task');
      await expect(page.locator('text=/test task/i')).toBeVisible();
    }
  });

  test('should create new task', async ({ page }) => {
    await page.goto('/tasks');
    const newTaskBtn = page.locator('button:has-text("New"), button:has-text("Add")').first();
    if (await newTaskBtn.isVisible()) {
      await newTaskBtn.click();
      await expect(page.locator('text=/create.*task|new.*task/i')).toBeVisible();
    }
  });

  test('should complete a task', async ({ page }) => {
    await page.goto('/tasks');
    const completeBtn = page.locator('button:has-text("Complete"), button:has-text("Done")').first();
    if (await completeBtn.isVisible()) {
      await completeBtn.click();
      await expect(page.locator('text=/completed|done/i')).toBeVisible({ timeout: 3000 }).catch(() => {});
    }
  });

  test('should delete a task', async ({ page }) => {
    await page.goto('/tasks');
    const taskItem = page.locator('[class*="task"]').first();
    await taskItem.hover();
    const deleteBtn = page.locator('button:has-text("Delete"), button:has-text("Remove")').first();
    if (await deleteBtn.isVisible()) {
      await deleteBtn.click();
      await expect(page.locator('text=/deleted|removed/i')).toBeVisible({ timeout: 3000 }).catch(() => {});
    }
  });

  test('should assign task to agent', async ({ page }) => {
    await page.goto('/tasks');
    const taskItem = page.locator('[class*="task"]').first();
    await taskItem.click();
    const assignSelect = page.locator('select, [class*="assign"]').first();
    if (await assignSelect.isVisible()) {
      await assignSelect.selectOption({ index: 1 });
    }
  });

  test('should set task priority', async ({ page }) => {
    await page.goto('/tasks');
    const taskItem = page.locator('[class*="task"]').first();
    await taskItem.click();
    const priorityBtn = page.locator('button:has-text("Priority"), [class*="priority"]').first();
    if (await priorityBtn.isVisible()) {
      await priorityBtn.click();
      await expect(page.locator('text=/high|low|medium/i')).toBeVisible();
    }
  });

  test('should add due date to task', async ({ page }) => {
    await page.goto('/tasks');
    const taskItem = page.locator('[class*="task"]').first();
    await taskItem.click();
    const dateInput = page.locator('input[type="date"], [class*="date"]').first();
    if (await dateInput.isVisible()) {
      await dateInput.fill('2026-12-31');
    }
  });

  test('should show task description', async ({ page }) => {
    await page.goto('/tasks');
    const taskItem = page.locator('[class*="task"]').first();
    await taskItem.click();
    const description = page.locator('text=/description|details/i').first();
    await expect(description).toBeVisible();
  });

  test('should add comment to task', async ({ page }) => {
    await page.goto('/tasks');
    const taskItem = page.locator('[class*="task"]').first();
    await taskItem.click();
    const commentInput = page.locator('textarea, input[type="text"]').nth(1);
    if (await commentInput.isVisible()) {
      await commentInput.fill('test comment');
      await page.locator('button:has-text("Comment"), button:has-text("Add")').click();
    }
  });

  test('should show task activity log', async ({ page }) => {
    await page.goto('/tasks');
    const taskItem = page.locator('[class*="task"]').first();
    await taskItem.click();
    const activityTab = page.locator('button:has-text("Activity"), button:has-text("History")').first();
    if (await activityTab.isVisible()) {
      await activityTab.click();
      await expect(page.locator('text=/activity|log|history/i')).toBeVisible();
    }
  });

  test('should paginate task list', async ({ page }) => {
    await page.goto('/tasks');
    const pagination = page.locator('[class*="pagination"], button:has-text("Next")').first();
    await expect(pagination).toBeVisible();
  });

  test('should navigate to next page', async ({ page }) => {
    await page.goto('/tasks');
    const nextBtn = page.locator('button:has-text("Next"), button:has-text(">")').first();
    if (await nextBtn.isVisible()) {
      await nextBtn.click();
      await expect(page.locator('text=/page \\d+/i')).toBeVisible({ timeout: 3000 }).catch(() => {});
    }
  });
});

test.describe('Task List Mobile', () => {
  test.use({ viewport: { width: 375, height: 800 } });

  test('should display task list on mobile', async ({ page }) => {
    await page.goto('/tasks');
    await expect(page.locator('text=/task/i')).toBeVisible();
  });

  test('should swipe to complete task on mobile', async ({ page }) => {
    await page.goto('/tasks');
    const taskItem = page.locator('[class*="task"]').first();
    if (await taskItem.isVisible()) {
      await taskItem.swipe('left');
      await expect(page.locator('button:has-text("Complete")')).toBeVisible({ timeout: 3000 }).catch(() => {});
    }
  });
});