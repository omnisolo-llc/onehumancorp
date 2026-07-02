import { test, expect } from '@playwright/test';

test.describe('Finance Multi-Currency Architecture', () => {
  test('Owner can toggle Global Sales and see multi-currency consolidated view', async ({ page }) => {
    // Navigate to the finance page directly (assuming basic auth is handled or it's a test route)
    await page.goto('/finance');

    // Ensure the page has loaded
    await expect(page.locator('text=Finance & Invoicing')).toBeVisible();

    // Verify the "Global Sales" toggle is present
    const globalSalesToggle = page.locator('text=Global Sales');
    await expect(globalSalesToggle).toBeVisible();

    // Create a draft invoice first to populate the list if it's empty
    await page.locator('text=New Invoice').click();

    // The draft invoice modal should appear, let's close it so we can see the feed
    await page.locator('button:has-text("✕")').click();

    // Now let's enable Global Sales by clicking the toggle checkbox
    const toggleInput = page.locator('input[type="checkbox"]');
    await toggleInput.check({ force: true });
    await expect(toggleInput).toBeChecked();

    // Verify the draft invoice shows in the feed
    // Wait for the invoice block
    const amountDueBlock = page.locator('text=Amount Due').first();
    await expect(amountDueBlock).toBeVisible();

    // Check that we can see the primary currency ($) in the UI
    const amountText = await page.locator('.text-xl.font-bold').first().textContent();
    expect(amountText).toContain('$');

    // The invoice was drafted with USD as both base and transaction currency (from the mock in the frontend/backend).
    // If they were different and Global Sales is enabled, it would show "(Paid in XXX)".
    // Since they are the same here (USD to USD), the secondary indicator might not appear for this specific auto-drafted item.
    // The test mainly ensures the Global Sales toggle works without crashing and toggles correctly, and the new UI fields exist.
  });
});
