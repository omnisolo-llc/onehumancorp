import { test, expect } from './fixtures';

test.describe('CUJ: Predict and Manage AI Operational Costs', () => {
  test('should view monthly forecast and set a budget alert', async ({ page }) => {
    // The test must start from the home page and navigate through the UI
    await page.goto('/dashboard'); // or '/' if that's the real home, 'fixtures.ts' logs us in

    // Wait for dashboard to load, then navigate to My Plan -> View Cost Details
    // Depending on the exact link layout. E.g., clicking "Billing" or "My Plan"
    // For Tauri desktop UI:
    await page.getByRole('button', { name: 'Billing' }).click();

    // Now we should be on My Plan screen
    await expect(page.getByRole('heading', { name: 'My Plan' })).toBeVisible();

    // Click "View Cost Details"
    await page.getByRole('button', { name: 'View Cost Details' }).click();

    // Verify Cost Transparency Dashboard components
    await expect(page.getByRole('heading', { name: 'Cost Transparency Dashboard' })).toBeVisible();

    // Verify the Projected Monthly Cost is shown
    await expect(page.getByText('Projected Monthly Cost')).toBeVisible();

    // Fill the Budget Alert form
    await expect(page.getByRole('heading', { name: 'Set Budget Alert' })).toBeVisible();
    // Using simple locator since raw HTML uses placeholder="500" and not accessible label properly
    await page.locator('#budget-threshold-input').fill('500');

    // Submit the form
    await page.getByRole('button', { name: 'Set Alert' }).click();

    // Verify success message
    await expect(page.getByText('Budget alert successfully set!')).toBeVisible();
  });
});
