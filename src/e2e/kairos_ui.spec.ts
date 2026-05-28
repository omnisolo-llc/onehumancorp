import { test, expect } from '@playwright/test';

test('navigation to kairos works', async ({ page }) => {
  await page.goto('/dashboard');

  // Click on the KAIROS link
  const kairosLink = page.getByRole('link', { name: /⚡️ KAIROS/i });
  await kairosLink.click();

  // Verify KAIROS dashboard loaded
  await expect(page.getByRole('heading', { name: 'KAIROS Orchestration' })).toBeVisible();
});

test('shared task list loads', async ({ page }) => {
  await page.goto('/kairos');

  await expect(page.getByRole('heading', { name: 'Shared Task List' })).toBeVisible();
  await expect(page.getByText('Inventory Reorder Strategy')).toBeVisible();
});

test('teammate mesh nodes are visible', async ({ page }) => {
  await page.goto('/kairos');

  await expect(page.getByRole('heading', { name: 'Teammate Mesh' })).toBeVisible();

  // Checking node types are present
  await expect(page.getByText('Brain', { exact: true })).toBeVisible();
  await expect(page.getByText('Nerve', { exact: true })).toBeVisible();
  await expect(page.getByText('Memory', { exact: true })).toBeVisible();
});

test('autodream memory stats', async ({ page }) => {
  await page.goto('/kairos');

  await expect(page.getByRole('heading', { name: 'AutoDream Memory' })).toBeVisible();
  await expect(page.getByText('Infinite Context')).toBeVisible();

  // Assert loading skeleton is visible initially
  await expect(page.getByTestId('memory-density-skeleton')).toBeVisible();
  await expect(page.getByTestId('memory-clusters-skeleton')).toBeVisible();

  // Wait for the mock data to load (2 seconds + margin)
  await expect(page.getByTestId('memory-density')).toBeVisible({ timeout: 5000 });
  await expect(page.getByTestId('memory-clusters')).toBeVisible({ timeout: 5000 });

  // Verify the new stats are displayed
  await expect(page.getByTestId('memory-density')).toHaveText('845.2 MB');
  await expect(page.getByTestId('memory-clusters')).toHaveText('14 Active');
});

test('walkthrough tooltips appear', async ({ page }) => {
  await page.goto('/kairos?walkthrough=true');

  // The walkthrough has a 1 second delay
  await page.waitForTimeout(1500);

  // The walkthrough should show a tooltip
  await expect(page.getByText("The Shared Task List is the 'Brain'")).toBeVisible();
});
