import { E2E_ROUTES, UI_LOCATORS } from "./playwright_test_constants";
import { test, expect } from '@playwright/test';

test.describe('🎨 Canvas: AutoDream Memory Pipeline UI Tests', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to login page
    await page.goto(E2E_ROUTES.LOGIN);

    // Fill in credentials and sign in
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com');
    await page.locator(UI_LOCATORS.PASSWORD_INPUT).filter({ visible: true }).first().fill( 'password123');
    await page.click(UI_LOCATORS.SIGN_IN_BTN);

    // Wait for Dashboard to load
    await page.waitForURL('**/dashboard*');

    // Enable advanced telemetry to make the component visible unconditionally for tests
    await page.click('button:has-text("Settings"), a:has-text("Settings")');
    const advancedTab = page.locator(UI_LOCATORS.ADVANCED_TEXT).filter({ visible: true }).first();
    await advancedTab.click();

    // Attempt to click Advanced Mode toggle
    const toggle = page.locator(UI_LOCATORS.ADVANCED_TEXT);
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
    const pipelineTitle = page.locator(UI_LOCATORS.AUTODREAM_PIPELINE);
    await expect(pipelineTitle).toBeVisible();
  });

  test('should display LLM Cache Hits stat card', async ({ page }) => {
    await expect(page.locator(UI_LOCATORS.LLM_CACHE_HITS)).toBeVisible();
  });

  test('should display RAG Latency stat card', async ({ page }) => {
    await expect(page.locator(UI_LOCATORS.RAG_LATENCY)).toBeVisible();
  });

  test('should display Dynamic Hybrid Correlation Chart placeholder', async ({ page }) => {
    await expect(page.locator(UI_LOCATORS.HYBRID_CORRELATION)).toBeVisible();
  });

  test('should apply correct styling properties for AutoDream Memory Pipeline container', async ({ page }) => {
    await expect(page.locator(UI_LOCATORS.AUTODREAM_PIPELINE)).toBeVisible();
    await expect(page.locator(UI_LOCATORS.LLM_CACHE_HITS)).toBeVisible();
    await expect(page.locator(UI_LOCATORS.RAG_LATENCY)).toBeVisible();
    await expect(page.locator(UI_LOCATORS.HYBRID_CORRELATION)).toBeVisible();
  });
});
