import { test, expect } from '@playwright/test';

test.describe('Assistant Insights Widget', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the dashboard page where the widget is placed
    await page.goto('/dashboard');
  });

  test('should render the insights widget with pending actions', async ({ page }) => {
    // Wait for the widget to appear
    const widget = page.getByTestId('assistant-insights-widget');
    await expect(widget).toBeVisible({ timeout: 10000 });

    // Assert that the widget header is correct
    await expect(widget.getByRole('heading', { name: 'Assistant Insights' })).toBeVisible();

    // Find the first insight item
    const firstInsight = page.locator('[data-testid^="insight-item-"]').first();
    await expect(firstInsight).toBeVisible();

    // Click the approve button on the first item
    const approveButton = firstInsight.getByRole('button', { name: 'Approve & Send' });
    await expect(approveButton).toBeVisible();
    await approveButton.click();

    // Verify optimistic UI update (the item should disappear)
    await expect(firstInsight).not.toBeVisible();
  });
});
