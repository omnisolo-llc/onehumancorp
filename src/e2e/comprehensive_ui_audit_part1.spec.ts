import { test, expect } from '@playwright/test';

const VIEWPORTS = [
  { width: 375, height: 667, name: 'Mobile' },
  { width: 768, height: 1024, name: 'Tablet' },
  { width: 1024, height: 768, name: 'Desktop' },
  { width: 1440, height: 900, name: 'Large Desktop' }
];

test.describe('Comprehensive UI Audit - Part 1', () => {
  for (const vp of VIEWPORTS) {
    test.describe(`Viewport: ${vp.name}`, () => {
      test.use({ viewport: vp });

      test.beforeEach(async ({ page }) => {
        await page.goto('/');
      });

      test('CUJ: Complete Signup Flow', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('signup-screen'));
        await expect(page.locator('#signup-screen')).toBeVisible();
        await expect(page.locator('#signup-screen h1')).toHaveText('Create an account');
        await page.fill('#signup-screen input[type="email"]', 'test@example.com');
        await page.fill('#signup-screen input[type="password"]', 'password123');
        await page.click('#signup-screen button:has-text("Sign Up")');
      });

      test('CUJ: Dashboard layout and visibility', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('dashboard-screen'));
        await expect(page.locator('#dashboard-screen')).toBeVisible();
        await expect(page.locator('nav')).toBeVisible();
        await expect(page.locator('#dashboard-screen h1')).toHaveText('Dashboard');

        // Assert glassmorphism presence via class checks
        const cards = page.locator('.card.glass');
        expect(await cards.count()).toBeGreaterThan(0);

        // Verify nav bar links
        await expect(page.locator('nav a').filter({ hasText: 'Dashboard' })).toBeVisible();
        await expect(page.locator('nav a').filter({ hasText: 'Agents' })).toBeVisible();
        await expect(page.locator('nav a').filter({ hasText: 'Setup Wizard' })).toBeVisible();
        await expect(page.locator('nav a').filter({ hasText: 'Software' })).toBeVisible();
      });

      test('CUJ: Agents Screen interaction', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('agents-screen'));
        await expect(page.locator('#agents-screen')).toBeVisible();
        await expect(page.locator('#agents-screen h1')).toHaveText('Agents');

        const deployBtn = page.locator('#agents-screen button:has-text("Deploy New Agent")');
        await expect(deployBtn).toBeVisible();
      });

      test('CUJ: Setup Wizard Flow', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('setup-screen'));
        await expect(page.locator('#setup-screen')).toBeVisible();
        await expect(page.locator('#setup-screen h1')).toHaveText('Business Setup');

        const nextBtn = page.locator('#setup-screen button:has-text("Next")');
        await expect(nextBtn).toBeVisible();
      });

      test('CUJ: Inbox interaction', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('inbox-screen'));
        await expect(page.locator('#inbox-screen')).toBeVisible();
        await expect(page.locator('#inbox-screen h2')).toHaveText('Inbox');
      });

      test('CUJ: Meetings Schedule', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('meetings-screen'));
        await expect(page.locator('#meetings-screen')).toBeVisible();
        await expect(page.locator('#meetings-screen h2')).toHaveText('Meetings & Calendar');
      });

      test('CUJ: Meeting Room', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('meeting-room-screen'));
        await expect(page.locator('#meeting-room-screen')).toBeVisible();
        await expect(page.locator('#meeting-room-screen h2')).toHaveText('Strategic Planning Room');
      });

      test('CUJ: Referral Dashboard', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('referral-dashboard-screen'));
        await expect(page.locator('#referral-dashboard-screen')).toBeVisible();
        await expect(page.locator('#referral-dashboard-screen h1')).toHaveText('Referral Dashboard');
      });

      test('CUJ: Users Management', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('users-screen'));
        await expect(page.locator('#users-screen')).toBeVisible();
        await expect(page.locator('#users-screen h1')).toHaveText('User Management');
      });

      test('CUJ: Settings Configuration', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('settings-screen'));
        await expect(page.locator('#settings-screen')).toBeVisible();
        await expect(page.locator('#settings-screen h1')).toHaveText('Settings');
      });

      test('CUJ: Pricing selection', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('pricing-screen'));
        await expect(page.locator('#pricing-screen')).toBeVisible();
        await expect(page.locator('#pricing-screen h1')).toHaveText('Upgrade Your Plan');
      });

      test('CUJ: Plan Details', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('my-plan-screen'));
        await expect(page.locator('#my-plan-screen')).toBeVisible();
        await expect(page.locator('#my-plan-screen h1')).toHaveText('My Plan');
      });

      test('CUJ: Checkout Process', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('checkout-screen'));
        await expect(page.locator('#checkout-screen')).toBeVisible();
        await expect(page.locator('#checkout-screen h1')).toHaveText('Checkout');
      });

      test('CUJ: Diagnostics Check', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('diagnostics-screen'));
        await expect(page.locator('#diagnostics-screen')).toBeVisible();
        await expect(page.locator('#diagnostics-screen h1')).toHaveText('System Diagnostics');
      });

      test('CUJ: Services Listing', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('services-screen'));
        await expect(page.locator('#services-screen')).toBeVisible();
        await expect(page.locator('#services-screen h1')).toHaveText('Active Services');
      });

      test('CUJ: Scaling Operations', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('scaling-screen'));
        await expect(page.locator('#scaling-screen')).toBeVisible();
        await expect(page.locator('#scaling-screen h1')).toHaveText('Infrastructure Scaling');
      });

      test('CUJ: API Integration', async ({ page }) => {
        await page.evaluate(() => (window as any).showScreen('api-screen'));
        await expect(page.locator('#api-screen')).toBeVisible();
        await expect(page.locator('#api-screen h1')).toHaveText('API Access');
      });
    });
  }
});
