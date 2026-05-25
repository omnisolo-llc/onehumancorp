import { test, expect } from './fixtures';

test.describe('Onboarding Guide E2E Journey', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/website-builder');
    await expect(page.locator('text="Your business, live in minutes."')).toBeVisible();
  });

  test('Complete Path to Live Business and Checklist', async ({ page }) => {
    // 1. Wizard start
    await page.click('button:has-text("🚀 Start My Business")');

    // 2. Business Type
    await page.click('text="Online Store"');
    await page.click('button:has-text("Next →")');

    // 3. Company Info
    await page.fill('input[placeholder="What is your business called?"]', 'Journey Shop');
    await page.click('button:has-text("Generate Description")');
    await page.waitForLoadState("networkidle");
    await page.click('button:has-text("Next →")');

    // 4. Selling Categories
    await page.check('text="Physical Products"');
    await page.click('button:has-text("Next →")');

    // 5. First Product
    await page.fill('input[placeholder="What is the name of this product?"]', 'The Journey Book');
    await page.fill('input[placeholder="0.00"]', '29.99');

    await expect(page.locator('button:has-text("Generate AI Description")')).toBeVisible();
    await page.click('button:has-text("Generate AI Description")');
    await page.waitForLoadState("networkidle");

    await page.click('button:has-text("Next →")');

    // 6. Payments
    await page.click('text="Online"');
    await page.click('button:has-text("Next →")');

    // 7. Theme
    await page.click('text="Modern"');
    await page.click('button:has-text("Next →")');

    // 8. Domain
    await page.click('text="🌐 Free OHC Domain"');
    await page.click('button:has-text("Next →")');

    // 9. Review & Launch
    await expect(page.locator('text="Publish my business"')).toBeVisible();
    await page.click('button:has-text("Publish my business")');

    // Wait for the success state/confetti
    await expect(page.getByRole('heading', { name: 'Success! Your business is live!' })).toBeVisible({ timeout: 10000 });

    // 10. Welcome Checklist
    const viewChecklistBtn = page.locator('text="View Welcome Checklist →"');
    await viewChecklistBtn.click();

    // Verify the checklist loaded correctly
    await expect(page.locator('text="You\'re set up! Here\'s what to do next:"')).toBeVisible();

    // Verify all tasks
    await expect(page.locator('text="✅ Business live"')).toBeVisible();
    await expect(page.locator('text="⬜ Add 3 more products"')).toBeVisible();
    await expect(page.locator('text="⬜ Connect Instagram"')).toBeVisible();
    await expect(page.locator('text="⬜ Share your link with a friend"')).toBeVisible();

    // Verify Dashboard link exit
    const dashboardLink = page.locator('text="Go to Dashboard →"');
    await expect(dashboardLink).toBeVisible();
    await dashboardLink.click();
  });
});
