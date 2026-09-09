import { test, expect } from '../../../../e2e/fixtures';

test.describe('Anthropic 3-Stage Tool Gating - Marketplace Simulation', () => {
  test('should keep marketplace controls usable when the agent service is unavailable', async ({ page }) => {
    test.setTimeout(60000);
    await page.goto('/agent-marketplace');

    await expect(page.getByRole('heading', { name: 'Agent Marketplace' }).first()).toBeVisible();

    const searchInput = page.getByPlaceholder('Search for agents...');
    await expect(searchInput).toBeVisible();
    await searchInput.fill('Sales');
    await searchInput.press('Enter');

    await expect(page.getByText('Failed to fetch agents', { exact: true })).toBeVisible({ timeout: 15000 });
    await expect(page.getByRole('link', { name: 'Publish Agent' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Retry marketplace' })).toBeVisible();
  });

  test('should allow navigating to publish agent page and verify form elements', async ({ page }) => {
    test.setTimeout(60000);
    await page.goto('/agent-marketplace');

    const publishLink = page.getByRole('link', { name: 'Publish Agent' });
    await expect(publishLink).toBeVisible();
    await publishLink.click();

    await expect(page.getByRole('heading', { name: 'Publish New Agent' })).toBeVisible();

    // Verify form fields
    const nameInput = page.getByRole('textbox', { name: /agent name/i });
    await expect(nameInput).toBeVisible();

    const descInput = page.getByRole('textbox', { name: /description/i });
    await expect(descInput).toBeVisible();

    const publishButton = page.getByRole('button', { name: /publish/i });
    await expect(publishButton).toBeVisible();
  });
});
