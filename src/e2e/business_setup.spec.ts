import { test, expect } from '@playwright/test';

test.describe('Business Setup Wizard', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/login');
    await page.click('button:has-text("Don\'t have an account? Sign Up")');
    await page.fill('input[placeholder="Email or Username"]', 'test@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('button:has-text("Sign Up")');
  });

  test('should show welcome step', async ({ page }) => {
    await expect(page.locator('text=Welcome')).toBeVisible();
  });

  test('should display the Setup Wizard hero animation elements', async ({ page }) => {
    await expect(page.locator('text=Your business,\nlive in minutes.')).toBeVisible();
    await expect(page.locator('text=Zero tech skills needed. We do the heavy lifting.')).toBeVisible();
    await expect(page.locator('text=Guided Setup →')).toBeVisible();
    await expect(page.locator('text=⚡ Instant Build (AI) →')).toBeVisible();
  });

  test('should display welcome message', async ({ page }) => {
    await expect(page.locator('text=/welcome|get started|Your business, live in minutes/i')).toBeVisible();
  });

  test('should show next button on welcome step', async ({ page }) => {
    await expect(page.locator('text=Guided Setup →')).toBeVisible();
  });

  test('should navigate to business type step', async ({ page }) => {
    await page.click('text=Guided Setup →');
    await expect(page.locator('text=/What kind of business are you building/i')).toBeVisible();
  });

  test('should show business type options', async ({ page }) => {
    await page.click('text=Guided Setup →');
    await expect(page.locator('text=Online Store')).toBeVisible();
    await expect(page.locator('text=Service Business')).toBeVisible();
    await expect(page.locator('text=Restaurant')).toBeVisible();
  });

  test('should select online store option', async ({ page }) => {
    await page.click('text=Guided Setup →');
    await page.locator('text=Online Store').click();
    await expect(page.locator('text=Give your business a name')).toBeVisible();
  });

  test('should select service business option', async ({ page }) => {
    await page.click('text=Guided Setup →');
    await page.locator('text=Service Business').click();
  });

  test('should select restaurant option', async ({ page }) => {
    await page.click('text=Guided Setup →');
    await page.locator('text=Restaurant').click();
  });

  test('should select creative portfolio option', async ({ page }) => {
    await page.click('text=Guided Setup →');
    await page.locator('text=Creative').click();
  });

  test('should select local business option', async ({ page }) => {
    await page.click('text=Guided Setup →');
    await page.locator('text=Local Business').click();
  });

  test('should navigate through wizard steps', async ({ page }) => {
    await page.click('text=Guided Setup →');
    await page.click('text=🛒 Online Store');
    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', 'Test Company');
    await page.click('text=Next →');
    await expect(page.locator('text=What do you sell')).toBeVisible();
  });

  test('should allow going back', async ({ page }) => {
    await page.click('text=Guided Setup →');
    const backButton = page.locator('button:has-text("Back")');
    await expect(backButton).toBeVisible();
    await backButton.click();
    await expect(page.locator('text="Your business, live in minutes."')).toBeVisible();
  });

  test('should show company name input', async ({ page }) => {
    await page.click('text=Guided Setup →');
    await page.click('text=🛒 Online Store');
    await expect(page.locator('input[placeholder="e.g. Maya\'s Cakes"]').first()).toBeVisible();
  });

  test('should show what you sell step', async ({ page }) => {
    await page.click('text=Guided Setup →');
    await page.click('text=🛒 Online Store');
    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', 'Test Company');
    await page.click('text=Next →');
    await expect(page.locator('text=/what do you sell/i')).toBeVisible();
  });

  test('should show physical products option', async ({ page }) => {
    await page.click('text=Guided Setup →');
    await page.click('text=🛒 Online Store');
    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', 'Test Company');
    await page.click('text=Next →');
    await expect(page.locator('text=Physical')).toBeVisible();
  });

  test('should show digital products option', async ({ page }) => {
    await page.click('text=Guided Setup →');
    await page.click('text=🛒 Online Store');
    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', 'Test Company');
    await page.click('text=Next →');
    await expect(page.locator('text=Digital')).toBeVisible();
  });

  test('should show services option', async ({ page }) => {
    await page.click('text=Guided Setup →');
    await page.click('text=🛒 Online Store');
    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', 'Test Company');
    await page.click('text=Next →');
    await expect(page.locator('text=Services')).toBeVisible();
  });

  test('should show payments step', async ({ page }) => {
    await page.click('text=Guided Setup →');
    await page.click('text=🛒 Online Store');
    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', 'Test Company');
    await page.click('text=Next →');
    await page.click('text=📦 Physical products');
    await page.click('text=Next →');
    await expect(page.locator('text=/payment/i')).toBeVisible();
  });

  test('should show admin account step', async ({ page }) => {
    await page.click('text=Guided Setup →');
    await page.click('text=🛒 Online Store');
    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', 'Test Company');
    await page.click('text=Next →');
    await page.click('text=📦 Physical products');
    await page.click('text=Next →');
    await page.click('text=🌐 Online only');
    await page.click('text=Next →');
    await expect(page.locator('text=/admin|account|Create your account/i')).toBeVisible();
  });

  test('should show template selection step', async ({ page }) => {
    await page.click('text=Guided Setup →');
    await page.click('text=🛒 Online Store');
    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', 'Test Company');
    await page.click('text=Next →');
    await page.click('text=📦 Physical products');
    await page.click('text=Next →');
    await page.click('text=🌐 Online only');
    await page.click('text=Next →');
    await page.fill('input[placeholder="e.g. Maya Smith"]', 'Maya Smith');
    await page.fill('input[placeholder="you@email.com"]', 'maya@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('text=Next →');
    await expect(page.locator('text=/Select a Template/i')).toBeVisible();
  });

  test('should show domain step', async ({ page }) => {
    await page.click('text=Guided Setup →');
    await page.click('text=🛒 Online Store');
    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', 'Test Company');
    await page.click('text=Next →');
    await page.click('text=📦 Physical products');
    await page.click('text=Next →');
    await page.click('text=🌐 Online only');
    await page.click('text=Next →');
    await page.fill('input[placeholder="e.g. Maya Smith"]', 'Maya Smith');
    await page.fill('input[placeholder="you@email.com"]', 'maya@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('text=Next →');
    await page.click('text=✨ Modern');
    await page.click('text=Next →');
    await page.fill('input[placeholder="e.g. Custom Birthday Cake"]', 'Test Cake');
    await page.fill('input[placeholder="e.g. 50.00"]', '50.00');
    await page.click('text=Next →');
    await expect(page.locator('text=/Choose your domain/i')).toBeVisible();
  });

  test('should show review and launch step', async ({ page }) => {
    await page.click('text=Guided Setup →');
    await page.click('text=🛒 Online Store');
    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', 'Test Company');
    await page.click('text=Next →');
    await page.click('text=📦 Physical products');
    await page.click('text=Next →');
    await page.click('text=🌐 Online only');
    await page.click('text=Next →');
    await page.fill('input[placeholder="e.g. Maya Smith"]', 'Maya Smith');
    await page.fill('input[placeholder="you@email.com"]', 'maya@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('text=Next →');
    await page.click('text=✨ Modern');
    await page.click('text=Next →');
    await page.fill('input[placeholder="e.g. Custom Birthday Cake"]', 'Test Cake');
    await page.fill('input[placeholder="e.g. 50.00"]', '50.00');
    await page.click('text=Next →');
    await page.click('text=🌐 Free OHC Domain');
    await page.click('text=Next →');
    await expect(page.locator('text=/review|launch|Ready to launch!/i')).toBeVisible();
  });

  test('should show launch button on final step', async ({ page }) => {
    await page.click('text=Guided Setup →');
    await page.click('text=🛒 Online Store');
    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', 'Test Company');
    await page.click('text=Next →');
    await page.click('text=📦 Physical products');
    await page.click('text=Next →');
    await page.click('text=🌐 Online only');
    await page.click('text=Next →');
    await page.fill('input[placeholder="e.g. Maya Smith"]', 'Maya Smith');
    await page.fill('input[placeholder="you@email.com"]', 'maya@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('text=Next →');
    await page.click('text=✨ Modern');
    await page.click('text=Next →');
    await page.fill('input[placeholder="e.g. Custom Birthday Cake"]', 'Test Cake');
    await page.fill('input[placeholder="e.g. 50.00"]', '50.00');
    await page.click('text=Next →');
    await page.click('text=🌐 Free OHC Domain');
    await page.click('text=Next →');
    await expect(page.locator('text="Launch My Business →"')).toBeVisible();
  });

  test('should show welcome checklist after successful launch', async ({ page }) => {
    await page.click('text=Guided Setup →');
    await page.click('text=🛒 Online Store');
    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', 'Test Company');
    await page.click('text=Next →');
    await page.click('text=📦 Physical products');
    await page.click('text=Next →');
    await page.click('text=🌐 Online only');
    await page.click('text=Next →');
    await page.fill('input[placeholder="e.g. Maya Smith"]', 'Maya Smith');
    await page.fill('input[placeholder="you@email.com"]', 'maya@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('text=Next →');
    await page.click('text=✨ Modern');
    await page.click('text=Next →');
    await page.fill('input[placeholder="e.g. Custom Birthday Cake"]', 'Test Cake');
    await page.fill('input[placeholder="e.g. 50.00"]', '50.00');
    await page.click('text=Next →');
    await page.click('text=🌐 Free OHC Domain');
    await page.click('text=Next →');

    // Launch the business
    await page.click('text="Launch My Business →"');

    // Wait for the success state/confetti
    await expect(page.locator('text=/CONFETTI.*SUCCESS/i')).toBeVisible({ timeout: 5000 });

    // Click view welcome checklist
    const viewChecklistBtn = page.locator('text="View Welcome Checklist →"');
    await viewChecklistBtn.click();

    // We should be on step 10 now
    await expect(page.locator('text="You\'re set up! Here\'s what to do next:"')).toBeVisible();

    // Verify the checklist elements exist
    const addProducts = page.locator('text="Add 3 more products"');
    await expect(addProducts).toBeVisible();

    const connectInstagram = page.locator('text="Connect Instagram"');
    await expect(connectInstagram).toBeVisible();

    const shareLink = page.locator('text="Share your link with a friend"');
    await expect(shareLink).toBeVisible();

    const dashboardLink = page.locator('text="Go to Dashboard →"');
    await expect(dashboardLink).toBeVisible();

    // Verify exit state by clicking to Dashboard
    await dashboardLink.click();
  });

  test('should display the Setup Wizard hero animation elements and complete full setup flow', async ({ page }) => {
    await expect(page.locator('text=Your business,\nlive in minutes.')).toBeVisible();
    await expect(page.locator('text=Zero tech skills needed. We do the heavy lifting.')).toBeVisible();
    await expect(page.locator('text=Guided Setup →')).toBeVisible();
    await expect(page.locator('text=⚡ Instant Build (AI) →')).toBeVisible();

    await page.click('text=Guided Setup →');
    await expect(page.locator('text=What kind of business are you building?')).toBeVisible();
    await page.click('text=🛒 Online Store');
    await page.click('text=Next →');

    await expect(page.locator('text=Give your business a name')).toBeVisible();
    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', 'Test Company Hero');
    await page.click('text=Next →');

    await expect(page.locator('text=What do you sell?')).toBeVisible();
    await page.click('text=📦 Physical products');
    await page.click('text=Next →');

    await expect(page.locator('text=How do you want to receive payments?')).toBeVisible();
    await page.click('text=🌐 Online only');
    await page.click('text=Next →');

    await expect(page.locator('text=Create your account')).toBeVisible();
    await page.fill('input[placeholder="e.g. Maya Smith"]', 'Maya Smith');
    await page.fill('input[placeholder="you@email.com"]', 'maya@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('text=Next →');

    await expect(page.locator('text=Choose a Template')).toBeVisible();
    await page.click('text=✨ Modern');
    await page.click('text=Next →');

    await expect(page.locator('text=Add your first product or service')).toBeVisible();
    await page.fill('input[placeholder="e.g. Custom Birthday Cake"]', 'Test Cake');
    await page.fill('input[placeholder="e.g. 50.00"]', '50.00');
    await page.click('text=Next →');

    await expect(page.locator('text=Choose a Domain')).toBeVisible();
  });
});

test.describe('Business Setup Wizard Validation', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/login');
    await page.click('button:has-text("Don\'t have an account? Sign Up")');
    await page.fill('input[placeholder="Email or Username"]', 'test@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('button:has-text("Sign Up")');
  });

  test('should require business type selection', async ({ page }) => {
    await page.click('text=Guided Setup →');
    const nextBtn = page.locator('text=Next →');
    if (await nextBtn.isVisible()) {
      await nextBtn.click();
      await expect(page.locator('text=/select.*type|choose.*type/i')).toBeVisible({ timeout: 3000 });
    }
  });

  test('should require company name', async ({ page }) => {
    await page.click('text=Guided Setup →');
    await page.locator('text=Online Store').click();
    await page.click('text=Next →'); // To step 3
    await expect(page.locator('text=/required|name.*required/i')).toBeVisible({ timeout: 3000 });
  });

  test('should validate email format', async ({ page }) => {
    await page.click('text=Guided Setup →');
    await page.click('text=🛒 Online Store');
    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', 'Test Company');
    await page.click('text=Next →');
    await page.click('text=📦 Physical products');
    await page.click('text=Next →');
    await page.click('text=🌐 Online only');
    await page.click('text=Next →');

    await page.fill('input[placeholder="you@email.com"]', 'invalidemail');
    await page.click('text=Next →');
    await expect(page.locator('text=/invalid.*email|email.*invalid/i')).toBeVisible({ timeout: 3000 });
  });

  test('should validate password strength', async ({ page }) => {
    await page.click('text=Guided Setup →');
    await page.click('text=🛒 Online Store');
    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', 'Test Company');
    await page.click('text=Next →');
    await page.click('text=📦 Physical products');
    await page.click('text=Next →');
    await page.click('text=🌐 Online only');
    await page.click('text=Next →');

    await page.fill('input[placeholder="Password"]', 'weak');
    await expect(page.locator('text=Strength: Weak')).toBeVisible({ timeout: 3000 });
  });
});

  test('should use Minimalist template and verify launch successfully triggers', async ({ page }) => {
    await page.click('text=Guided Setup →');
    await page.click('text=🛒 Online Store');
    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', 'Test Company');
    await page.click('text=Next →');
    await page.click('text=📦 Physical products');
    await page.click('text=Next →');
    await page.click('text=🌐 Online only');
    await page.click('text=Next →');
    await page.fill('input[placeholder="e.g. Maya Smith"]', 'Maya Smith');
    await page.fill('input[placeholder="you@email.com"]', 'maya@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('text=Next →');

    // Select Minimalist template
    await page.click('text=Minimalist');
    await page.click('text=Next →');

    await page.fill('input[placeholder="e.g. Custom Birthday Cake"]', 'Test Cake');
    await page.fill('input[placeholder="e.g. 50.00"]', '50.00');
    await page.click('text=Next →');
    await page.click('text=🌐 Free OHC Domain');
    await page.click('text=Next →');
    await expect(page.locator('text="Launch My Business →"')).toBeVisible();
    await page.click('text="Launch My Business →"');
    await expect(page.locator('text=/CONFETTI.*SUCCESS/i')).toBeVisible({ timeout: 5000 });
  });

  test('should input specific product name "Custom Vegan Cookies" and advance to launch', async ({ page }) => {
    await page.click('text=Guided Setup →');
    await page.click('text=🛒 Online Store');
    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', 'Test Company');
    await page.click('text=Next →');
    await page.click('text=📦 Physical products');
    await page.click('text=Next →');
    await page.click('text=🌐 Online only');
    await page.click('text=Next →');
    await page.fill('input[placeholder="e.g. Maya Smith"]', 'Maya Smith');
    await page.fill('input[placeholder="you@email.com"]', 'maya@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('text=Next →');
    await page.click('text=✨ Modern');
    await page.click('text=Next →');

    // Specific product name
    await page.fill('input[placeholder="e.g. Custom Birthday Cake"]', 'Custom Vegan Cookies');
    await page.fill('input[placeholder="e.g. 50.00"]', '50.00');
    await page.click('text=Next →');

    await page.click('text=🌐 Free OHC Domain');
    await page.click('text=Next →');
    await expect(page.locator('text="Launch My Business →"')).toBeVisible();
    await page.click('text="Launch My Business →"');
    await expect(page.locator('text=/CONFETTI.*SUCCESS/i')).toBeVisible({ timeout: 5000 });
  });

  test('should set product price "24.99" and verify launch', async ({ page }) => {
    await page.click('text=Guided Setup →');
    await page.click('text=🛒 Online Store');
    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', 'Test Company');
    await page.click('text=Next →');
    await page.click('text=📦 Physical products');
    await page.click('text=Next →');
    await page.click('text=🌐 Online only');
    await page.click('text=Next →');
    await page.fill('input[placeholder="e.g. Maya Smith"]', 'Maya Smith');
    await page.fill('input[placeholder="you@email.com"]', 'maya@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('text=Next →');
    await page.click('text=✨ Modern');
    await page.click('text=Next →');

    await page.fill('input[placeholder="e.g. Custom Birthday Cake"]', 'Test Cake');
    // Set specific product price
    await page.fill('input[placeholder="e.g. 50.00"]', '24.99');
    await page.click('text=Next →');

    await page.click('text=🌐 Free OHC Domain');
    await page.click('text=Next →');
    await expect(page.locator('text="Launch My Business →"')).toBeVisible();
    await page.click('text="Launch My Business →"');
    await expect(page.locator('text=/CONFETTI.*SUCCESS/i')).toBeVisible({ timeout: 5000 });
  });

  test('should use a "custom domain" option and validate launch step proceeds', async ({ page }) => {
    await page.click('text=Guided Setup →');
    await page.click('text=🛒 Online Store');
    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', 'Test Company');
    await page.click('text=Next →');
    await page.click('text=📦 Physical products');
    await page.click('text=Next →');
    await page.click('text=🌐 Online only');
    await page.click('text=Next →');
    await page.fill('input[placeholder="e.g. Maya Smith"]', 'Maya Smith');
    await page.fill('input[placeholder="you@email.com"]', 'maya@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('text=Next →');
    await page.click('text=✨ Modern');
    await page.click('text=Next →');
    await page.fill('input[placeholder="e.g. Custom Birthday Cake"]', 'Test Cake');
    await page.fill('input[placeholder="e.g. 50.00"]', '50.00');
    await page.click('text=Next →');

    // Select custom domain
    await page.click('text=🔗 Connect Custom Domain');
    // Assuming there might be an input field appearing for custom domain, we fill it if needed, or simply proceed.
    // Based on the UI, domain_choice is set to "custom"
    await page.click('text=Next →');

    await expect(page.locator('text="Launch My Business →"')).toBeVisible();
    await page.click('text="Launch My Business →"');
    await expect(page.locator('text=/CONFETTI.*SUCCESS/i')).toBeVisible({ timeout: 5000 });
  });

  test('should select "Use OHC subdomain" and successfully launch wizard', async ({ page }) => {
    await page.click('text=Guided Setup →');
    await page.click('text=🛒 Online Store');
    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', 'Test Company');
    await page.click('text=Next →');
    await page.click('text=📦 Physical products');
    await page.click('text=Next →');
    await page.click('text=🌐 Online only');
    await page.click('text=Next →');
    await page.fill('input[placeholder="e.g. Maya Smith"]', 'Maya Smith');
    await page.fill('input[placeholder="you@email.com"]', 'maya@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('text=Next →');
    await page.click('text=✨ Modern');
    await page.click('text=Next →');
    await page.fill('input[placeholder="e.g. Custom Birthday Cake"]', 'Test Cake');
    await page.fill('input[placeholder="e.g. 50.00"]', '50.00');
    await page.click('text=Next →');

    // Select Free OHC Domain (subdomain)
    await page.click('text=🌐 Free OHC Domain');
    await page.click('text=Next →');

    await expect(page.locator('text="Launch My Business →"')).toBeVisible();
    await page.click('text="Launch My Business →"');
    await expect(page.locator('text=/CONFETTI.*SUCCESS/i')).toBeVisible({ timeout: 5000 });
  });

    await page.click('text=Next →');

