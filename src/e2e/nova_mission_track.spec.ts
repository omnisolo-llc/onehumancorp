import { test, expect } from '@playwright/test';

test.describe('Nova Mission Track', () => {
  test('should display the Nova Mission Track dashboard and filter correctly', async ({ page }) => {
    await page.goto('/nova-mission-track');

    // Check header
    await expect(page.locator('h1')).toContainText('Mission Control');
    await expect(page.locator('text=Nova Track')).toBeVisible();

    // Check all missions are visible initially
    await expect(page.locator('text=Dynamic Organization Generation')).toBeVisible();
    await expect(page.locator('text=Apply Design Tokens')).toBeVisible();
    await expect(page.locator('text=Multimodal LLM Endpoints')).toBeVisible();

    // Filter by Active
    await page.click('button:has-text("active")');
    await expect(page.locator('text=Multimodal LLM Endpoints')).toBeVisible();
    await expect(page.locator('text=Dynamic Organization Generation')).not.toBeVisible();

    // Filter by Completed
    await page.click('button:has-text("completed")');
    await expect(page.locator('text=Dynamic Organization Generation')).toBeVisible();
    await expect(page.locator('text=Multimodal LLM Endpoints')).not.toBeVisible();

    // Filter by Pending
    await page.click('button:has-text("pending")');
    await expect(page.locator('text=Semantic Distillation Worker')).toBeVisible();
    await expect(page.locator('text=Multimodal LLM Endpoints')).not.toBeVisible();
    await expect(page.locator('text=Dynamic Organization Generation')).not.toBeVisible();

    // Test search functionality
    await page.click('button:has-text("all")'); // Reset filter

    const searchInput = page.getByPlaceholder('Search missions...');
    await expect(searchInput).toBeVisible();

    // Search by title
    await searchInput.fill('Semantic');
    await expect(page.locator('text=Semantic Distillation Worker')).toBeVisible();
    await expect(page.locator('text=Apply Design Tokens')).not.toBeVisible();

    // Search by agent
    await searchInput.fill('Architect');
    await expect(page.locator('text=Dynamic Organization Generation')).toBeVisible();
    await expect(page.locator('text=Semantic Distillation Worker')).not.toBeVisible();

    // Empty search state
    await searchInput.fill('ThisWillNotMatchAnything');
    await expect(page.locator('text=No missions found')).toBeVisible();
  });
});
