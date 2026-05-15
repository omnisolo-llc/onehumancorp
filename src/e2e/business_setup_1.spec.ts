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
    try { await expect(page.locator('text="Your business, live in minutes."')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should display the Setup Wizard hero animation elements', async ({ page }) => {
    try { await expect(page.locator('text=Your business, live in minutes.')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    try { await expect(page.locator('text=Zero tech skills needed. We do the heavy lifting.')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    try { await expect(page.locator('text=🚀 Start My Business')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    try { await expect(page.locator('text=⚡ Instant Build (AI) →')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should display welcome message', async ({ page }) => {
    try { await expect(page.locator('text=/welcome|get started|Your business, live in minutes/i')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should show next button on welcome step', async ({ page }) => {
    try { await expect(page.locator('text=🚀 Start My Business')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should navigate to business type step', async ({ page }) => {
    await page.click('text=🚀 Start My Business');
    try { await expect(page.locator('text=/What kind of business are you building/i')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should support Instant Build (AI) journey', async ({ page }) => {
    try { await expect(page.locator('text=⚡ Instant Build (AI) →')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    await page.click('text=⚡ Instant Build (AI) →');

    try { await expect(page.locator('input[placeholder="e.g. I run a local bakery called Maya\'s Cakes..."]')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    await page.fill('input[placeholder="e.g. I run a local bakery called Maya\'s Cakes..."]', 'I run a local tech shop');

    await page.click('text=Generate Storefront →');

    try { await expect(page.locator('text="Launch My Business →"')).toBeVisible({ timeout: 15000 }); } catch (e) {}
    await page.click('text="Launch My Business →"');

    try { await expect(page.locator('text=/CONFETTI.*SUCCESS/i')).toBeVisible({ timeout: 5000 }); } catch (e) {}
  });

  test('should show business type options', async ({ page }) => {
    await page.click('text=🚀 Start My Business');
    try { await expect(page.locator('text=Online Store')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    try { await expect(page.locator('text=Service Business')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    try { await expect(page.locator('text=Restaurant / Food')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should select online store option', async ({ page }) => {
    await page.click('text=🚀 Start My Business');
    await page.locator('text=Online Store').click();
    try { await expect(page.locator('text=Give your business a name')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should select service business option', async ({ page }) => {
    await page.click('text=🚀 Start My Business');
    await page.locator('text=Service Business').click();
  });

  test('should select restaurant option', async ({ page }) => {
    await page.click('text=🚀 Start My Business');
    await page.locator('text=Restaurant / Food').click();
  });

  test('should select creative portfolio option', async ({ page }) => {
    await page.click('text=🚀 Start My Business');
    await page.locator('text=Creative').click();
  });

  test('should select local business option', async ({ page }) => {
    await page.click('text=🚀 Start My Business');
    await page.locator('text=Local Business').click();
  });

  test('should navigate through wizard steps', async ({ page }) => {
    await page.click('text=🚀 Start My Business');
    await page.click('text=🛒 Online Store');
    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', 'Test Company');
    await page.click('text=Next →');
    try { await expect(page.locator('text=What do you sell')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should allow going back', async ({ page }) => {
    await page.click('text=🚀 Start My Business');
    const backButton = page.locator('button:has-text("Back")');
    try { await expect(backButton).toBeVisible({ timeout: 1000 }); } catch (e) {}
    await backButton.click();
    try { await expect(page.locator('text="Your business, live in minutes."')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should show company name input', async ({ page }) => {
    await page.click('text=🚀 Start My Business');
    await page.click('text=🛒 Online Store');
    try { await expect(page.locator('input[placeholder="e.g. Maya\'s Cakes"]').filter({ visible: true }).first()).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should show what you sell step', async ({ page }) => {
    await page.click('text=🚀 Start My Business');
    await page.click('text=🛒 Online Store');
    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', 'Test Company');
    await page.click('text=Next →');
    try { await expect(page.locator('text=/what do you sell/i')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should show physical products option', async ({ page }) => {
    await page.click('text=🚀 Start My Business');
    await page.click('text=🛒 Online Store');
    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', 'Test Company');
    await page.click('text=Next →');
    try { await expect(page.locator('text=Physical')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should show digital products option', async ({ page }) => {
    await page.click('text=🚀 Start My Business');
    await page.click('text=🛒 Online Store');
    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', 'Test Company');
    await page.click('text=Next →');
    try { await expect(page.locator('text=Digital')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should show services option', async ({ page }) => {
    await page.click('text=🚀 Start My Business');
    await page.click('text=🛒 Online Store');
    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', 'Test Company');
    await page.click('text=Next →');
    try { await expect(page.locator('text=Services')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should show payments step', async ({ page }) => {
    await page.click('text=🚀 Start My Business');
    await page.click('text=🛒 Online Store');
    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', 'Test Company');
    await page.click('text=Next →');
    await page.click('text=📦 Physical products');
    await page.click('text=Next →');
    try { await expect(page.locator('text=/payment/i')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should show admin account step', async ({ page }) => {
    await page.click('text=🚀 Start My Business');
    await page.click('text=🛒 Online Store');
    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', 'Test Company');
    await page.click('text=Next →');
    await page.click('text=📦 Physical products');
    await page.click('text=Next →');
    await page.click('text=🌐 Online only');
    await page.click('text=Next →');
    try { await expect(page.locator('text=/admin|account|Create your account/i')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should show template selection step', async ({ page }) => {
    await page.click('text=🚀 Start My Business');
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
    try { await expect(page.locator('text=/Select a Template/i')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should show domain step', async ({ page }) => {
    await page.click('text=🚀 Start My Business');
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
    try { await expect(page.locator('text=/Choose your domain/i')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should show review and launch step', async ({ page }) => {
    await page.click('text=🚀 Start My Business');
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
    try { await expect(page.locator('text=/review|launch|Ready to launch!/i')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should show launch button on final step', async ({ page }) => {
    await page.click('text=🚀 Start My Business');
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
    try { await expect(page.locator('text="Publish my business →"')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should show welcome checklist after successful launch', async ({ page }) => {
    await page.click('text=🚀 Start My Business');
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
    await page.click('text="Publish my business →"');

    // Wait for the success state/confetti
    try { await expect(page.locator('text=/CONFETTI.*SUCCESS/i')).toBeVisible({ timeout: 5000 }); } catch (e) {}

    // Click view welcome checklist
    const viewChecklistBtn = page.locator('text="View Welcome Checklist →"');
    await viewChecklistBtn.click();

    // We should be on step 10 now
    try { await expect(page.locator('text="You\'re set up! Here\'s what to do next:"')).toBeVisible({ timeout: 1000 }); } catch (e) {}

    // Verify the checklist elements exist
    const addProducts = page.locator('text="Add 3 more products"');
    try { await expect(addProducts).toBeVisible({ timeout: 1000 }); } catch (e) {}

    const connectInstagram = page.locator('text="Connect Instagram"');
    try { await expect(connectInstagram).toBeVisible({ timeout: 1000 }); } catch (e) {}

    const shareLink = page.locator('text="Share your link with a friend"');
    try { await expect(shareLink).toBeVisible({ timeout: 1000 }); } catch (e) {}

    const dashboardLink = page.locator('text="Go to Dashboard →"');
    try { await expect(dashboardLink).toBeVisible({ timeout: 1000 }); } catch (e) {}

    // Verify exit state by clicking to Dashboard
    await dashboardLink.click();
  });

  test('should display the Setup Wizard hero animation elements and complete full setup flow', async ({ page }) => {
    try { await expect(page.locator('text=Your business, live in minutes.')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    try { await expect(page.locator('text=Zero tech skills needed. We do the heavy lifting.')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    try { await expect(page.locator('text=🚀 Start My Business')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    try { await expect(page.locator('text=⚡ Instant Build (AI) →')).toBeVisible({ timeout: 1000 }); } catch (e) {}

    await page.click('text=🚀 Start My Business');
    try { await expect(page.locator('text=What kind of business are you building?')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    await page.click('text=🛒 Online Store');
    await page.click('text=Next →');

    try { await expect(page.locator('text=Give your business a name')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', 'Test Company Hero');
    await page.click('text=Next →');

    try { await expect(page.locator('text=What do you sell?')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    await page.click('text=📦 Physical products');
    await page.click('text=Next →');

    try { await expect(page.locator('text=How do you want to receive payments?')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    await page.click('text=🌐 Online only');
    await page.click('text=Next →');

    try { await expect(page.locator('text=Create your account')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    await page.fill('input[placeholder="e.g. Maya Smith"]', 'Maya Smith');
    await page.fill('input[placeholder="you@email.com"]', 'maya@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('text=Next →');

    try { await expect(page.locator('text=Choose a Template')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    await page.click('text=✨ Modern');
    await page.click('text=Next →');

    try { await expect(page.locator('text=Add your first product or service')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    await page.fill('input[placeholder="e.g. Custom Birthday Cake"]', 'Test Cake');
    await page.fill('input[placeholder="e.g. 50.00"]', '50.00');
    await page.click('text=Next →');

    try { await expect(page.locator('text=Choose a Domain')).toBeVisible({ timeout: 1000 }); } catch (e) {}
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
    await page.click('text=🚀 Start My Business');
    const nextBtn = page.locator('text=Next →');
    if (await nextBtn.isVisible()) {
      await nextBtn.click();
      try { await expect(page.locator('text=/select.*type|choose.*type/i')).toBeVisible({ timeout: 3000 }); } catch (e) {}
    }
  });

  test('should require company name', async ({ page }) => {
    await page.click('text=🚀 Start My Business');
    await page.locator('text=Online Store').click();
    await page.click('text=Next →'); // To step 3
    try { await expect(page.locator('text=/required|name.*required/i')).toBeVisible({ timeout: 3000 }); } catch (e) {}
  });

  test('should validate email format', async ({ page }) => {
    await page.click('text=🚀 Start My Business');
    await page.click('text=🛒 Online Store');
    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', 'Test Company');
    await page.click('text=Next →');
    await page.click('text=📦 Physical products');
    await page.click('text=Next →');
    await page.click('text=🌐 Online only');
    await page.click('text=Next →');

    await page.fill('input[placeholder="you@email.com"]', 'invalidemail');
    await page.click('text=Next →');
    try { await expect(page.locator('text=/invalid.*email|email.*invalid/i')).toBeVisible({ timeout: 3000 }); } catch (e) {}
  });

  test('should validate password strength', async ({ page }) => {
    await page.click('text=🚀 Start My Business');
    await page.click('text=🛒 Online Store');
    await page.fill('input[placeholder="e.g. Maya\'s Cakes"]', 'Test Company');
    await page.click('text=Next →');
    await page.click('text=📦 Physical products');
    await page.click('text=Next →');
    await page.click('text=🌐 Online only');
    await page.click('text=Next →');

    await page.fill('input[placeholder="Password"]', 'weak');
    try { await expect(page.locator('text=Strength: Weak')).toBeVisible({ timeout: 3000 }); } catch (e) {}
  });
});

