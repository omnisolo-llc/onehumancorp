import { test, expect } from './fixtures';

test.describe('Business Journey End-to-End', () => {
  test('User can complete onboarding and reach dashboard', async ({ page }) => {
    // 1. Start from the Home Page (Dashboard)
    await page.goto('/');

    // 2. Navigate to setup via the Setup nav link
    await page.locator('nav a:has-text("Setup")').click();
    await expect(page.locator('#setup-screen')).toBeVisible();

    // 3. Step 1: Welcome
    await expect(page.locator('text=Your business, live in 3 minutes.')).toBeVisible();
    await page.click('text=Start My Business');

    // 4. Step 2: What do you sell?
    await expect(page.locator('text=What do you sell?')).toBeVisible();
    await page.click('text=Products'); // Modular grid button

    // 5. Step 3: Business Name
    await expect(page.locator('text=What is your business name?')).toBeVisible();
    await page.fill('#business-name-input', "Maya's Bakery");
    await page.click('#step-3 button:has-text("Next")');

    // 6. Step 4: Stripe Connect
    await expect(page.locator('text=Accept Payments')).toBeVisible();
    await page.click('text=Skip for now');

    // 7. Magic Moment: AI Generation
    await expect(page.locator('text=The Promoter is designing your site...')).toBeVisible();

    // Wait for AI generation to complete (it has progressive messages)
    await expect(page.locator('text=Your store is ready!')).toBeVisible({ timeout: 10000 });

    // Click the final CTA to reach dashboard
    await page.click('#continue-to-dashboard-btn');

    // 8. Assert Dashboard State
    await expect(page.locator('#dashboard-screen')).toBeVisible();
    await expect(page.locator('text=Overview')).toBeVisible();
    await expect(page.locator('#action-banner')).toBeVisible();
    await expect(page.locator('text=Action Required')).toBeVisible();
    await expect(page.locator('text=Revenue Today')).toBeVisible();
    await expect(page.locator('text=The Promoter')).toBeVisible();
    await expect(page.locator('text=The Manager')).toBeVisible();
    await expect(page.locator('text=The Salesperson')).toBeVisible();
  });

  test('Mobile responsiveness check (375px)', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/business-setup');

    // Check if the card is visible and fits
    const card = page.locator('#setup-screen .card').first();
    await expect(card).toBeVisible();
    const box = await card.boundingBox();
    expect(box?.width).toBeLessThanOrEqual(375);
  });

  test('Touch target size check', async ({ page }) => {
    await page.goto('/business-setup');
    const button = page.locator('button:has-text("Start My Business")').first();
    const box = await button.boundingBox();
    expect(box?.height).toBeGreaterThanOrEqual(44);
    expect(box?.width).toBeGreaterThanOrEqual(44);
  });

  test('Zero jargon check on dashboard', async ({ page }) => {
    await page.goto('/');

    const content = await page.innerText('body');
    const technicalJargon = ['Kubernetes', 'Bazel', 'Rust', 'SQL', 'API', 'JSON'];

    for (const jargon of technicalJargon) {
      expect(content).not.toContain(jargon);
    }
  });

  test('Advanced Developer Settings toggle works', async ({ page }) => {
    await page.goto('/');
    await page.click('text=Settings');

    // Developer mode should be disabled by default
    await expect(page.locator('#dev-mode-status')).toHaveText('Disabled');

    // Toggle it on
    await page.click('#dev-mode-status');
    await expect(page.locator('#dev-mode-status')).toHaveText('Enabled');

    // Technical buttons should now be visible
    await expect(page.locator('text=System Diagnostics')).toBeVisible();
    await expect(page.locator('text=API Management')).toBeVisible();

    // Check nav for Connect Tools
    const connectToolsLink = page.locator('#main-nav a:has-text("Connect Tools")');
    await expect(connectToolsLink).toBeVisible();

    // Toggle it off
    await page.click('#dev-mode-status');
    await expect(page.locator('#dev-mode-status')).toHaveText('Disabled');
    await expect(page.locator('text=System Diagnostics')).not.toBeVisible();
  });

  test('Conversational progress updates', async ({ page }) => {
    await page.goto('/business-setup');
    await page.click('text=Start My Business');
    await page.click('text=Products');
    await page.fill('#business-name-input', "Test Biz");
    await page.click('#step-3 button:has-text("Next")');
    await page.click('text=Skip for now');

    // Check for progressive AI messages
    await expect(page.locator('#ai-progress-text')).toBeVisible();
    // It should cycle through messages. We just check if it exists and has content.
    const text = await page.innerText('#ai-progress-text');
    expect(text.length).toBeGreaterThan(0);
  });
});
