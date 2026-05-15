import { test, expect } from '@playwright/test';

test.describe('KAIROS Orchestration Walkthrough - Grandmother Test', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate naturally from the home page
    await page.goto('/');

    // Ensure the home page has loaded
    await expect(page.locator('h1', { hasText: 'OneHumanCorp' })).toBeVisible();

    // Use the actual navigation anchor
    await page.locator('a#nav-walkthrough').click();

    // Wait for navigation
    await page.waitForURL('**/walkthroughs/kairos_orchestration');
  });

  test('should display the main header', async ({ page }) => {
    const header = page.locator('h1', { hasText: 'KAIROS Walkthrough' });
    await expect(header).toBeVisible();

    const desc = page.locator('header p');
    await expect(desc).toContainText('Interactive Documentation & System Architecture Explorer');
  });

  test('should display the Triad Architecture section', async ({ page }) => {
    const section = page.locator('#architecture');
    await expect(section).toBeVisible();
    await expect(section.locator('h2')).toContainText('1. The KAIROS Triad Architecture');

    // Check if the SVG diagram is present
    const svg = section.locator('svg');
    await expect(svg).toBeVisible();
    await expect(section).toContainText('KAIROS Orchestrator');
  });

  test('should allow interaction with the API Explorer tabs', async ({ page }) => {
    const section = page.locator('#api');
    await expect(section).toBeVisible();

    // Click the first API tab
    const tab1 = section.locator('div', { hasText: '/api/v1/kairos/mesh/health' }).first();
    await tab1.click();

    // Verify the description updates
    await expect(section).toContainText('Retrieves the health status of all connected Swarm agents');

    // Click the second tab (Consolidate endpoint)
    const tab2 = section.locator('div', { hasText: '/api/v1/kairos/memory/consolidate' }).first();
    await tab2.click();

    // Verify the description updates
    await expect(section).toContainText('Forces an AutoDream consolidation cycle immediately');
  });

  test('should display the Event Timeline correctly', async ({ page }) => {
    const section = page.locator('#timeline');
    await expect(section).toBeVisible();
    await expect(section.locator('h2')).toContainText('3. Distributed Event Timeline');

    await expect(section).toContainText('Lock acquired');
    await expect(section).toContainText('AutoDream vector generated');
  });

  test('should have a working back button that returns to home', async ({ page }) => {
    const backBtn = page.locator('a', { hasText: 'Back to Home' });
    await backBtn.click();
    await page.waitForURL('**/');
    await expect(page.locator('h1', { hasText: 'OneHumanCorp' })).toBeVisible();
  });
});
