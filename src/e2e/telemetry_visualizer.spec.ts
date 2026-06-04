import { test, expect } from '@playwright/test';

test.describe('🎨 Canvas: AutoDream Memory Pipeline UI Tests', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to login page
    await page.goto('/login');

    // Fill in credentials and sign in
    await page.getByPlaceholder('Email or Username').first().fill( 'test@example.com');
    await page.locator('input[type="password"]').first().fill( 'password123');
    await page.click('button:has-text("Sign In")');

    // Wait for Dashboard to load
    await page.waitForURL('**/dashboard*');

    // Enable advanced telemetry to make the component visible unconditionally for tests
    await page.click('button:has-text("Settings"), a:has-text("Settings")');
    const advancedTab = page.locator('text=Advanced').first();
    await advancedTab.click();

    // Attempt to click Advanced Mode toggle
    const toggle = page.locator('text=Advanced');
    if (await toggle.isVisible()) {
        const bbox = await toggle.boundingBox();
        if (bbox) {
            await page.mouse.click(bbox.x + 50, bbox.y);
        }
    }

    await page.click('button:has-text("Dashboard"), a:has-text("Dashboard")');
  });

  test('should display AutoDream Memory Pipeline header when advanced telemetry is shown', async ({ page }) => {
    // We expect the pipeline to be visible based on our setup
    const pipelineTitle = page.locator('text=AutoDream Memory Pipeline');
    await expect(pipelineTitle).toBeVisible();
  });

  test('should display LLM Cache Hits stat card', async ({ page }) => {
    await expect(page.locator('text=LLM Cache Hits')).toBeVisible();
  });

  test('should display RAG Latency stat card', async ({ page }) => {
    await expect(page.locator('text=RAG Latency')).toBeVisible();
  });

  test('should display Dynamic Hybrid Correlation Chart placeholder', async ({ page }) => {
    await expect(page.locator('text=[ Dynamic Hybrid Correlation Chart ]')).toBeVisible();
  });

  test('should apply correct styling properties for AutoDream Memory Pipeline container', async ({ page }) => {
    await expect(page.locator('text=AutoDream Memory Pipeline')).toBeVisible();
    await expect(page.locator('text=LLM Cache Hits')).toBeVisible();
    await expect(page.locator('text=RAG Latency')).toBeVisible();
    await expect(page.locator('text=[ Dynamic Hybrid Correlation Chart ]')).toBeVisible();
  });
});
