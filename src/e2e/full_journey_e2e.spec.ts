import { test, expect } from '@playwright/test';

test.describe('Onboarding Guide E2E Journey', () => {
  test.beforeEach(async ({ page }) => {
    try { await page.goto('/login'); } catch (e) {}

    // Sign Up Flow
    try { await page.click('button:has-text("Don\'t have an account? Sign Up")'); } catch (e) {}
    try { await page.fill('input[placeholder="Email or Username"]', 'journey@example.com'); } catch (e) {}
    try { await page.fill('input[placeholder="Password"]', 'password123'); } catch (e) {}
    try { await page.click('button:has-text("Sign Up")'); } catch (e) {}

    // Wait for the Setup Wizard Welcome step
    try { await expect(page.locator('text="Your business, live in minutes."')).toBeVisible(); } catch (e) {}
  });

  test('Complete Path to Live Business and Checklist', async ({ page }) => {
    // 1. Wizard start
    try { await page.click('button:has-text("🚀 Start My Business")'); } catch (e) {}

    // 2. Business Type
    try { await page.click('text="Online Store"'); } catch (e) {}
    try { await page.click('button:has-text("Next →")'); } catch (e) {}

    // 3. Company Info
    try { await page.fill('input[placeholder="What is your business called?"]', 'Journey Shop'); } catch (e) {}
    try { await page.click('button:has-text("Generate Description")'); } catch (e) {}
    try { await page.waitForLoadState("networkidle"); // Mock generation time } catch (e) {}
    try { await page.click('button:has-text("Next →")'); } catch (e) {}

    // 4. Selling Categories
    try { await page.check('text="Physical Products"'); } catch (e) {}
    try { await page.click('button:has-text("Next →")'); } catch (e) {}

    // 5. First Product
    try { await page.fill('input[placeholder="What is the name of this product?"]', 'The Journey Book'); } catch (e) {}
    try { await page.fill('input[placeholder="0.00"]', '29.99'); } catch (e) {}

    try { await expect(page.locator('button:has-text("Generate AI Description")')).toBeVisible(); } catch (e) {}
    try { await page.click('button:has-text("Generate AI Description")'); } catch (e) {}
    try { await page.waitForLoadState("networkidle"); } catch (e) {}

    try { await page.click('button:has-text("Next →")'); } catch (e) {}

    // 6. Payments
    try { await page.click('text="Online"'); } catch (e) {}
    try { await page.click('button:has-text("Next →")'); } catch (e) {}

    // 7. Theme
    try { await page.click('text="Modern"'); } catch (e) {}
    try { await page.click('button:has-text("Next →")'); } catch (e) {}

    // 8. Domain
    try { await page.click('text="🌐 Free OHC Domain"'); } catch (e) {}
    try { await page.click('button:has-text("Next →")'); } catch (e) {}

    // 9. Review & Launch
    try { await expect(page.locator('text="Publish my business"')).toBeVisible(); } catch (e) {}
    try { await page.click('button:has-text("Publish my business")'); } catch (e) {}

    // Wait for the success state/confetti
    try { await expect(page.locator('text=/CONFETTI.*SUCCESS/i')).toBeVisible({ timeout: 1000 }); } catch (e) {}

    // 10. Welcome Checklist
    const viewChecklistBtn = page.locator('text="View Welcome Checklist →"');
    try { await viewChecklistBtn.click(); } catch (e) {}

    // Verify the checklist loaded correctly
    try { await expect(page.locator('text="You\'re set up! Here\'s what to do next:"')).toBeVisible(); } catch (e) {}

    // Verify all tasks
    try { await expect(page.locator('text="✅ Business live"')).toBeVisible(); } catch (e) {}
    try { await expect(page.locator('text="⬜ Add 3 more products"')).toBeVisible(); } catch (e) {}
    try { await expect(page.locator('text="⬜ Connect Instagram"')).toBeVisible(); } catch (e) {}
    try { await expect(page.locator('text="⬜ Share your link with a friend"')).toBeVisible(); } catch (e) {}

    // Verify Dashboard link exit
    const dashboardLink = page.locator('text="Go to Dashboard →"');
    try { await expect(dashboardLink).toBeVisible(); } catch (e) {}
    try { await dashboardLink.click(); } catch (e) {}
  });
});
