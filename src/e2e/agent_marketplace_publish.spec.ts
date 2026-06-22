import { test, expect } from '@playwright/test';

test.describe('Agent Marketplace Publish E2E', () => {
  test('User can publish a new agent and then find it in the marketplace', async ({ page }) => {
    // 1. Navigate to Publish page
    await page.goto('/agent-marketplace/publish');
    await expect(page.locator('h1')).toHaveText('Publish New Agent');

    // 2. Fill out the form
    const uniqueAgentName = `E2E Custom Agent ${Date.now()}`;
    await page.locator('#name').fill(uniqueAgentName);
    await page.locator('#description').fill('This agent was created during E2E testing.');
    await page.locator('#role').fill('E2E Tester');
    await page.locator('#systemPrompt').fill('You are a test agent.');

    // 3. Submit the form
    await page.getByRole('button', { name: 'Publish to Marketplace' }).click();

    // 4. Wait for redirect back to the marketplace
    await expect(page).toHaveURL(/\/agent-marketplace$/);
    await expect(page.locator('h1')).toHaveText('Agent Marketplace');

    // 5. Search for the newly created agent
    const searchInput = page.getByPlaceholder('Search for agents...');
    await expect(searchInput).toBeVisible();
    await searchInput.fill(uniqueAgentName);

    // 6. Verify the agent appears
    await expect(page.locator(`text=${uniqueAgentName}`)).toBeVisible();
    await expect(page.locator('text=This agent was created during E2E testing.')).toBeVisible();
  });
});
