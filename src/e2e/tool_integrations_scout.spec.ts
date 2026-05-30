import { test, expect } from './fixtures';

test.describe('Scout Custom Tool Integration UI', () => {
  test.beforeEach(async ({ page }) => {
    // Dismiss the upgrade modal if it appears
    page.on('dialog', dialog => dialog.accept());
    await page.goto('/integrations');
    await expect(page.getByRole('heading', { name: 'Tool Integrations' }).first()).toBeVisible();
  });

  test('can trigger Scout Agent to build custom tool integration', async ({ page }) => {
    // Wait for page load
    await page.waitForTimeout(1000);

    // Locate the custom tool integration card and click Connect
    const customToolCard = page.locator('.rounded-\\[16px\\]').filter({ hasText: 'Custom Tool' });
    await expect(customToolCard).toBeVisible();
    await customToolCard.getByRole('button', { name: 'Connect' }).click();

    // Verify modal appeared
    await expect(page.getByRole('heading', { name: 'Request Custom Integration' })).toBeVisible();

    // Fill the form
    await page.getByPlaceholder('e.g. My Custom API').fill('Dummy Search API');
    await page.getByPlaceholder('What does this tool do?').fill('Searches dummy data');
    await page.getByPlaceholder('https://api.example.com').fill('https://api.dummy-search.com');

    // Submit the form
    const submitBtn = page.getByRole('button', { name: 'Submit Request' });
    await submitBtn.click();

    // The modal should close and the UI should reflect the connected state
    await expect(page.getByRole('heading', { name: 'Request Custom Integration' })).not.toBeVisible();
    await expect(customToolCard.getByRole('button', { name: 'Manage' })).toBeVisible();
    await expect(customToolCard.getByText('connected')).toBeVisible();
  });
});
