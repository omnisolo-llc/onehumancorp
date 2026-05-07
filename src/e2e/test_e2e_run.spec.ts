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

test('verify login form error message UX wrap behavior', async ({ page }) => {
    await page.goto('/');

    // Trigger an error to see if error message is displayed
    await page.fill('input[type="email"]', 'invalid@example.com');
    await page.fill('input[type="password"]', 'wrong');
    await page.click('button:has-text("Sign In")');

    // Assume an error message like 'Invalid email or password' appears
    await expect(page.locator('text="Invalid"')).toBeVisible();

    // Get bounding box of the error message to verify it wraps and doesn't exceed screen width
    const errorLocator = page.locator('text="Invalid"');
    const box = await errorLocator.boundingBox();
    expect(box).not.toBeNull();
    if (box) {
      expect(box.width).toBeLessThanOrEqual(400); // login card max width
    }
});

test('verify app settings toggle', async ({ page }) => {
    await page.goto('/');

    // We changed 'App Settings' to 'App Settings'
    await page.click('button:has-text("App Settings")');

    // Expect the settings to be shown
    await expect(page.locator('text="Settings"')).toBeVisible();
});

test('verify sign up and sign in toggle', async ({ page }) => {
    await page.goto('/');

    // Ensure we start at Sign In
    await expect(page.locator('button:has-text("Sign In")')).toBeVisible();

    // Click Don't have an account
    await page.click('button:has-text("Don\'t have an account? Sign Up")');

    // Ensure we are at Sign Up
    await expect(page.locator('button:has-text("Sign Up")')).toBeVisible();
    await expect(page.locator('button:has-text("Already have an account? Sign In")')).toBeVisible();

    // Toggle back
    await page.click('button:has-text("Already have an account? Sign In")');

    // Ensure we are back at Sign In
    await expect(page.locator('button:has-text("Sign In")')).toBeVisible();
});

test('verify password toggle', async ({ page }) => {
    await page.goto('/');

    // Fill password
    await page.fill('input[type="password"]', 'secretpassword');

    // Click Show
    await page.click('button:has-text("Show")');

    // Check if input type is text
    const inputType = await page.getAttribute('input[value="secretpassword"]', 'type');
    expect(inputType).toBe('text');

    // Click Hide
    await page.click('button:has-text("Hide")');

    // Check if input type is password
    const inputTypeAfter = await page.getAttribute('input[value="secretpassword"]', 'type');
    expect(inputTypeAfter).toBe('password');
});

test('verify login empty submission', async ({ page }) => {
    await page.goto('/');

    // Submit empty form
    await page.click('button:has-text("Sign In")');

    // Wait for validation error
    await expect(page.locator('text="Username cannot be empty"')).toBeVisible();
});

test('verify checklist flow and integration', async ({ page }) => {
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

    // Proceed to last step in wizard to see the checklist
    // Step 0 -> Step 1
    await page.click('button:has-text("Next")');
    // Step 1: Business Type -> 2
    await page.click('text="Online Store"');
    await page.click('button:has-text("Next")');
    // Step 2: Company Info -> 3
    await page.fill('input[placeholder="What is your business called?"]', 'Checklist Store');
    await page.click('button:has-text("Generate Description")');
    await page.waitForTimeout(1000);
    await page.click('button:has-text("Next")');
    // Step 3: Selling Categories -> 4
    await page.check('text="Physical Products"');
    await page.click('button:has-text("Next")');
    // Step 4: First Product -> 5
    await page.fill('input[placeholder="What is the name of this product?"]', 'Prod');
    await page.fill('input[placeholder="0.00"]', '10');
    await page.click('button:has-text("Next")');
    // Step 5: Payments -> 6
    await page.click('text="Online"');
    await page.click('button:has-text("Next")');
    // Step 6: Theme -> 7
    await page.click('text="Modern"');
    await page.click('button:has-text("Next")');
    // Step 7: Domain -> 8
    await page.click('text="Get a free sub-domain"');
    await page.click('button:has-text("Next")');
    // Step 8: Admin Info -> 9
    await page.fill('input[placeholder="Your Full Name"]', 'Jane Doe');
    await page.fill('input[placeholder="your@email.com"]', 'jane@example.com');
    await page.fill('input[placeholder="Create a strong password"]', 'securepass123');
    await page.click('button:has-text("Review & Launch")');
    // 9: Launch View
    await expect(page.locator('text="Almost there"')).toBeVisible({ timeout: 5000 });
    await page.click('button:has-text("Launch!")');
    await expect(page.locator('text="Onboarding Complete!"')).toBeVisible({ timeout: 10000 });

    if (await page.locator('text="Continue to Setup Checklist"').isVisible()) {
        await page.click('button:has-text("Continue to Setup Checklist")');
    } else {
        await page.click('button:has-text("Go to Dashboard")');
    }

    // Verify Welcome Checklist shows up
    await expect(page.locator('text="Welcome Checklist"')).toBeVisible({ timeout: 5000 });

    // Should route to WebsiteBuilder
    await page.click('text="Add 3 more products"');
    await expect(page.locator('text="Website Builder"')).toBeVisible();
});

test('verify checklist connects instagram routing', async ({ page }) => {
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

    // Proceed to last step in wizard
    await page.click('button:has-text("Next")');
    await page.click('text="Online Store"');
    await page.click('button:has-text("Next")');
    await page.fill('input[placeholder="What is your business called?"]', 'Checklist Store');
    await page.click('button:has-text("Generate Description")');
    await page.waitForTimeout(1000);
    await page.click('button:has-text("Next")');
    await page.check('text="Physical Products"');
    await page.click('button:has-text("Next")');
    await page.fill('input[placeholder="What is the name of this product?"]', 'Prod');
    await page.fill('input[placeholder="0.00"]', '10');
    await page.click('button:has-text("Next")');
    await page.click('text="Online"');
    await page.click('button:has-text("Next")');
    await page.click('text="Modern"');
    await page.click('button:has-text("Next")');
    await page.click('text="Get a free sub-domain"');
    await page.click('button:has-text("Next")');
    await page.fill('input[placeholder="Your Full Name"]', 'Jane Doe');
    await page.fill('input[placeholder="your@email.com"]', 'jane@example.com');
    await page.fill('input[placeholder="Create a strong password"]', 'securepass123');
    await page.click('button:has-text("Review & Launch")');
    await expect(page.locator('text="Almost there"')).toBeVisible({ timeout: 5000 });
    await page.click('button:has-text("Launch!")');
    await expect(page.locator('text="Onboarding Complete!"')).toBeVisible({ timeout: 10000 });

    if (await page.locator('text="Continue to Setup Checklist"').isVisible()) {
        await page.click('button:has-text("Continue to Setup Checklist")');
    } else {
        await page.click('button:has-text("Go to Dashboard")');
    }

    await expect(page.locator('text="Welcome Checklist"')).toBeVisible({ timeout: 5000 });

    // Check routing
    await page.click('text="Connect Instagram"');
    await expect(page.locator('text="Integrations"')).toBeVisible();
});

test('verify checklist share link routing', async ({ page }) => {
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

    // Proceed to last step in wizard
    await page.click('button:has-text("Next")');
    await page.click('text="Online Store"');
    await page.click('button:has-text("Next")');
    await page.fill('input[placeholder="What is your business called?"]', 'Checklist Store');
    await page.click('button:has-text("Generate Description")');
    await page.waitForTimeout(1000);
    await page.click('button:has-text("Next")');
    await page.check('text="Physical Products"');
    await page.click('button:has-text("Next")');
    await page.fill('input[placeholder="What is the name of this product?"]', 'Prod');
    await page.fill('input[placeholder="0.00"]', '10');
    await page.click('button:has-text("Next")');
    await page.click('text="Online"');
    await page.click('button:has-text("Next")');
    await page.click('text="Modern"');
    await page.click('button:has-text("Next")');
    await page.click('text="Get a free sub-domain"');
    await page.click('button:has-text("Next")');
    await page.fill('input[placeholder="Your Full Name"]', 'Jane Doe');
    await page.fill('input[placeholder="your@email.com"]', 'jane@example.com');
    await page.fill('input[placeholder="Create a strong password"]', 'securepass123');
    await page.click('button:has-text("Review & Launch")');
    await expect(page.locator('text="Almost there"')).toBeVisible({ timeout: 5000 });
    await page.click('button:has-text("Launch!")');
    await expect(page.locator('text="Onboarding Complete!"')).toBeVisible({ timeout: 10000 });

    if (await page.locator('text="Continue to Setup Checklist"').isVisible()) {
        await page.click('button:has-text("Continue to Setup Checklist")');
    } else {
        await page.click('button:has-text("Go to Dashboard")');
    }

    await expect(page.locator('text="Welcome Checklist"')).toBeVisible({ timeout: 5000 });

    // Check routing
    await page.click('text="Share your link"');
    await expect(page.locator('text="Referrals"')).toBeVisible();
});

test('verify checklist fully completed state', async ({ page }) => {
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

    // Proceed to last step in wizard
    await page.click('button:has-text("Next")');
    await page.click('text="Online Store"');
    await page.click('button:has-text("Next")');
    await page.fill('input[placeholder="What is your business called?"]', 'Checklist Store');
    await page.click('button:has-text("Generate Description")');
    await page.waitForTimeout(1000);
    await page.click('button:has-text("Next")');
    await page.check('text="Physical Products"');
    await page.click('button:has-text("Next")');
    await page.fill('input[placeholder="What is the name of this product?"]', 'Prod');
    await page.fill('input[placeholder="0.00"]', '10');
    await page.click('button:has-text("Next")');
    await page.click('text="Online"');
    await page.click('button:has-text("Next")');
    await page.click('text="Modern"');
    await page.click('button:has-text("Next")');
    await page.click('text="Get a free sub-domain"');
    await page.click('button:has-text("Next")');
    await page.fill('input[placeholder="Your Full Name"]', 'Jane Doe');
    await page.fill('input[placeholder="your@email.com"]', 'jane@example.com');
    await page.fill('input[placeholder="Create a strong password"]', 'securepass123');
    await page.click('button:has-text("Review & Launch")');
    await expect(page.locator('text="Almost there"')).toBeVisible({ timeout: 5000 });
    await page.click('button:has-text("Launch!")');
    await expect(page.locator('text="Onboarding Complete!"')).toBeVisible({ timeout: 10000 });

    if (await page.locator('text="Continue to Setup Checklist"').isVisible()) {
        await page.click('button:has-text("Continue to Setup Checklist")');
    } else {
        await page.click('button:has-text("Go to Dashboard")');
    }

    await expect(page.locator('text="Welcome Checklist"')).toBeVisible({ timeout: 5000 });

    const checkboxes = page.locator('input[type="checkbox"]');
    const count = await checkboxes.count();
    for (let i = 0; i < count; i++) {
        await checkboxes.nth(i).check();
    }

    await expect(page.locator('text=/Congratulations/i')).toBeVisible({ timeout: 5000 });
});

test('verify checklist completion progress', async ({ page }) => {
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

    // Proceed to last step in wizard
    await page.click('button:has-text("Next")');
    await page.click('text="Online Store"');
    await page.click('button:has-text("Next")');
    await page.fill('input[placeholder="What is your business called?"]', 'Checklist Store');
    await page.click('button:has-text("Generate Description")');
    await page.waitForTimeout(1000);
    await page.click('button:has-text("Next")');
    await page.check('text="Physical Products"');
    await page.click('button:has-text("Next")');
    await page.fill('input[placeholder="What is the name of this product?"]', 'Prod');
    await page.fill('input[placeholder="0.00"]', '10');
    await page.click('button:has-text("Next")');
    await page.click('text="Online"');
    await page.click('button:has-text("Next")');
    await page.click('text="Modern"');
    await page.click('button:has-text("Next")');
    await page.click('text="Get a free sub-domain"');
    await page.click('button:has-text("Next")');
    await page.fill('input[placeholder="Your Full Name"]', 'Jane Doe');
    await page.fill('input[placeholder="your@email.com"]', 'jane@example.com');
    await page.fill('input[placeholder="Create a strong password"]', 'securepass123');
    await page.click('button:has-text("Review & Launch")');
    await expect(page.locator('text="Almost there"')).toBeVisible({ timeout: 5000 });
    await page.click('button:has-text("Launch!")');
    await expect(page.locator('text="Onboarding Complete!"')).toBeVisible({ timeout: 10000 });

    if (await page.locator('text="Continue to Setup Checklist"').isVisible()) {
        await page.click('button:has-text("Continue to Setup Checklist")');
    } else {
        await page.click('button:has-text("Go to Dashboard")');
    }

    await expect(page.locator('text="Welcome Checklist"')).toBeVisible({ timeout: 5000 });

    // Verify checklist items
    const checkboxes = page.locator('input[type="checkbox"]');
    if (await checkboxes.count() > 0) {
        // Mark first as complete
        await checkboxes.nth(0).check();
        await expect(page.locator('text=/25%/i')).toBeVisible({ timeout: 5000 });

        // Mark second as complete
        await checkboxes.nth(1).check();
        await expect(page.locator('text=/50%/i')).toBeVisible({ timeout: 5000 });
    }
});
