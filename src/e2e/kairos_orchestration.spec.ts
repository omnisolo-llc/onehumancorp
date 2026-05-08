import { test, expect } from '@playwright/test';

test.describe('KAIROS Orchestration Walkthrough End-to-End Flow Validation', () => {
  test.use({ viewport: { width: 375, height: 800 } });

  test('Flow 1: User navigates to Automations tour from Quick Actions', async ({ page }) => {
    // 1. Start from the home page after user login
    await page.goto('/login');
    await page.fill('input[type="email"]', 'grandma@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Sign In")');
    await page.waitForURL('**/*');

    await expect(page.locator('text=My Business').first()).toBeVisible();

    // 2. Open Automate Work Tour
    await page.click('button:has-text("Menu")');
    const automateBtn = page.locator('button:has-text("Automate Work Tour")');
    if (await automateBtn.isVisible()) {
        await automateBtn.click();
    } else {
        await page.click('button:has-text("Automations")');
    }

    // 3. Verify KAIROS Orchestration Walkthrough screen is shown
    await expect(page.locator('text=How Your Helpers Work Together').first()).toBeVisible();

    // 4. Verify step 1
    await expect(page.locator('text=The Helper System')).toBeVisible();
    await expect(page.locator('text=Your helpers work together using a simple three-part system: memory, messaging, and a shared to-do list.')).toBeVisible();

    // 5. Navigate to step 2
    await page.click('button:has-text("Next Step")');
    await expect(page.locator('text=1. Shared To-Do List')).toBeVisible();

    // 6. Navigate to step 3
    await page.click('button:has-text("Next Step")');
    await expect(page.locator('text=2. Instant Messaging')).toBeVisible();

    // 7. Navigate to step 4
    await page.click('button:has-text("Next Step")');
    await expect(page.locator('text=3. Long-Term Memory')).toBeVisible();

    // 8. Click Done
    await page.click('button:has-text("Done")');

    // 9. Verify we are back
    await expect(page.locator('text=How Your Helpers Work Together').first()).not.toBeVisible();
  });
});
