import { test, expect } from '@playwright/test';

test.describe('AI Agentic Predictive Replenishment E2E', () => {
  test('Customer Success Agent can use the predictive replenishment tool to draft a restock message', async ({ page }) => {
    // 1. Log into the application
    await page.goto('/login');
    await page.fill('input[name="email"]', 'maya@example.com');
    await page.fill('input[name="password"]', 'password123');
    await page.click('button[type="submit"]');

    // Wait for the home page to load
    await page.waitForURL('**/dashboard**');

    // 2. Navigate to the AI Agents section
    await page.click('a:has-text("Agents")');
    await page.waitForURL('**/agents**');

    // 3. Select the Customer Success Agent (The Ambassador)
    await page.click('text="Customer Success"');

    // 4. Verify that the new predictive_replenishment tool is available and active
    const toolSelector = 'text="predictive_replenishment"';
    await expect(page.locator(toolSelector)).toBeVisible();

    // 5. Test the chat interface with the Customer Success Agent
    await page.click('button:has-text("Chat")');
    await page.fill('textarea', 'Can you predict when customer cus_predict_1 needs a restock?');
    await page.click('button[aria-label="Send message"]');

    // 6. Verify the AI response contains the predicted date (simulated output)
    const responseSelector = '.agent-message:has-text("predicted to need a restock")';
    await expect(page.locator(responseSelector)).toBeVisible({ timeout: 15000 });
  });
});
