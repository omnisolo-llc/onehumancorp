import { test, expect } from '@playwright/test';

test.describe('Onboarding and Cross-Device Resume Flow', () => {
  test('User completes onboarding and state resumes correctly', async ({ page }) => {
    // 1. Visit the app and log in
    await page.goto('/');

    await page.fill('input[placeholder="Email or Username"]', 'test@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('text="Sign In"');

    // 2. Validate Wizard shows Welcome Step
    // Some apps navigate automatically, wait for the app URL to settle
    await page.waitForURL('**/*');
    await expect(page.locator('text="Welcome to OneHumanCorp"').first()).toBeVisible();

    // 3. Complete Step 1: Click "Get Started ->"
    await page.click('text="Get Started →"');
    await expect(page.locator('text="What kind of business"').first()).toBeVisible();

    // 4. Select Business Type (e.g. Freelancer)
    await page.click('text="Freelancer"');

    // 5. Fill Business Name
    await expect(page.locator('text="What is your business called?"').first()).toBeVisible();
    // Assuming the Slint bridge maps the input field directly or we find the generic input
    await page.fill('input[placeholder="e.g. Acme Corp"]', 'Acme E2E Setup');
    await page.click('text="Next →"');

    // 6. Simulate Cross-Device Resume: Reload the page
    await page.reload();

    // We expect the user to be re-prompted to log in or automatically be logged in depending on session
    // For this test, we re-authenticate
    await page.fill('input[placeholder="Email or Username"]', 'test@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('text="Sign In"');

    // 7. Verify the wizard resumes exactly at the step they left off (Step 3: What do you sell)
    await page.waitForURL('**/*');
    await expect(page.locator('text="What are you selling?"').first()).toBeVisible();

    // 8. Progress to the end of the wizard to ensure completion
    await page.click('text="Physical Products"');
    await page.click('text="Next →"');

    // Step 4: Payments
    await expect(page.locator('text="How do you want to get paid?"').first()).toBeVisible();
    await page.click('text="Online Only"');

    // Step 5: Admin setup
    await expect(page.locator('text="Who is running this business?"').first()).toBeVisible();
    // In Slint testing, the specific inputs might be sequential
    await page.fill('input[placeholder="e.g. admin@acme.com"]', 'admin@acme.com');
    await page.click('text="Next →"');

    // Step 6: Template
    await expect(page.locator('text="Choose your vibe"').first()).toBeVisible();
    await page.click('text="Minimal & Clean"');

    // Step 7: Add Product
    await expect(page.locator('text="Add your first product"').first()).toBeVisible();
    await page.fill('input[placeholder="e.g. Custom Birthday Cake"]', 'Test Product');
    await page.fill('input[placeholder="e.g. 50.00"]', '10.00');
    await page.click('text="Next →"');

    // Step 8: Domain
    await expect(page.locator('text="Choose a Domain"').first()).toBeVisible();
    await page.click('text="Free OHC Domain"');

    // Step 9: Launch
    await expect(page.locator('text="Ready to launch!"').first()).toBeVisible();
    await page.click('text="Launch My Business →"');

    // Wait for checklist / Dashboard
    await expect(page.locator('text="You\'re set up!"').first()).toBeVisible();
    await page.click('text="Go to Dashboard →"');

    // Verify Dashboard displays correctly
    await expect(page.locator('text="Today\'s Sales"').first()).toBeVisible();
  });
});
