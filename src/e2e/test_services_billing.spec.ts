import { test, expect } from './fixtures';

test.describe('Subscription Billing Services', () => {
  test('should create a service with recurring payment', async ({ page }) => {
    await page.goto('/services/new');
    await expect(page.getByRole('heading', { name: 'Add Service' })).toBeVisible();

    // Fill the title
    await page.fill('input[placeholder="e.g. Weekly Music Tutoring"]', 'Weekly Math Tutoring');

    // Click auto-draft
    await page.click('button:has-text("Auto-draft")');
    await expect(page.locator('textarea')).not.toBeEmpty();

    // Fill price
    await page.fill('input[placeholder="0.00"]', '50');

    // Enable recurring
    await page.click('input[type="checkbox"] + div');

    // Select frequency
    await page.selectOption('select', 'weekly');

    // Save
    await page.click('button:has-text("Save Service")');

    // Check success
    await expect(page.getByRole('heading', { name: 'Service Saved!' })).toBeVisible();
  });

  test('should redirect to dashboard on mobile', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/services/new');
    await page.click('a:has-text("< Back")');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });
});
