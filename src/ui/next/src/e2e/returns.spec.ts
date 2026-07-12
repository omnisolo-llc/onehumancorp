import { test, expect } from '../../../../e2e/fixtures';

test.describe('Omnichannel Returns & Exchange Flow', () => {
  test('Customer initiates return and Owner approves it', async ({ page }) => {
    // Phase 1: Customer View
    await page.goto('/returns');

    // Fill return form
    await page.fill('input[type="text"]', 'ORD-12345');
    await page.selectOption('select', 'Refund');
    await page.fill('textarea', 'Item was too small');

    // Submit request
    await page.click('button[type="submit"]');

    // Validate success message
    await expect(page.locator('text=Request Submitted')).toBeVisible();

    // Phase 2: Owner View (Work Feed)
    // We mock login token logic just for the E2E to access the protected page
    await page.addInitScript(() => {
      window.localStorage.setItem('token', 'test-owner-token');
    });

    await page.goto('/action-center');

    // Find the return card
    // The returns API initiated a draft, which action center should show
    await expect(page.locator('text=Process Refund and refund for order ORD-12345')).toBeVisible();

    // Approve it
    await page.click('button:has-text("Approve & Send")');

    // Check for success banner
    await expect(page.locator('text=Action approved and executed.')).toBeVisible();
  });
});
