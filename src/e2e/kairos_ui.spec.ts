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
  await expect(page.getByText('842.5 MB')).toBeVisible();
});

test('walkthrough tooltips appear', async ({ page }) => {
  // If the e2e tests are running, walkthroughs are disabled to prevent flake
  // We can skip this test if we detect the E2E flag
  test.skip(process.env.NEXT_PUBLIC_E2E === 'true', 'Walkthroughs disabled in E2E mode');

  await page.goto('/kairos?walkthrough=true');

  // The walkthrough has a 1 second delay
  await page.waitForTimeout(1500);

  // The walkthrough should show a tooltip
  await expect(page.getByText("The Shared Task List is the 'Brain'")).toBeVisible();
});

test('UI uses correct glassmorphism styling and responsiveness classes', async ({ page }) => {
  await page.goto('/kairos');

  // Check the main container has the new responsive padding classes applied
  const main = page.locator('main');
  await expect(main).toHaveClass(/p-4/);
  await expect(main).toHaveClass(/sm:p-6/);
  await expect(main).toHaveClass(/md:p-8/);

  // Check the brain section has the specific ohc-hybrid-panel glassmorphism class
  // and hover states applied
  const brainPanel = page.locator('#kairos-brain > div').first();
  await expect(brainPanel).toHaveClass(/ohc-hybrid-panel/);
  await expect(brainPanel).toHaveClass(/hover:-translate-y-1/);
});
