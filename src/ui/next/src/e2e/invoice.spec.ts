import { test, expect } from '@playwright/test';

test.describe('Agentic Invoicing System E2E', () => {
  test('should verify invoice generator page renders properly', async ({ page }) => {
    // Navigate to the invoice generator page
    await page.goto('/invoice-generator');

    // Verify header
    await expect(page.locator('h1').first()).toHaveText('Invoice Generator');

    // Verify form elements exist
    await expect(page.locator('input[placeholder="e.g. Acme Corp"]')).toBeVisible();
    await expect(page.locator('textarea[placeholder="e.g. Website Redesign and SEO Optimization"]')).toBeVisible();
    await expect(page.locator('input[placeholder="e.g. 1500.00"]')).toBeVisible();
    await expect(page.locator('button:has-text("Generate Shareable Invoice")')).toBeVisible();
  });

  test('should display and approve simulated invoice draft', async ({ page }) => {
    // Navigate to the feed page
    await page.goto('/feed');

    // Make the simulate button visible to be clicked
    await page.evaluate(() => {
      const container = document.querySelector('.opacity-20') as HTMLElement;
      if (container) {
        container.style.opacity = '1';
      }
    });

    // Click the simulate invoice draft button
    await page.click('[data-testid="simulate-invoice-draft-btn"]');

    // Wait for the invoice draft card to appear
    await expect(page.locator('text="Generated Invoice"').first()).toBeVisible({ timeout: 10000 });

    // Approve the invoice draft
    const approveBtn = page.locator('button[data-testid="feed-approve-btn"]', { hasText: 'Approve' }).first();
    await approveBtn.click();

    // Verify it disappears from the feed
    await expect(page.locator('text="Generated Invoice"')).toHaveCount(0);
  });

  test('should display and approve simulated invoice follow-up', async ({ page }) => {
    // Navigate to the feed page
    await page.goto('/feed');

    // Make the simulate button visible to be clicked
    await page.evaluate(() => {
      const container = document.querySelector('.opacity-20') as HTMLElement;
      if (container) {
        container.style.opacity = '1';
      }
    });

    // Click the simulate invoice follow-up button
    await page.click('[data-testid="simulate-invoice-followup-btn"]');

    // Wait for the invoice follow-up card to appear
    await expect(page.locator('text="Overdue Invoice Detected"').first()).toBeVisible({ timeout: 10000 });

    // Approve the invoice follow-up
    const approveBtn = page.locator('button[data-testid="feed-approve-btn"]', { hasText: 'Approve' }).first();
    await approveBtn.click();

    // Verify it disappears from the feed
    await expect(page.locator('text="Overdue Invoice Detected"')).toHaveCount(0);
  });
});
