import { test, expect } from '@playwright/test';

test('verify wizard UI state propagation to backend', async ({ page }) => {
    // Navigate to the home page
    await page.goto('/');

    // Login
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Login")');

    // Wait for the Dashboard
    await expect(page.locator('text="Welcome"')).toBeVisible();

    // Start setup wizard
    await page.click('button:has-text("Start Setup")');

    // Wait for Wizard to show
    await expect(page.locator('text="Setup Wizard"')).toBeVisible();

    // 0: Welcome -> 1
    await page.click('button:has-text("Next")');

    // 1: Business Type -> 2
    // Choose "Online Store"
    await page.click('text="Online Store"');
    await page.click('button:has-text("Next")');

    // 2: Company Info -> 3
    await page.fill('input[placeholder="What is your business called?"]', 'My Awesome Store');
    await page.click('button:has-text("Generate Description")');
    await page.waitForTimeout(1000); // Wait for mock gen
    await page.click('button:has-text("Next")');

    // 3: Selling Categories -> 4
    await page.check('text="Physical Products"');
    await page.click('button:has-text("Next")');

    // 4: First Product -> 5
    await page.fill('input[placeholder="What is the name of this product?"]', 'Awesome Product');
    await page.fill('input[placeholder="0.00"]', '19.99');
    await page.click('button:has-text("Next")');

    // 5: Payments -> 6
    await page.click('text="Online"');
    await page.click('button:has-text("Next")');

    // 6: Theme -> 7
    await page.click('text="Modern"');
    await page.click('button:has-text("Next")');

    // 7: Domain -> 8
    await page.click('text="Get a free sub-domain"');
    await page.click('button:has-text("Next")');

    // 8: Admin Info -> 9
    await page.fill('input[placeholder="Your Full Name"]', 'Jane Doe');
    await page.fill('input[placeholder="your@email.com"]', 'jane@example.com');
    await page.fill('input[placeholder="Create a strong password"]', 'securepass123');
    await page.click('button:has-text("Review & Launch")');

    // 9: Launch View
    await expect(page.locator('text="Almost there"')).toBeVisible({ timeout: 5000 });

    // We expect "Launch!" button. When clicked, it hits the `on_launch` handler in Rust.
    await page.click('button:has-text("Launch!")');

    // Verify it transitions away from launching after mock async backend setup
    // And lands on the success step or dashboard
    await expect(page.locator('text="Onboarding Complete!"')).toBeVisible({ timeout: 10000 });

    // Check that we can navigate back to dashboard
    await page.click('button:has-text("Go to Dashboard")');
    await expect(page.locator('text="Welcome"')).toBeVisible();
});

test('verify wizard AI agent configuration', async ({ page }) => {
    // Navigate to the home page
    await page.goto('/');

    // Login
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Login")');

    // Navigate to agent settings
    await page.click('text="Manage Agents"');

    await expect(page.locator('text="Agent Configuration"')).toBeVisible();

    // Toggle capabilities
    await page.click('text="Customer Support"');
    await page.check('text="Can Reply to Messages"');

    // Save
    await page.click('button:has-text("Activate Agent")');

    await expect(page.locator('text="Agent Activated"')).toBeVisible();
});

test('verify wizard prompt tuning', async ({ page }) => {
    // Navigate to the home page
    await page.goto('/');

    // Login
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Login")');

    // Navigate to agent settings
    await page.click('text="Manage Agents"');

    // Open Prompt Tuning
    await page.click('button:has-text("Tune Prompt")');

    await expect(page.locator('text="Prompt Tuning"')).toBeVisible();

    // Select Tone
    await page.click('text="Friendly"');

    // Toggle Focus
    await page.check('text="Only discuss business"');

    // Save
    await page.click('button:has-text("Save Configuration")');

    await expect(page.locator('text="Configuration Saved"')).toBeVisible();
});

test('verify grow business suggestions', async ({ page }) => {
    await page.goto('/');
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Login")');

    await page.click('button:has-text("Grow Business")');
    await expect(page.locator('text="Actionable Insights"')).toBeVisible();

    await page.click('button:has-text("Dismiss")');
    // Ensure modal closes
    await expect(page.locator('text="Actionable Insights"')).toBeHidden();
});

test('verify website builder flow', async ({ page }) => {
    await page.goto('/');
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Login")');

    await page.click('button:has-text("Website Builder")');

    // Step 0 -> Step 1
    await page.click('button:has-text("Start Building")');

    // Step 1: Select Template
    await page.click('text="Modern"');
    await page.click('button:has-text("Next")');

    // Step 2: Branding
    await page.click('button:has-text("Generate Logo")');
    await page.waitForTimeout(1000);
    await page.click('button:has-text("Next")');

    // Step 3: Publish
    await page.click('button:has-text("Publish Site")');

    await expect(page.locator('text="Site Published!"')).toBeVisible({ timeout: 5000 });
});
