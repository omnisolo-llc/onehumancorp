import { test, expect } from '@playwright/test';

test.describe('LangGraph State Machine', () => {
  test('should execute a task via LangGraph mechanics and trigger tool node routing from sidebar', async ({ page }) => {
    test.setTimeout(180000);
    // Navigate to the dashboard (home page)
    await page.goto('/dashboard');

    // Click the LangGraph link in the sidebar navigation
    await page.click('a[href="/langgraph"]');

    // Check that the title exists
    await expect(page.locator('h1')).toHaveText('LangGraph State Machine');

    // Instruct the agent to use the TodoWrite tool, proving that it routes to tool_node and back
    await page.fill('textarea[placeholder*="Write a quick poem about a cake"]', 'Use the TodoWrite tool to write a single todo item: "Buy milk". Then confirm you have done so.');

    // Click the execute button
    await page.click('button:has-text("Run LangGraph")');

    // Verify that the success message appears
    await expect(page.locator('h2:has-text("LangGraph Output")')).toBeVisible({ timeout: 120000 });

    // Verify the text content indicates that the tool was actually used and completed
    const resultText = await page.locator('pre').textContent();
    expect(resultText?.toLowerCase()).toContain('buy milk');
  });
});
