import { test, expect } from '@playwright/test';

test.describe('Agent Marketplace', () => {
  test('should load the agent marketplace and allow searching, verifying basic data presence', async ({ page }) => {
    test.setTimeout(60000);
    await page.goto('/agent-marketplace');

    await expect(page.getByRole('heading', { name: 'Agent Marketplace' })).toBeVisible();

    const searchInput = page.getByPlaceholder('Search for agents...');
    await expect(searchInput).toBeVisible();
    await searchInput.fill('Sales');
    await searchInput.press('Enter');

    // Wait for the grid of agents to populate
    const agentCards = page.locator('h3').first();
    await expect(agentCards).toBeVisible({ timeout: 15000 });
  });

  test('should allow navigating to publish agent page and verify form elements', async ({ page }) => {
    test.setTimeout(60000);
    await page.goto('/agent-marketplace');

    const publishLink = page.getByRole('link', { name: 'Publish Agent' });
    await expect(publishLink).toBeVisible();
    await publishLink.click();

    await expect(page.getByRole('heading', { name: 'Publish New Agent' })).toBeVisible();

    // Verify form fields
    const nameInput = page.getByRole('textbox', { name: /name/i }).first();
    await expect(nameInput).toBeVisible();

    const descInput = page.getByRole('textbox', { name: /description/i });
    await expect(descInput).toBeVisible();

    const publishButton = page.getByRole('button', { name: /publish/i });
    await expect(publishButton).toBeVisible();
  });
});
