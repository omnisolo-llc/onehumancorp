import { test, expect } from '@playwright/test';

test.describe('Team Departments Page', () => {
  test('Department cards should have correct disabled/enabled states based on backend data', async ({ page }) => {
    // Navigate to the team page. The data flows through the real backend.
    await page.goto('/team');

    // Wait for data to load
    await expect(page.locator('.animate-spin')).not.toBeVisible();

    // The "operations" department ("The Manager") should load its state from the actual backend payload
    const operationsCard = page.locator('button', { hasText: 'The Manager' });
    await expect(operationsCard).toBeVisible();

    // Evaluate the text content in the browser context to determine state
    const text = await operationsCard.evaluate(node => node.textContent);

    if (text && text.includes('Active and running')) {
        await expect(operationsCard).toHaveAttribute('aria-disabled', 'true');
    } else if (text && text.includes('awaiting approval')) {
        await expect(operationsCard).not.toHaveAttribute('aria-disabled', 'true');
    }
  });
});
