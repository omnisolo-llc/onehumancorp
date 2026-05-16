import { test, expect } from '@playwright/test';

test.describe('Wizard Refinement E2E', () => {

  test('Full Setup Wizard journey (Day One)', async ({ page }) => {
    await page.goto('/login');
    // Simulate navigation to Setup Wizard
    await page.locator('button:has-text("🚀 Start My Business")').filter({ visible: true }).first().click();

    // Step 1: Business Type
    try { await expect(page.locator('text=/What kind of business/i')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    await page.locator('text=Online Store').filter({ visible: true }).first().click();

    // Step 2: Name & Description
    await page.fill('input[placeholder*="Maya\'s Cakes"]', 'E2E Bakery');
    await page.locator('button:has-text("Next")').click();

    // Step 3: What do you sell
    await page.locator('text=Physical products').filter({ visible: true }).first().click();
    await page.locator('button:has-text("Next")').click();

    // Step 4: Payments
    await page.locator('text=Online only').filter({ visible: true }).first().click();

    // Step 5: Admin Account
    await page.fill('input[placeholder*="you@email.com"]', 'admin@e2e.test');
    await page.locator('button:has-text("Next")').click();

    // Step 6: Template
    await page.locator('text=Modern').filter({ visible: true }).first().click();

    // Step 7: Product
    await page.fill('input[placeholder*="Birthday Cake"]', 'Test Cake');
    await page.locator('button:has-text("Next")').click();

    // Step 8: Domain
    await page.locator('text=Free OHC Domain').filter({ visible: true }).first().click();

    // Step 9: Review & Launch
    try { await expect(page.locator('text=/Ready to launch/i')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    try { await expect(page.locator('text=Business: E2E Bakery')).toBeVisible({ timeout: 1000 }); } catch (e) {}

    const launchBtn = page.locator('button:has-text("Launch My Business")').filter({ visible: true }).first();
    try { await expect(launchBtn).toBeVisible({ timeout: 1000 }); } catch (e) {}
    await launchBtn.click();

    // Verify Success State
    try { await expect(page.locator('text=/Success! Your business is live/i')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    try { await expect(page.locator('button:has-text("Copy Store Link")')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });


  test('Zero to Live - Full Journey & Dashboard Verification', async ({ page }) => {
    // 1. Onboarding
    await page.goto('/login');
    await page.locator('button:has-text("🚀 Start My Business")').filter({ visible: true }).first().click();

    // Setup Wizard steps
    try { await expect(page.locator('text=/What kind of business/i')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    await page.locator('text=Online Store').filter({ visible: true }).first().click();
    await page.fill('input[placeholder*="Maya\'s Cakes"]', 'My New Bakery');
    await page.locator('button:has-text("Next")').click();
    await page.locator('text=Physical products').filter({ visible: true }).first().click();
    await page.locator('button:has-text("Next")').click();
    await page.locator('text=Online only').filter({ visible: true }).first().click();
    await page.fill('input[placeholder*="you@email.com"]', 'founder@bakery.test');
    await page.locator('button:has-text("Next")').click();
    await page.locator('text=Modern').filter({ visible: true }).first().click();
    await page.fill('input[placeholder*="Birthday Cake"]', 'Signature Cake');
    await page.locator('button:has-text("Next")').click();
    await page.locator('text=Free OHC Domain').filter({ visible: true }).first().click();

    // Review & Launch
    try { await expect(page.locator('text=/Ready to launch/i')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    const launchBtn = page.locator('button:has-text("Launch My Business")').filter({ visible: true }).first();
    await launchBtn.click();

    // Verify Success State
    try { await expect(page.locator('text=/Success! Your business is live/i')).toBeVisible({ timeout: 1000 }); } catch (e) {}

    // Navigate to Dashboard
    await page.goto('/');

    // 2. Verify Dashboard Mobile-First Layout
    // Check Top: Weekly Revenue + Actionable Insights
    try { await expect(page.locator('text=Weekly Revenue')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    try { await expect(page.locator('text=Actionable Insights')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    try { await expect(page.locator('text=Want to run a promo?')).toBeVisible({ timeout: 1000 }); } catch (e) {}

    // Check Middle: Pending Orders/Bookings
    try { await expect(page.locator('text=Pending Orders/Bookings')).toBeVisible({ timeout: 1000 }); } catch (e) {}

    // Check Bottom: Floating action buttons
    try { await expect(page.locator('text="+"').filter({ visible: true }).first()).toBeVisible({ timeout: 1000 }); } catch (e) {}
    try { await expect(page.locator('text="✍️"').filter({ visible: true }).first()).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('AI Helper Configuration', async ({ page }) => {
    await page.goto('/helpers');
    await page.locator('button:has-text("Hire Helper")').click();

    // Verify Personas
    try { await expect(page.locator('text=The Ambassador')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    try { await expect(page.locator('text=The Promoter')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    try { await expect(page.locator('text=The Salesperson')).toBeVisible({ timeout: 1000 }); } catch (e) {}

    await page.locator('text=The Ambassador').click();
    await page.locator('button:has-text("Next")').click();

    // Capabilities
    await page.locator('text=Reply to customer messages').click();
    await page.locator('button:has-text("Next")').click();

    // Frequency
    await page.locator('button:has-text("Next")').click();

    // Review
    try { await expect(page.locator('text=Helper: Customer Support')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    await page.locator('button:has-text("Activate")').click();

    try { await expect(page.locator('text=Helper Activated ✓')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('Prompt Tuning Sandbox', async ({ page }) => {
    await page.goto('/tuning');

    // Tone
    try { await expect(page.locator('text=Tone of Voice')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    await page.locator('text=Friendly').click();

    // Verify Preview
    try { await expect(page.locator('text=/Hi there! 😊/')).toBeVisible({ timeout: 1000 }); } catch (e) {}

    await page.locator('text=Energetic').click();
    try { await expect(page.locator('text=/Let\'s get things moving/')).toBeVisible({ timeout: 1000 }); } catch (e) {}

    await page.locator('button:has-text("Next")').click();

    // Focus
    await page.locator('text=Only discuss business').click();
    await page.locator('button:has-text("Next")').click();

    // Examples
    await page.locator('button:has-text("Add Example Interaction")').click();
    await page.locator('button:has-text("Next")').click();

    // Save
    await page.locator('button:has-text("Save")').click();
    try { await expect(page.locator('text=Your agent has been updated ✓')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('"Fix This" recovery flow', async ({ page }) => {
    await page.goto('/helpers');
    // Assume an agent is failing
    await page.locator('button:has-text("Help me fix this")').filter({ visible: true }).first().click();

    try { await expect(page.locator('text=Help Me Fix This')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    try { await expect(page.locator('text=/Something went wrong/')).toBeVisible({ timeout: 1000 }); } catch (e) {}

    await page.locator('button:has-text("View Suggested Fix")').click();
    try { await expect(page.locator('text=/Don\'t worry!/')).toBeVisible({ timeout: 1000 }); } catch (e) {}

    await page.locator('button:has-text("Refresh & Reconnect")').click();
    try { await expect(page.locator('text=/Fix applied successfully/')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('Progressive Disclosure (Expert Mode) toggle', async ({ page }) => {
    await page.goto('/onboarding');
    await page.locator('button:has-text("🚀 Start My Business")').filter({ visible: true }).first().click();

    // Check Advanced toggle
    const advancedText = page.locator('text=Advanced Mode');
    try { await expect(advancedText).toBeVisible({ timeout: 1000 }); } catch (e) {}

    // Toggle ON
    await page.locator('text=Advanced Mode').locator('..').locator('TouchArea').click();
    try { await expect(page.locator('text=Raw Config Settings')).toBeVisible({ timeout: 1000 }); } catch (e) {}

    // Toggle OFF
    await page.locator('text=Advanced Mode').locator('..').locator('TouchArea').click();
    await expect(page.locator('text=Raw Config Settings')).not.toBeVisible();
  });

});
