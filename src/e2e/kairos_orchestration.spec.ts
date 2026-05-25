import { test, expect } from './fixtures';

test.describe('KAIROS AI OS Orchestration Dashboard (Phase 4)', () => {

  test('Test 1: Verify the dashboard loads and displays the Dashboard heading', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('Test 2: Navigate to /kairos page and verify the KAIROS Orchestration heading', async ({ page }) => {
    await page.goto('/kairos');
    await expect(page.getByRole('heading', { name: 'KAIROS Orchestration' })).toBeVisible();
  });

  test('Test 3: Verify the Shared Task List (The Brain) section is displayed', async ({ page }) => {
    await page.goto('/kairos');
    await expect(page.getByRole('heading', { name: 'Shared Task List' })).toBeVisible();
    await expect(page.getByText('THE BRAIN', { exact: false })).toBeVisible();
  });

  test('Test 4: Verify the Teammate Mesh (The Nerves) section is displayed', async ({ page }) => {
    await page.goto('/kairos');
    await expect(page.getByRole('heading', { name: 'Teammate Mesh' })).toBeVisible();
    await expect(page.getByText('THE NERVES', { exact: false })).toBeVisible();
  });

  test('Test 5: Verify the AutoDream Memory section is displayed', async ({ page }) => {
    await page.goto('/kairos');
    await expect(page.getByRole('heading', { name: 'AutoDream Memory' })).toBeVisible();
    await expect(page.getByText('THE MEMORY', { exact: false })).toBeVisible();
  });

  test('Test 6: Verify the hover shimmer effect class exists on task items', async ({ page }) => {
    await page.goto('/kairos');
    // Ensure task items are rendered with the group class
    const tasks = page.locator('#kairos-brain .group');
    await expect(tasks.first()).toBeVisible();
    // Hover over the first task
    await tasks.first().hover();
    // Validate the shimmer child element exists
    const shimmerElement = tasks.first().locator('.group-hover\\:animate-\\[shimmer_1\\.5s_infinite\\]');
    await expect(shimmerElement).toBeAttached();
  });
});
