import { test, expect } from '@playwright/test';

test.describe('Wizard Refinement E2E', () => {

  test('Full Setup Wizard journey (Day One Conversational Onboarding)', async ({ page }) => {
    await page.goto('/');

    // Step 1: Start chat with Promoter
    await expect(page.locator('text=/What kind of business/i')).toBeVisible();

    // Enter prompt simulating a user building a custom bakery storefront
    const input = page.locator('input[placeholder*="e.g."]');
    await input.fill('I bake custom cakes');

    // Press the send button
    const sendBtn = page.locator('button').locator('nth=1'); // fallback if key not accessible
    await sendBtn.click();

    // AI Processing state
    await expect(page.locator('text=I have generated a storefront preview for you.')).toBeVisible({ timeout: 10000 });

    // Wait for transition to preview
    await page.waitForTimeout(1500);

    // Verify Preview State
    await expect(page.locator('text=Live Preview')).toBeVisible();
    await expect(page.locator('text=Custom Bakery')).toBeVisible();

    const launchBtn = page.locator('button:has-text("Launch My Business")').first();
    await expect(launchBtn).toBeVisible();
    await launchBtn.click();

    // Navigate to Dashboard Check
    await expect(page.locator('text=Dashboard')).toBeVisible({ timeout: 5000 });
  });

  test('Zero to Live - Full Journey & Dashboard Verification', async ({ page }) => {
    // 1. Onboarding
    await page.goto('/');

    // Start chat with Promoter
    await expect(page.locator('text=/What kind of business/i')).toBeVisible();
    const input = page.locator('input[placeholder*="e.g."]');
    await input.fill('My New Bakery');

    // Press the send button
    const sendBtn = page.locator('button').locator('nth=1'); // fallback if key not accessible
    await sendBtn.click();

    await expect(page.locator('text=I have generated a storefront preview for you.')).toBeVisible({ timeout: 10000 });

    // Wait for transition to preview
    await page.waitForTimeout(1500);

    const launchBtn = page.locator('button:has-text("Launch My Business")').first();
    await expect(launchBtn).toBeVisible();
    await launchBtn.click();

    // Navigate to Dashboard Check
    await expect(page.locator('text=Dashboard')).toBeVisible({ timeout: 5000 });

    // 2. Verify Dashboard Mobile-First Layout
    // Check Top: Weekly Revenue + Actionable Insights
    await expect(page.locator('text=Weekly Revenue')).toBeVisible();
    await expect(page.locator('text=Actionable Insights')).toBeVisible();
    await expect(page.locator('text=Want to run a promo?')).toBeVisible();

    // Check Middle: Pending Orders/Bookings
    await expect(page.locator('text=Pending Orders/Bookings')).toBeVisible();

    // Check Bottom: Floating action buttons
    await expect(page.locator('text="+"').first()).toBeVisible();
    await expect(page.locator('text="✍️"').first()).toBeVisible();
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
    await page.locator('button:has-text("Help me fix this")').first().click();

    await expect(page.locator('text=Help Me Fix This')).toBeVisible();
    await expect(page.locator('text=/Something went wrong/')).toBeVisible();

    await page.locator('button:has-text("View Suggested Fix")').click();
    await expect(page.locator('text=/Don\'t worry!/')).toBeVisible();

    await page.locator('button:has-text("Refresh & Reconnect")').click();
    await expect(page.locator('text=/Fix applied successfully/')).toBeVisible();
  });

  test('Progressive Disclosure (Expert Mode) toggle', async ({ page }) => {
    await page.goto('/onboarding');
    await page.locator('button:has-text("🚀 Start My Business")').first().click();

    // Check Advanced toggle
    const advancedText = page.locator('text=Advanced Mode');
    await expect(advancedText).toBeVisible();

    // Toggle ON
    await page.locator('text=Advanced Mode').locator('..').locator('TouchArea').click();
    await expect(page.locator('text=Raw Config Settings')).toBeVisible();

    // Toggle OFF
    await page.locator('text=Advanced Mode').locator('..').locator('TouchArea').click();
    await expect(page.locator('text=Raw Config Settings')).not.toBeVisible();
  });

});
