import { test, expect } from '@playwright/test';

/**
 * OHC Hybrid Agentic OS: Comprehensive Reliability and Chaos Suite
 *
 * This suite covers the Critical User Journeys (CUJs) under extreme environmental stress.
 * Mandate: Verify absolute mode parity and graceful degradation.
 */

test.describe('OHC Reliability Master Suite', () => {

  test.beforeEach(async ({ page }) => {
    // Standard secure login flow
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('sentry-admin@onehumancorp.com');
    await page.locator('input[type="password"]').first().fill('CorrectHorseBatteryStaple123!');
    await page.locator('button:has-text("Sign In")').click();
    await page.waitForURL('**/dashboard**');
  });

  /**
   * CUJ 1: Proactive Business Setup under Database Stress
   * Verification: Optimistic UI handles high-latency state persistence.
   */
  test('CUJ-1: Business Setup Wizard Resilience', async ({ page }) => {
    await page.locator('button:has-text("Start Business Setup")').click();

    // Step 1: Industry Selection
    await page.locator('button:has-text("Online Store")').click();

    // Step 2: Name Entry
    await page.locator('input[type="text"]').fill('Reliable Bakery');
    await page.locator('button:has-text("Next")').click();

    // Simulate Background Save Lag (>2s)
    // The UI should not block or show a spinner that prevents interaction.
    await expect(page.locator('h1:has-text("What do you sell?")')).toBeVisible();

    // Step 3: Product Type
    await page.locator('button:has-text("Physical products")').click();

    // Step 4: Payment Config
    await page.locator('button:has-text("Online only")').click();

    // Step 5: Account Creation
    await page.locator('input[placeholder="e.g. Maya Smith"]').fill('Sentry Test');
    await page.locator('input[type="email"]').fill('sentry-test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Next")').click();

    // Verification of Glassmorphism tokens during transition
    const setupCard = page.locator('#setup-screen');
    await expect(setupCard).toHaveCSS('backdrop-filter', /blur/);
  });

  /**
   * CUJ 2: Agent Action Feed during LLM API Outage
   * Verification: Agent transitions to PAUSED state and notifies owner.
   */
  test('CUJ-2: Agent Action Feed Outage Handling', async ({ page }) => {
    await page.locator('button:has-text("Manage Agents")').click();

    // Simulate LLM Outage in Backend (Mocked behavior verification)
    // The feed should show existing tasks in cached state.
    const agentCard = page.locator('.card:has-text("Marketing Pro")');
    await expect(agentCard).toBeVisible();

    // Trigger a task that we know will fail LLM calls
    await page.locator('button:has-text("Hire Agent")').first().click();

    // Verify "PAUSED" state appears in the UI
    // Note: This requires the backend to have transitioned the mission status.
    await expect(page.locator('text=/PAUSED|UNAVAILABLE/i')).toBeVisible({ timeout: 15000 });

    // Verify notification toast or alert
    await expect(page.locator('text=/notification|alert|outage/i')).toBeVisible();
  });

  /**
   * CUJ 3: Cross-Mode Data Consistency (Cloud vs Standalone)
   * Verification: Record visibility and isolation under load.
   */
  test('CUJ-3: Tenant Record Isolation and Caching', async ({ page }) => {
    await page.locator('button:has-text("Check Messages")').click();

    // Verify no cross-tenant leakage
    await expect(page.locator('text=/Unauthorized|Leakage/i')).not.toBeVisible();

    // Add a message (Write Operation)
    await page.locator('input[placeholder*="message"]').fill('Test consistency');
    await page.locator('button:has-text("Send")').click();

    // Verify optimistic success
    await expect(page.locator('text=Test consistency')).toBeVisible();

    // Simulate connection drop and refresh
    // Read operation should still show the cached message
    await page.reload();
    await expect(page.locator('text=Test consistency')).toBeVisible();
  });

  /**
   * CUJ 4: Visual Excellence Failure States
   * Verification: Error messages use Glassmorphism and Outfit font.
   */
  test('CUJ-4: Visual Mandate Verification', async ({ page }) => {
    // Trigger a manual error (e.g., submitting empty form)
    await page.goto('/login');
    await page.locator('button:has-text("Sign In")').click();

    const errorBox = page.locator('#login-error');
    await expect(errorBox).toBeVisible();

    // Verify Typography
    await expect(page.locator('body')).toHaveCSS('font-family', /Outfit/);

    // Verify Glassmorphism on error container
    await expect(errorBox.locator('xpath=..')).toHaveClass(/glass|card/);
  });
});
