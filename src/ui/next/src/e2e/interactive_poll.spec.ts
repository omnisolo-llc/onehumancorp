import { test, expect } from '@playwright/test';

test.describe('Interactive Poll Generator E2E', () => {
  test('should allow creating a poll and viewing embed modal', async ({ page }) => {
    // Navigate directly to the generator
    await page.goto('/interactive-poll-generator');

    // Ensure the page loaded
    await expect(page.getByRole('heading', { name: 'Interactive Poll Generator' })).toBeVisible();

    // Change question
    await page.getByPlaceholder('E.g., What should we build next?').fill('Which feature next?');

    // The preview should update
    await expect(page.getByRole('heading', { name: 'Which feature next?' })).toBeVisible();

    // Add option
    await page.getByRole('button', { name: '+ Add Option' }).click();
    await page.getByPlaceholder('Option 4').fill('Something else');

    // Ensure new option is in preview
    await expect(page.getByText('Something else', { exact: true })).toBeVisible();

    // Toggle dark theme
    await page.getByLabel('Dark').click();

    // Toggle email requirement
    await page.getByLabel(/Require Email to Vote/).check();
    await expect(page.getByPlaceholder('Enter your email to vote')).toBeVisible();

    // Generate Embed Code
    await page.getByRole('button', { name: 'Generate Embed Code' }).click();

    // Verify modal appears with embed code
    await expect(page.getByRole('heading', { name: 'Your Embed Code' })).toBeVisible();

    // Code should contain our custom question URL encoded
    await expect(page.locator('pre')).toContainText('Which%20feature%20next%3F');
    await expect(page.locator('pre')).toContainText('email=true');
    await expect(page.locator('pre')).toContainText('theme=dark');

    // Close modal
    await page.getByRole('button', { name: 'Close' }).click();
    await expect(page.getByRole('heading', { name: 'Your Embed Code' })).not.toBeVisible();
  });

  test('should handle soft paywall for removing branding', async ({ page }) => {
    // Set localStorage explicitly for the test to ensure no pro status
    await page.addInitScript(() => {
      window.localStorage.setItem('has_pro', 'false');
    });

    await page.goto('/interactive-poll-generator');

    // Try to check the remove branding box
    await page.getByLabel(/Remove OHC Branding/).check();

    // Verify paywall modal appears
    await expect(page.getByRole('heading', { name: 'Upgrade to Pro' })).toBeVisible();

    // Click 'Keep Branding for Now'
    await page.getByRole('button', { name: 'Keep Branding for Now' }).click();

    // Paywall goes away
    await expect(page.getByRole('heading', { name: 'Upgrade to Pro' })).not.toBeVisible();
  });

  test('dashboard should have a link to the generator', async ({ page }) => {
    // Set up onboarded state to access dashboard
    await page.addInitScript(() => {
      window.localStorage.setItem('has_onboarded', 'true');
    });

    await page.goto('/dashboard');

    // Verify link exists
    const link = page.getByRole('link', { name: /Interactive Poll Generator/i });
    await expect(link).toBeVisible();

    // Click and verify navigation
    await link.click();
    await expect(page).toHaveURL(/.*\/interactive-poll-generator/);
  });
});
