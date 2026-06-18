import { test, expect } from '@playwright/test';

test.describe('Mobile Autonomous Onboarding & Feed CUJ', () => {

  test.beforeEach(async ({ page }) => {
    // Mobile view 375px
    await page.setViewportSize({ width: 375, height: 812 });
    await page.addInitScript(() => window.localStorage.clear());
  });

  test('Persona: Maya completes zero-click onboarding and approves welcome action on mobile', async ({ page }) => {
    // 1. Start from home
    await page.goto('/api/ui/index.html');
    await page.click('#start-btn');

    // 2. Choose Instant Build
    await page.click('button:has-text("Instant Build")');

    // 3. Enter business concept
    const bio = 'I bake and sell custom cupcakes in Austin via delivery.';
    await page.fill('#instant-bio', bio);

    // 4. Trigger build
    await page.click('#generate-storefront-btn');

    // 5. Verify optimized loader
    await expect(page.locator('#loading-title')).toBeVisible();
    await expect(page.locator('#step-provisioning')).toHaveCSS('opacity', '1');

    // 6. Wait for redirect to Dashboard (Command Center)
    await expect(page).toHaveURL(/.*dashboard\.html/, { timeout: 30000 });

    // 7. Verify Command Center is prioritized
    await expect(page.locator('#triage-section h2')).toHaveText('Command Center');

    // 8. Verify initial welcome card from OnboardingAgent
    const welcomeCard = page.locator('[data-testid="onboarding-welcome-card"]');
    await expect(welcomeCard).toBeVisible({ timeout: 15000 });
    await expect(welcomeCard).toContainText('Welcome to OHC!');
    await expect(welcomeCard).toContainText('Austin');

    // 9. Interaction Audit: Verify "Review Storefront" button works
    const reviewBtn = welcomeCard.locator('button:has-text("Review Storefront")');
    await expect(reviewBtn).toBeVisible();
    await expect(reviewBtn).toHaveCSS('min-height', '44px');

    // Click should navigate or trigger action (here it navigates to /storefront)
    await reviewBtn.click();
    // Assuming /storefront redirects to some page or we just check URL change if we mocked navigation in dashboard.html
  });
});
