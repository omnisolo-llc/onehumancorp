import { test, expect } from '@playwright/test';

test('navigation to kairos works', async ({ page }) => {
  await page.goto('/dashboard');

  // Click on the KAIROS link
  const kairosLink = page.getByRole('link', { name: /KAIROS/i });
  await kairosLink.click();

  // Verify KAIROS dashboard loaded
  await expect(page.getByRole('heading', { name: 'KAIROS Orchestration' })).toBeVisible();
});

test('shared task list loads', async ({ page }) => {
  await page.goto('/kairos');

  await expect(page.getByRole('heading', { name: 'Shared Task List' })).toBeVisible({ timeout: 15000 });
  await expect(page.getByText('Inventory Reorder Strategy')).toBeVisible({ timeout: 15000 });
});

test('teammate mesh nodes are visible', async ({ page }) => {
  await page.goto('/kairos');

  await expect(page.getByRole('heading', { name: 'Teammate Mesh' })).toBeVisible({ timeout: 15000 });

  // Checking node types are present
  await expect(page.locator('#kairos-nerves').getByText('Brain', { exact: true })).toBeVisible({ timeout: 15000 });
  await expect(page.locator('#kairos-nerves').getByText('Nerve', { exact: true })).toBeVisible({ timeout: 15000 });
  await expect(page.locator('#kairos-nerves').getByText('Memory', { exact: true })).toBeVisible({ timeout: 15000 });
});

test('autodream memory stats', async ({ page }) => {
  await page.goto('/kairos');

  await expect(page.getByRole('heading', { name: 'AutoDream Memory' })).toBeVisible({ timeout: 15000 });
  await expect(page.getByText('Infinite Context')).toBeVisible({ timeout: 15000 });
  await expect(page.getByText('842.5 MB')).toBeVisible({ timeout: 15000 });
});

test('walkthrough tooltips appear', async ({ page }) => {
  await page.goto('/kairos?walkthrough=true');

  // The walkthrough has a 1 second delay
  await page.waitForTimeout(1500);

  // The walkthrough should show a tooltip
  await expect(page.getByText("The Shared Task List is the 'Brain'")).toBeVisible({ timeout: 15000 });
});
