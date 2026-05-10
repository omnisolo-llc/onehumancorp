import { test, expect } from '@playwright/test';

test.describe('Wizard E2E Comprehensive Flows', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://127.0.0.1:18789');
  });

  test('Business Setup Wizard Flow', async ({ page }) => {
    await page.fill('input[placeholder="Email"]', 'test@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('text="Sign In"');

    // Wait for the UI to load
    await expect(page.locator('text="Next →"').first()).toBeVisible({ timeout: 10000 });

    // Proceed through the Business Setup Wizard
    await page.click('text="Next →"');

    // Select Business Type
    await expect(page.locator('text="Online Store"').first()).toBeVisible({ timeout: 10000 });
    await page.click('text="Online Store"');

    // Enter Company Info
    await expect(page.locator('input[placeholder="e.g. Maya\'s Bakery"]').first()).toBeVisible({ timeout: 10000 });
    await page.fill('input[placeholder="e.g. Maya\'s Bakery"]', 'Acme Corp');
    await page.fill('input[placeholder="e.g. Delicious vegan cakes"]', 'Great stuff');
    await page.click('text="Next →"');

    // What to sell
    await expect(page.locator('text="📦 Physical products"').first()).toBeVisible({ timeout: 10000 });
    await page.click('text="📦 Physical products"');
    await page.click('text="Next →"');

    // Payment Pref
    await expect(page.locator('text="💳 Online only"').first()).toBeVisible({ timeout: 10000 });
    await page.click('text="💳 Online only"');

    // Admin Info
    await expect(page.locator('input[placeholder="e.g. Maya"]').first()).toBeVisible({ timeout: 10000 });
    await page.fill('input[placeholder="e.g. Maya"]', 'John Doe');
    await page.fill('input[placeholder="e.g. maya@example.com"]', 'john@example.com');
    await page.fill('input[placeholder="Choose a strong password"]', 'pass');
    await page.click('text="Next →"');

    // Template
    await expect(page.locator('text="✨ Modern"').first()).toBeVisible({ timeout: 10000 });
    await page.click('text="✨ Modern"');
    await page.click('text="Next →"');

    // Product
    await expect(page.locator('input[placeholder="e.g. Vegan Chocolate Cake"]').first()).toBeVisible({ timeout: 10000 });
    await page.fill('input[placeholder="e.g. Vegan Chocolate Cake"]', 'Super Thing');
    await page.fill('input[placeholder="e.g. 45.00"]', '100');
    await page.click('text="Next →"');

    // Domain
    await expect(page.locator('text="🌍 Use my own domain"').first()).toBeVisible({ timeout: 10000 });
    await page.click('text="🌍 Use my own domain"');
    await page.click('text="Next →"');

    // Launch
    await expect(page.locator('text="Launch My Business →"').first()).toBeVisible({ timeout: 10000 });
    await page.click('text="Launch My Business →"');

    await expect(page.locator('text="Your live storefront!"')).toBeVisible({ timeout: 10000 });
    await page.click('text="Continue to Dashboard →"');
    await expect(page.locator('text="Dashboard"')).toBeVisible();
  });

  test('Website Builder Onboarding Flow', async ({ page }) => {
    await page.fill('input[placeholder="Email"]', 'test@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('text="Sign In"');

    // Check if dashboard or skip straight to url
    const buildBtn = page.locator('text="Build My Website"');
    if (await buildBtn.isVisible()) {
      await buildBtn.click();
    } else {
      await page.goto('http://127.0.0.1:18789/website-builder');
    }

    // Select Template
    await expect(page.locator('text="✨ Modern"').first()).toBeVisible({ timeout: 10000 });
    await page.click('text="✨ Modern"');
    await page.click('text="Next →"');

    // Brand Colors & Logo
    await expect(page.locator('text="🔴 Bold Red"').first()).toBeVisible({ timeout: 10000 });
    await page.click('text="🔴 Bold Red"');
    await page.click('text="Next →"');

    // Product
    await expect(page.locator('input[placeholder="e.g. Custom Birthday Cake"]').first()).toBeVisible({ timeout: 10000 });
    await page.fill('input[placeholder="e.g. Custom Birthday Cake"]', 'Service Test');
    await page.fill('input[placeholder="e.g. 50.00"]', '20');
    await page.click('text="Next →"');

    // Connect Domain
    await expect(page.locator('text="🌍 Use my own domain"').first()).toBeVisible({ timeout: 10000 });
    await page.click('text="🌍 Use my own domain"');
    await page.click('text="Next →"');

    // Go Live
    await expect(page.locator('text="Publish"').first()).toBeVisible({ timeout: 10000 });
    await page.click('text="Publish"');
    await expect(page.locator('text="Publishing Site..."')).toBeVisible({ timeout: 10000 });
  });

  test('Agent Config Wizard Flow', async ({ page }) => {
    await page.fill('input[placeholder="Email"]', 'test@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('text="Sign In"');

    const manageTeam = page.locator('text="Manage my AI team"');
    if (await manageTeam.isVisible()) {
      await manageTeam.click();
    } else {
      await page.goto('http://127.0.0.1:18789/agents');
    }

    await expect(page.locator('text="Hire Helper"').first()).toBeVisible({ timeout: 10000 });
    await page.click('text="Hire Helper"');

    // Select Agent
    await expect(page.locator('text="Order Manager"').first()).toBeVisible({ timeout: 10000 });
    await page.click('text="Order Manager"');
    await page.click('text="Next"');

    // Capabilities
    await expect(page.locator('text="Reply to customer messages"').first()).toBeVisible({ timeout: 10000 });
    await page.click('text="Reply to customer messages"');
    await page.click('text="Next"');

    // Schedule
    await expect(page.locator('text="Next"').first()).toBeVisible({ timeout: 10000 });
    await page.click('text="Next"');

    // Activate
    await expect(page.locator('text="Activate"').first()).toBeVisible({ timeout: 10000 });
    await page.click('text="Activate"');
    await expect(page.locator('text="Helper Activated ✓"')).toBeVisible({ timeout: 5000 });
  });

  test('Prompt Tuning Flow', async ({ page }) => {
    await page.fill('input[placeholder="Email"]', 'test@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('text="Sign In"');

    await page.goto('http://127.0.0.1:18789/agents');
    // Assuming there is an agent listed
    await expect(page.locator('text="Tune"').first()).toBeVisible({ timeout: 10000 });
    await page.click('text="Tune"');

    // Tone
    await expect(page.locator('text="Friendly & Warm"').first()).toBeVisible({ timeout: 10000 });
    await page.click('text="Friendly & Warm"');
    await page.click('text="Next"');

    // Focus
    await expect(page.locator('text="🎯 Only discuss business"').first()).toBeVisible({ timeout: 10000 });
    await page.click('text="🎯 Only discuss business"');
    await page.click('text="Next"');

    // Examples
    await expect(page.locator('text="Next"').first()).toBeVisible({ timeout: 10000 });
    await page.click('text="Next"');

    // Save
    await expect(page.locator('text="Save Prompt"').first()).toBeVisible({ timeout: 10000 });
    await page.click('text="Save Prompt"');
  });

  test('Grow Business Flow', async ({ page }) => {
    await page.fill('input[placeholder="Email"]', 'test@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('text="Sign In"');

    // From dashboard, grow business button
    const growBtn = page.locator('text="Grow My Business"');
    if (await growBtn.isVisible()) {
      await growBtn.click();
    } else {
      await page.goto('http://127.0.0.1:18789/grow');
    }

    // Select strategy
    await expect(page.locator('text="📦 Add 5 more products"').first()).toBeVisible({ timeout: 10000 });
    await page.click('text="📦 Add 5 more products"');
    await page.click('text="Next →"');

    // Confirm
    await expect(page.locator('text="Launch Strategy"').first()).toBeVisible({ timeout: 10000 });
    await page.click('text="Launch Strategy"');

    // Success
    await expect(page.locator('text="🚀 Agents Engaged!"')).toBeVisible({ timeout: 5000 });
    await page.click('text="Return to Dashboard"');
  });

  test('Fix Issue Wizard Flow', async ({ page }) => {
    await page.fill('input[placeholder="Email"]', 'test@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('text="Sign In"');

    await page.goto('http://127.0.0.1:18789/agents');
    // Click Fix button if available
    const fixBtn = page.locator('text="Fix"').first();
    if (await fixBtn.isVisible()) {
      await fixBtn.click();
    } else {
      await page.goto('http://127.0.0.1:18789/wizard');
    }

    // Step 0 -> Step 1 -> Step 2
    await expect(page.locator('text="View Suggested Fix"').first()).toBeVisible({ timeout: 10000 });
    await page.click('text="View Suggested Fix"'); // to step 1
    await expect(page.locator('text="Refresh & Reconnect"').first()).toBeVisible({ timeout: 10000 });
    await page.click('text="Refresh & Reconnect"'); // to step 2
    await expect(page.locator('text="Apply Fix ✓"').first()).toBeVisible({ timeout: 10000 });
    await page.click('text="Apply Fix ✓"');
  });

});
