import { test, expect } from '@playwright/test';

test.describe('User Launch Business Flow (Issue #7977)', () => {
  test('Complete onboarding flow on mobile viewport', async ({ page }) => {
    // 1. Mobile-first UI (375px baseline)
    await page.setViewportSize({ width: 375, height: 812 });

    // 2. Start at the business setup entry point (mock path for testing)
    await page.goto('/setup');

    // Step 0: Welcome
    await expect(page.locator('text=Welcome to OneHumanCorp')).toBeVisible();
    await page.click('text=Next');

    // Step 1: Business Details
    await page.fill('input[placeholder="Business Name"]', "Maya's Custom Cakes");
    await page.fill('input[placeholder="Describe your business in a few words..."]', "Custom bakery offering vegan cakes via Instagram");
    await page.click('text=Next');

    // Step 2: Main Business Type (Polymorphic support)
    await page.click('text=Physical Products'); // Select type
    await page.click('text=Next');

    // Step 3: Platform Connections
    // Optionally check boxes or just proceed
    await page.click('text=Next');

    // Step 4: Add Team Members
    // AI Sales is checked by default
    await page.click('text=Next');

    // Step 5: Domain
    await page.fill('input[placeholder="Domain Name"]', "mayascakes");

    // Final Launch
    await page.click('text=Launch My AI Team');

    // Verify redirect or state change indicating success
    // Wait for the Dashboard to appear
    await expect(page.locator('text=Dashboard')).toBeVisible();
    await expect(page.locator('text=Active Agents')).toBeVisible();
    await expect(page.locator('text=Company Structure')).toBeVisible();
  });
});
