import { test, expect } from '@playwright/test';

test.describe('Business Journey Architecture', () => {
  test.use({ viewport: { width: 375, height: 812 } }); // Mobile-first 375px

  test('Maya the Baker: Full Journey from Onboarding to Morning Briefing', async ({ page }) => {
    // 1. Landing Page
    await page.goto('/');
    await page.click('button:has-text("🚀 Start Business Setup")');
    await expect(page.locator('text="Your business, live in minutes."')).toBeVisible();

    // 2. Select Business Type
    await page.click('button:has-text("🍕 Restaurant / Food")');

    // 3. Business Name
    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', "Maya's Bakes");
    await page.click('button:has-text("Next →")');

    // 4. Sell Categories
    await page.click('button:has-text("📦 Physical products")');

    // 5. Payments
    await page.click('button:has-text("🌐 Online only")');

    // 6. Account Creation
    await page.fill('input[placeholder="e.g. Maya Smith"]', 'Maya');
    await page.fill('input[placeholder="you@email.com"]', 'maya@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('button:has-text("Next →")');

    // 7. Template
    await page.click('button:has-text("✨ Modern")');

    // 8. Add First Product
    await page.fill('input[placeholder="e.g. Custom Birthday Cake"]', 'Classic Chocolate Cake');
    await page.fill('input[placeholder="e.g. 50.00"]', '45.00');
    await page.click('button:has-text("Next →")');

    // 9. Domain
    await page.click('button:has-text("🌐 Free OHC Domain")');

    // 10. Publish
    await page.click('button:has-text("Publish my business →")');

    // 11. AI Magic State (Loading)
    await expect(page.locator('text="Designing your storefront..."')).toBeVisible();
    await expect(page.locator('text="The Promoter is crafting a custom experience"')).toBeVisible();

    // 12. Success & Dashboard
    await page.click('button:has-text("Launch My Business →")');

    // 13. Verify Dashboard & Morning Briefing
    await expect(page.locator('text="Morning Briefing"')).toBeVisible();
    await expect(page.locator('text="Good morning! You\'ve got products ready to go."')).toBeVisible();

    // 14. Verify Success Milestone Celebration
    await expect(page.locator('text="🚀 First Product Added! 🚀"')).toBeVisible();
    await page.click('button:has-text("Amazing!")');
    await expect(page.locator('text="🚀 First Product Added! 🚀"')).not.toBeVisible();
  });

  test('Carlos the Handyman: Service Business Flow', async ({ page }) => {
    await page.goto('/');
    await page.click('button:has-text("🚀 Start Business Setup")');

    await page.click('button:has-text("🛠️ Service Business")');
    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', "Carlos Repairs");
    await page.click('button:has-text("Next →")');

    await page.click('button:has-text("📅 Services / appointments")');
    await page.click('button:has-text("🌍 Both Online & In-person")');

    // Skip to Publish via AI simulation
    await page.click('button:has-text("Next →")'); // Account
    await page.click('button:has-text("✨ Modern")'); // Template
    await page.click('button:has-text("Next →")'); // Product
    await page.click('button:has-text("🌐 Free OHC Domain")'); // Domain
    await page.click('button:has-text("Publish my business →")');

    await page.click('button:has-text("Launch My Business →")');

    await expect(page.locator('text="Morning Briefing"')).toBeVisible();
    await expect(page.locator('text="Carlos Repairs"')).toBeVisible();
  });
});
