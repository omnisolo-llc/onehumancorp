import { test, expect } from '@playwright/test';

test.describe('See AI Activity Flow', () => {

  test.beforeEach(async ({ page }) => {
    // Navigate to the login page (or home page which redirects to login)
    await page.goto('/');

    // Login
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Login")');

    // Wait for the dashboard/home page to load
    await expect(page.locator('text="Your business, live in minutes."')).toBeVisible();
  });

  test('should display See what your AI is doing and allow navigation to observability', async ({ page }) => {
    const seeAiActivity = page.locator('text="See what your AI is doing"');
    await expect(seeAiActivity.first()).toBeVisible();
    await seeAiActivity.first().click();

    // Assert the final state matches the design intent
    // Wait for observability panel to appear
    const observabilityPanel = page.locator('text="Swarm Observability"');
    await expect(observabilityPanel.first()).toBeVisible();
  });

  test('should display See what your AI is doing label on small screens', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    const seeAiActivity = page.locator('text="See what your AI is doing"');
    await expect(seeAiActivity.first()).toBeVisible();
  });

  test('should display See what your AI is doing label on medium screens', async ({ page }) => {
    await page.setViewportSize({ width: 768, height: 1024 });
    const seeAiActivity = page.locator('text="See what your AI is doing"');
    await expect(seeAiActivity.first()).toBeVisible();
  });

  test('should display See what your AI is doing label on large screens', async ({ page }) => {
    await page.setViewportSize({ width: 1440, height: 1080 });
    const seeAiActivity = page.locator('text="See what your AI is doing"');
    await expect(seeAiActivity.first()).toBeVisible();
  });

  test('should close the observability panel after opening from See what your AI is doing', async ({ page }) => {
    const seeAiActivity = page.locator('text="See what your AI is doing"');
    await expect(seeAiActivity.first()).toBeVisible();
    await seeAiActivity.first().click();

    // Check panel is visible
    const observabilityPanel = page.locator('text="Swarm Observability"');
    await expect(observabilityPanel.first()).toBeVisible();

    // Close panel
    const closeButton = page.locator('button:has-text("Close")');
    await closeButton.first().click();

    // Check panel is hidden
    await expect(observabilityPanel.first()).not.toBeVisible();
  });

});
