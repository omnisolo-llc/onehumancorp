import { test, expect } from '@playwright/test';

test.describe('Agent Marketplace E2E', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/agent-marketplace');
    await expect(page.locator('h1')).toHaveText('Agent Marketplace');
  });

  test('Page load and initial agents visible', async ({ page }) => {
    // Check if the search input is visible
    const searchInput = page.getByPlaceholder('Search for agents...');
    await expect(searchInput).toBeVisible();

    // Verify initial agents are loaded
    await expect(page.locator('h3').first()).toBeVisible();
    await expect(page.locator('text=Senior Rust Developer')).toBeVisible();
    await expect(page.locator('text=Technical Writer')).toBeVisible();
  });

  test('Search filters agents correctly', async ({ page }) => {
    const searchInput = page.getByPlaceholder('Search for agents...');

    // Search for a specific agent
    await searchInput.fill('Senior');

    // Wait for the search results to filter
    await expect(page.locator('text=Senior Rust Developer')).toBeVisible();
    await expect(page.locator('text=Technical Writer')).not.toBeVisible();
  });

  test('Search with no results shows empty state', async ({ page }) => {
    const searchInput = page.getByPlaceholder('Search for agents...');

    // Search for an agent that doesn't exist
    await searchInput.fill('NonexistentAgent123');
    await expect(page.locator('text=No agents found matching "NonexistentAgent123"')).toBeVisible();
  });

  test('Clear search restores original list', async ({ page }) => {
    const searchInput = page.getByPlaceholder('Search for agents...');

    // Search for a specific term
    await searchInput.fill('Senior');
    await expect(page.locator('text=Technical Writer')).not.toBeVisible();

    // Clear search and verify original agents are shown
    await searchInput.fill('');
    await expect(page.locator('text=Senior Rust Developer')).toBeVisible();
    await expect(page.locator('text=Technical Writer')).toBeVisible();
  });

  test('Install an agent shows toast notification and updates button', async ({ page }) => {
    const searchInput = page.getByPlaceholder('Search for agents...');

    // Search to isolate one agent
    await searchInput.fill('Senior Rust Developer');

    const installButton = page.getByRole('button', { name: 'Install Agent' }).first();
    await expect(installButton).toBeVisible();

    // Click install
    await installButton.click();

    // Verify the visual toast appears
    const toast = page.locator('text=Successfully installed Senior Rust Developer!');
    await expect(toast).toBeVisible();

    // Verify the button text changes to "Installed"
    const installedButton = page.getByRole('button', { name: 'Installed' }).first();
    await expect(installedButton).toBeVisible();

    // Check aria-pressed attribute
    await expect(installedButton).toHaveAttribute('aria-pressed', 'true');
  });
});
