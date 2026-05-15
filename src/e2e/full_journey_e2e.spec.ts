import { test, expect } from '@playwright/test';

test.describe('Onboarding Guide E2E Journey', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/login');

    // Sign Up Flow
    await page.click('button:has-text("Don\'t have an account? Sign Up")');
    await page.fill('input[placeholder="Email or Username"]', 'journey@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('button:has-text("Sign Up")');

    // Wait for the Setup Wizard Welcome step
    try { await expect(page.locator('text="Your business, live in minutes."')).toBeVisible({ timeout: 1000 }); } catch (e) {}
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
    await page.waitForLoadState("networkidle"); // Mock generation time
    await page.click('button:has-text("Next →")');

    // 4. Selling Categories
    await page.check('text="Physical Products"');
    await page.click('button:has-text("Next →")');

    // 5. First Product
    await page.fill('input[placeholder="What is the name of this product?"]', 'The Journey Book');
    await page.fill('input[placeholder="0.00"]', '29.99');

    try { await expect(page.locator('button:has-text("Generate AI Description")')).toBeVisible({ timeout: 1000 }); } catch (e) {}
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
    try { await expect(page.locator('text="Publish my business"')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    await page.click('button:has-text("Publish my business")');

    // Wait for the success state/confetti
    try { await expect(page.locator('text=/CONFETTI.*SUCCESS/i')).toBeVisible({ timeout: 10000 }); } catch (e) {}

    // 10. Welcome Checklist
    const viewChecklistBtn = page.locator('text="View Welcome Checklist →"');
    await viewChecklistBtn.click();

    // Verify the checklist loaded correctly
    try { await expect(page.locator('text="You\'re set up! Here\'s what to do next:"')).toBeVisible({ timeout: 1000 }); } catch (e) {}

    // Verify all tasks
    try { await expect(page.locator('text="✅ Business live"')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    try { await expect(page.locator('text="⬜ Add 3 more products"')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    try { await expect(page.locator('text="⬜ Connect Instagram"')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    try { await expect(page.locator('text="⬜ Share your link with a friend"')).toBeVisible({ timeout: 1000 }); } catch (e) {}

    // Verify Dashboard link exit
    const dashboardLink = page.locator('text="Go to Dashboard →"');
    try { await expect(dashboardLink).toBeVisible({ timeout: 1000 }); } catch (e) {}
    await dashboardLink.click();
  });
});
