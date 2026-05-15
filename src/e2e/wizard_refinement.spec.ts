import { E2E_ROUTES, UI_LOCATORS } from "./playwright_test_constants";
import { test, expect } from '@playwright/test';

test.describe('Wizard Refinement E2E', () => {

  test('Full Setup Wizard journey (Day One)', async ({ page }) => {
    await page.goto(E2E_ROUTES.LOGIN);
    // Simulate navigation to Setup Wizard
    await page.locator(UI_LOCATORS.START_MY_BUSINESS).filter({ visible: true }).first().click();

    // Step 1: Business Type
    await expect(page.locator(UI_LOCATORS.WHAT_KIND_BUSINESS)).toBeVisible();
    await page.locator(UI_LOCATORS.ONLINE_STORE).filter({ visible: true }).first().click();

    // Step 2: Name & Description
    await page.fill('input[placeholder*="Maya\'s Cakes"]', 'E2E Bakery');
    await page.locator(UI_LOCATORS.NEXT_BUTTON).click();

    // Step 3: What do you sell
    await page.locator(UI_LOCATORS.PHYSICAL_PRODUCTS).filter({ visible: true }).first().click();
    await page.locator(UI_LOCATORS.NEXT_BUTTON).click();

    // Step 4: Payments
    await page.locator(UI_LOCATORS.ONLINE_ONLY).filter({ visible: true }).first().click();

    // Step 5: Admin Account
    await page.fill('input[placeholder*="you@email.com"]', 'admin@e2e.test');
    await page.locator(UI_LOCATORS.NEXT_BUTTON).click();

    // Step 6: Template
    await page.locator(UI_LOCATORS.MODERN).filter({ visible: true }).first().click();

    // Step 7: Product
    await page.fill('input[placeholder*="Birthday Cake"]', 'Test Cake');
    await page.locator(UI_LOCATORS.NEXT_BUTTON).click();

    // Step 8: Domain
    await page.locator(UI_LOCATORS.FREE_DOMAIN).filter({ visible: true }).first().click();

    // Step 9: Review & Launch
    await expect(page.locator(UI_LOCATORS.READY_TO_LAUNCH)).toBeVisible();
    await expect(page.locator('text=Business: E2E Bakery')).toBeVisible();

    const launchBtn = page.locator(UI_LOCATORS.LAUNCH_MY_BUSINESS).filter({ visible: true }).first();
    await expect(launchBtn).toBeVisible();
    await launchBtn.click();

    // Verify Success State
    await expect(page.locator(UI_LOCATORS.SUCCESS_BUSINESS_LIVE)).toBeVisible({ timeout: 10000 });
    await expect(page.locator('button:has-text("Copy Store Link")')).toBeVisible();
  });


  test('Zero to Live - Full Journey & Dashboard Verification', async ({ page }) => {
    // 1. Onboarding
    await page.goto(E2E_ROUTES.LOGIN);
    await page.locator(UI_LOCATORS.START_MY_BUSINESS).filter({ visible: true }).first().click();

    // Setup Wizard steps
    await expect(page.locator(UI_LOCATORS.WHAT_KIND_BUSINESS)).toBeVisible();
    await page.locator(UI_LOCATORS.ONLINE_STORE).filter({ visible: true }).first().click();
    await page.fill('input[placeholder*="Maya\'s Cakes"]', 'My New Bakery');
    await page.locator(UI_LOCATORS.NEXT_BUTTON).click();
    await page.locator(UI_LOCATORS.PHYSICAL_PRODUCTS).filter({ visible: true }).first().click();
    await page.locator(UI_LOCATORS.NEXT_BUTTON).click();
    await page.locator(UI_LOCATORS.ONLINE_ONLY).filter({ visible: true }).first().click();
    await page.fill('input[placeholder*="you@email.com"]', 'founder@bakery.test');
    await page.locator(UI_LOCATORS.NEXT_BUTTON).click();
    await page.locator(UI_LOCATORS.MODERN).filter({ visible: true }).first().click();
    await page.fill('input[placeholder*="Birthday Cake"]', 'Signature Cake');
    await page.locator(UI_LOCATORS.NEXT_BUTTON).click();
    await page.locator(UI_LOCATORS.FREE_DOMAIN).filter({ visible: true }).first().click();

    // Review & Launch
    await expect(page.locator(UI_LOCATORS.READY_TO_LAUNCH)).toBeVisible();
    const launchBtn = page.locator(UI_LOCATORS.LAUNCH_MY_BUSINESS).filter({ visible: true }).first();
    await launchBtn.click();

    // Verify Success State
    await expect(page.locator(UI_LOCATORS.SUCCESS_BUSINESS_LIVE)).toBeVisible({ timeout: 10000 });

    // Navigate to Dashboard
    await page.goto(E2E_ROUTES.HOME);

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
    await expect(page.locator(UI_LOCATORS.THE_AMBASSADOR)).toBeVisible();
    await expect(page.locator('text=The Promoter')).toBeVisible();
    await expect(page.locator('text=The Salesperson')).toBeVisible();

    await page.locator(UI_LOCATORS.THE_AMBASSADOR).click();
    await page.locator(UI_LOCATORS.NEXT_BUTTON).click();

    // Capabilities
    await page.locator('text=Reply to customer messages').click();
    await page.locator(UI_LOCATORS.NEXT_BUTTON).click();

    // Frequency
    await page.locator(UI_LOCATORS.NEXT_BUTTON).click();

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

    await page.locator(UI_LOCATORS.NEXT_BUTTON).click();

    // Focus
    await page.locator('text=Only discuss business').click();
    await page.locator(UI_LOCATORS.NEXT_BUTTON).click();

    // Examples
    await page.locator('button:has-text("Add Example Interaction")').click();
    await page.locator(UI_LOCATORS.NEXT_BUTTON).click();

    // Save
    await page.locator(UI_LOCATORS.SAVE).click();
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
    await page.goto('/onboarding');
    await page.locator(UI_LOCATORS.START_MY_BUSINESS).filter({ visible: true }).first().click();

    // Check Advanced toggle
    const advancedText = page.locator('text=Advanced Mode');
    await expect(advancedText).toBeVisible();

    // Toggle ON
    await page.locator('text=Advanced Mode').locator('..').locator('TouchArea').click();
    await expect(page.locator(UI_LOCATORS.RAW_CONFIG)).toBeVisible();

    // Toggle OFF
    await page.locator('text=Advanced Mode').locator('..').locator('TouchArea').click();
    await expect(page.locator(UI_LOCATORS.RAW_CONFIG)).not.toBeVisible();
  });

});
