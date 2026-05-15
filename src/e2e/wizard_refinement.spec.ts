import { test, expect } from '@playwright/test';

test.describe('Wizard Refinement E2E', () => {

  test('Full Setup Wizard journey (Day One)', async ({ page }) => {
    await page.goto('/login');
    // Simulate navigation to Setup Wizard
    await page.locator('button:has-text("🚀 Start My Business")').filter({ visible: true }).first().click();

    // Step 1: Business Type
    await expect(page.locator('text=/What kind of business/i')).toBeVisible();
    await page.locator('text=🛒 Online Store').filter({ visible: true }).first().click();

    // Step 2: Name & Description
    await page.fill('input[placeholder*="Maya\'s Cakes"]', 'E2E Bakery');
    await page.locator('button:has-text("Next")').click();

    // Step 3: What do you sell
    await page.locator('text=📦 Physical products').filter({ visible: true }).first().click();
    await page.locator('button:has-text("Next")').click();

    // Step 4: Payments
    await page.locator('text=🌐 Online only').filter({ visible: true }).first().click();

    // Step 5: Admin Account
    await page.fill('input[placeholder*="you@email.com"]', 'admin@e2e.test');
    await page.locator('button:has-text("Next")').click();

    // Step 6: Template
    await page.locator('text=✨ Modern').filter({ visible: true }).first().click();
    await page.locator('text=Sunset').click();
    await page.locator('button:has-text("Next")').click();

    // Step 7: Product
    await page.fill('input[placeholder*="Birthday Cake"]', 'Test Cake');
    await page.locator('button:has-text("Next")').click();

    // Step 8: Domain
    await page.locator('text=🌐 Free OHC Domain').filter({ visible: true }).first().click();

    // Step 9: Review & Launch
    await expect(page.locator('text=/Review & Launch/i')).toBeVisible();

    const launchBtn = page.locator('button:has-text("Review & Launch")').filter({ visible: true }).first();
    await expect(launchBtn).toBeVisible();
    await launchBtn.click();

    // Verify Success State
    await expect(page.locator('text=/CONFETTI SUCCESS/i')).toBeVisible({ timeout: 10000 });
  });


  test('Zero to Live - Full Journey & Dashboard Verification', async ({ page }) => {
    // 1. Onboarding
    await page.goto('/login');
    await page.locator('button:has-text("🚀 Start My Business")').filter({ visible: true }).first().click();

    // Setup Wizard steps
    await expect(page.locator('text=/What kind of business/i')).toBeVisible();
    await page.locator('text=🛒 Online Store').filter({ visible: true }).first().click();
    await page.fill('input[placeholder*="Maya\'s Cakes"]', 'My New Bakery');
    await page.locator('button:has-text("Next")').click();
    await page.locator('text=📦 Physical products').filter({ visible: true }).first().click();
    await page.locator('button:has-text("Next")').click();
    await page.locator('text=🌐 Online only').filter({ visible: true }).first().click();
    await page.fill('input[placeholder*="you@email.com"]', 'founder@bakery.test');
    await page.locator('button:has-text("Next")').click();
    await page.locator('text=✨ Modern').filter({ visible: true }).first().click();
    await page.locator('text=Sunset').click();
    await page.locator('button:has-text("Next")').click();
    await page.fill('input[placeholder*="Birthday Cake"]', 'Signature Cake');
    await page.locator('button:has-text("Next")').click();
    await page.locator('text=🌐 Free OHC Domain').filter({ visible: true }).first().click();

    // Review & Launch
    await expect(page.locator('text=/Review & Launch/i')).toBeVisible();
    const launchBtn2 = page.locator('button:has-text("Review & Launch")').filter({ visible: true }).first();
    await launchBtn2.click();

    // Verify Success State
    await expect(page.locator('text=/CONFETTI SUCCESS/i')).toBeVisible({ timeout: 10000 });

    // Navigate to Dashboard
    await page.goto('/');

    // 2. Verify Dashboard Mobile-First Layout
    // Check Top: Weekly Revenue + Actionable Insights
    await expect(page.locator('text=Weekly Revenue')).toBeVisible();
    await expect(page.locator('text=Actionable Insights')).toBeVisible();
    await expect(page.locator('text=Want to run a promo?')).toBeVisible();

    // Check Middle: Pending Orders/Bookings
    await expect(page.locator('text=Pending Orders/Bookings')).toBeVisible();

    // Check Bottom: Floating action buttons
    await expect(page.locator('text="+"').filter({ visible: true }).first()).toBeVisible();
    await expect(page.locator('text="✍️"').filter({ visible: true }).first()).toBeVisible();
  });

  test('AI Helper Configuration', async ({ page }) => {
    await page.goto('/helpers');
    await page.locator('button:has-text("Hire Helper")').click();

    // Verify Personas
    await expect(page.locator('text=The Ambassador')).toBeVisible();
    await expect(page.locator('text=The Promoter')).toBeVisible();
    await expect(page.locator('text=The Salesperson')).toBeVisible();

    await page.locator('text=The Ambassador').click();
    await page.locator('button:has-text("Next")').click();

    // Capabilities
    await page.locator('text=Reply to customer messages').click();
    await page.locator('button:has-text("Next")').click();

    // Frequency
    await page.locator('button:has-text("Next")').click();

    // Review
    await expect(page.locator('text=Helper: Customer Support')).toBeVisible();
    await page.locator('button:has-text("Activate")').click();

    await expect(page.locator('text=Helper Activated ✓')).toBeVisible();
  });

  test('Prompt Tuning Sandbox', async ({ page }) => {
    await page.goto('/tuning');

    // Tone
    await expect(page.locator('text=Tone of Voice')).toBeVisible();
    await page.locator('text=Friendly').click();

    // Verify Preview
    await expect(page.locator('text=/Hi there! 😊/')).toBeVisible();

    await page.locator('text=Energetic').click();
    await expect(page.locator('text=/Let\'s get things moving/')).toBeVisible();

    await page.locator('button:has-text("Next")').click();

    // Focus
    await page.locator('text=Only discuss business').click();
    await page.locator('button:has-text("Next")').click();

    // Examples
    await page.locator('button:has-text("Add Example Interaction")').click();
    await page.locator('button:has-text("Next")').click();

    // Save
    await page.locator('button:has-text("Save")').click();
    await expect(page.locator('text=Your agent has been updated ✓')).toBeVisible();
  });

  test('"Fix This" recovery flow', async ({ page }) => {
    await page.goto('/helpers');
    // Assume an agent is failing
    await page.locator('button:has-text("Help me fix this")').filter({ visible: true }).first().click();

    await expect(page.locator('text=Help Me Fix This')).toBeVisible();
    await expect(page.locator('text=/Something went wrong/')).toBeVisible();

    await page.locator('button:has-text("View Suggested Fix")').click();
    await expect(page.locator('text=/Don\'t worry!/')).toBeVisible();

    await page.locator('button:has-text("Refresh & Reconnect")').click();
    await expect(page.locator('text=/Fix applied successfully/')).toBeVisible();
  });

  test('Progressive Disclosure (Expert Mode) toggle', async ({ page }) => {
    await page.goto('/dashboard');

    // Check Advanced toggle
    const advancedText = page.locator('text=Advanced Mode');
    await expect(advancedText).toBeVisible();

    // Toggle ON
    await page.locator('#mode-toggle').check();
    await page.locator('#mode-toggle').uncheck();
  });

});
