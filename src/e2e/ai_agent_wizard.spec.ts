import { test, expect } from '@playwright/test';

test.describe('AI Agent Configuration Wizard - E2E Flows', () => {
  test.beforeEach(async ({ page }) => {
    // 1. Start from home page login
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('grandma@example.com');
    await page.locator('input[type="password"]').fill('easy123');
    await page.locator('button:has-text("Login")').click();

    // Navigate to Dashboard then to Agents
    await page.waitForURL('**/dashboard');
    await page.locator('button:has-text("Manage my AI team"), a:has-text("AI Team")').first().click();
    await page.waitForURL('**/agents');
  });

  test('Flow 1: Hire Customer Support agent with replies enabled', async ({ page }) => {
    await page.locator('button:has-text("Hire Agent")').click();
    await expect(page.locator('text=Configure Helper')).toBeVisible();

    // Step 0: Agent Gallery Card Grid
    await page.locator('text=Customer Support').first().click();
    await page.locator('button:has-text("Next")').click();

    // Step 1: Capabilities
    const replyToggle = page.locator('text=Reply to customer messages');
    await replyToggle.click();
    await page.locator('button:has-text("Next")').click();

    // Step 2: Schedule/frequency
    await expect(page.locator('text=How often should this agent work?')).toBeVisible();
    await page.locator('button:has-text("Next")').click();

    // Step 3: Review & Activate
    await expect(page.locator('text=Review & Activate')).toBeVisible();
    await expect(page.locator('text=Helper: Customer Support')).toBeVisible();
    await page.locator('button:has-text("Activate")').click();

    // Toast confirmation
    await expect(page.locator('text=Helper Activated ✓')).toBeVisible();
  });

  test('Flow 2: Hire Social Media Manager with daily frequency', async ({ page }) => {
    await page.locator('button:has-text("Hire Agent")').click();

    // Step 0
    await page.locator('text=Social Media Manager').first().click();
    await page.locator('button:has-text("Next")').click();

    // Step 1
    await page.locator('button:has-text("Next")').click();

    // Step 2
    await page.locator('button:has-text("Next")').click();

    // Step 3
    await page.locator('button:has-text("Activate")').click();
    await expect(page.locator('text=Helper Activated ✓')).toBeVisible();
  });

  test('Flow 3: Prompt Tuning - Friendly tone for Customer Support', async ({ page }) => {
    // Tune an existing agent
    await page.locator('button:has-text("Tune this agent")').first().click();

    // Step 0: Tone
    await expect(page.locator('text=Tone of Voice')).toBeVisible();
    await page.locator('text=Friendly & Warm').click();
    await page.locator('button:has-text("Next")').click();

    // Step 1: Focus
    await expect(page.locator('text=Domain focus')).toBeVisible();
    await page.locator('text=Only discuss business').click();
    await page.locator('button:has-text("Next")').click();

    // Step 2: Examples
    await page.locator('button:has-text("Next")').click();

    // Step 3: Review
    await page.locator('button:has-text("Save")').click();
    await expect(page.locator('text=Your agent has been updated ✓')).toBeVisible();
  });

  test('Flow 4: Prompt Tuning - Professional tone with Competitor avoidance', async ({ page }) => {
    await page.locator('button:has-text("Tune this agent")').first().click();

    // Step 0
    await page.locator('text=Professional').click();
    await page.locator('button:has-text("Next")').click();

    // Step 1
    await page.locator('text=Avoid competitors').click();
    await page.locator('button:has-text("Next")').click();

    // Step 2
    await page.locator('button:has-text("Next")').click();

    // Step 3
    await page.locator('button:has-text("Save")').click();
    await expect(page.locator('text=Your agent has been updated ✓')).toBeVisible();
  });

  test('Flow 5: Verify Mobile Layout of Agent Gallery (375px viewport)', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });

    await page.locator('button:has-text("Hire Agent")').click();
    await expect(page.locator('text=Configure Helper')).toBeVisible();

    // The Customer Support card should be visible and not clipped
    const supportCard = page.locator('text=Customer Support').first();
    await expect(supportCard).toBeVisible();

    // The Email Marketer card should also be visible (in the stack)
    const emailCard = page.locator('text=Email Marketer').first();
    await expect(emailCard).toBeVisible();
  });
});
