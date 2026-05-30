import { test, expect } from '@playwright/test';

test.describe('Autonomous Multi-Location Topology', () => {
  test('Priya deploys a new location from the dashboard', async ({ page }) => {
    // Navigate to the dashboard
    await page.goto('http://localhost:3000/dashboard');

    // Wait for the dashboard to load and verify "Empire View" is the default
    await expect(page.locator('text=Empire View').first()).toBeVisible();

    // Open the location switcher
    await page.getByRole('button', { name: /Empire View/ }).click();

    // Click "Add Location"
    await page.getByRole('button', { name: /Add Location/ }).click();

    // Verify the "Deploy New Location" modal is visible
    await expect(page.locator('text=Deploy New Location')).toBeVisible();

    // Verify the configuration toggles
    await expect(page.locator('text=Clone Catalog & Pricing')).toBeVisible();
    await expect(page.locator('text=Share Current Staff')).toBeVisible();
    await expect(page.locator('text=Setup Local Tax Profile')).toBeVisible();

    // Click "Launch Location"
    await page.getByRole('button', { name: /Launch Location/ }).click();

    // Verify the modal closes and the active location updates to "5th Ave"
    await expect(page.locator('text=Deploy New Location')).not.toBeVisible();
    await expect(page.getByRole('button', { name: /5th Ave/ }).first()).toBeVisible();

    // Open the location switcher again to check if it's in the list
    await page.getByRole('button', { name: /5th Ave/ }).first().click();
    await expect(page.locator('button', { hasText: '5th Ave' }).first()).toBeVisible();
  });
});
