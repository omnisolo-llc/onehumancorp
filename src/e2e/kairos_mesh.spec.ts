import { test, expect } from '@playwright/test';
import { loginAsTestUser } from './fixtures';

test.describe('KAIROS Teammate Mesh', () => {
  test('should display mesh nodes with expected DOM structure', async ({ page }) => {
    // Navigate from home exactly as a real user would
    await loginAsTestUser(page);

    // Click Kairos in the sidebar
    await page.getByRole('link', { name: /kairos/i }).click();

    // Wait for the Teammate Mesh section to be visible
    const meshSection = page.locator('#kairos-nerves');
    await expect(meshSection).toBeVisible();

    // Verify the title
    await expect(meshSection.locator('h2')).toHaveText('Teammate Mesh');

    // Verify mesh nodes are displayed using the known static data from the UI state
    const nodes = meshSection.locator('.grid > div');
    await expect(nodes).toHaveCount(3);

    // Verify node content correctly reflecting the mock state in the code
    const node1 = nodes.nth(0);
    await expect(node1).toContainText('Brain');
    await expect(node1).toContainText('Online');
    await expect(node1).toContainText('12%');

    const node2 = nodes.nth(1);
    await expect(node2).toContainText('Nerve');
    await expect(node2).toContainText('Online');
    await expect(node2).toContainText('45%');

    const node3 = nodes.nth(2);
    await expect(node3).toContainText('Memory');
    await expect(node3).toContainText('Online');
    await expect(node3).toContainText('8%');
  });

  test('should display walkthrough tooltips when walkthrough query param is true', async ({ page }) => {
    await loginAsTestUser(page);

    // Navigate to the KAIROS dashboard with walkthrough query param
    await page.goto('/kairos?walkthrough=true');

    // The walkthrough adds a tooltip
    const tooltip = page.locator('.walkthrough-tooltip');

    // Check if walkthrough tooltip appears
    // We only wait for it, since it uses a timeout
    await expect(tooltip).toBeVisible({ timeout: 5000 });
  });
});
