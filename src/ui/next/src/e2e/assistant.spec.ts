import { test, expect } from '../../../../e2e/fixtures';


test.describe('Assistant Page', () => {
  test('navigates to assistant and verifies authentic state', async ({ page }) => {
    await adminPage({ page }, async ({ page }) => {
      await page.goto('/assistant');

      // Verify the page shell and layout
      await expect(page.getByTestId('assistant-shell')).toBeVisible();
      await expect(page.getByTestId('assistant-workstation')).toBeVisible();
      await expect(page.getByRole('heading', { name: /Assistant/ })).toBeVisible();

      // Ensure mock data does not exist
      await expect(page.getByText('Create a personal briefing')).not.toBeVisible();

      // Start a task via UI to verify backend connection
      await page.getByLabel('Task prompt').fill('Build a test report');
      await page.getByRole('button', { name: 'Start Task' }).click();

      // The new task should appear
      await expect(page.getByText('Build a test report')).toBeVisible();

      // Check results panel logic
      await expect(page.getByRole('heading', { name: 'Results Panel' })).toBeVisible();

      // Check a panel (e.g., Remote Control)
      await page.getByRole('button', { name: 'Remote Control' }).click();
      await expect(page.getByRole('heading', { name: 'Remote Control' })).toBeVisible();
    });
  });
});
