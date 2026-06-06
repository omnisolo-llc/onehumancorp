import { test, expect } from '@playwright/test';

test.describe('Scribe: Documentation & Help Center E2E', () => {

  test.beforeEach(async ({ page }) => {
    await page.setViewportSize({ width: 1280, height: 800 });
    await page.addInitScript(() => {
      window.localStorage.setItem('TEST_WALKTHROUGH', 'true');
      window.localStorage.setItem('TEST_CHAT', 'true');
      // Pre-seed builder store to avoid onboarding redirection
      window.localStorage.setItem('builder-storage', JSON.stringify({
        state: {
          bio: "I bake cakes",
          businessName: "Maya Cakes",
          businessCategory: "Bakery",
          vibe: "Friendly",
          wizardStep: 3,
          blocks: [],
          drafts: [],
          status: "idle",
          businessGoal: "products",
          liveUrl: ""
        },
        version: 0
      }));
    });
    await page.goto('http://localhost:3000/help');
    await page.waitForLoadState('networkidle');
  });

  test('Help Center search finds persona-specific articles', async ({ page }) => {
    const searchInput = page.locator('input[placeholder="Search for help articles and videos..."]');
    await searchInput.fill('Maya');
    await expect(page.getByText('Getting Started').first()).toBeVisible({ timeout: 10000 });
    await expect(page.getByText('Learn how Maya the baker').first()).toBeVisible();

    await searchInput.fill('Carlos');
    await expect(page.getByText('My Store').first()).toBeVisible({ timeout: 10000 });
    await expect(page.getByText('Add products like Carlos').first()).toBeVisible();
  });

  test('Help Article renders persona content correctly', async ({ page }) => {
    await page.getByText('Getting Started').first().click();
    await page.waitForURL(/.*getting-started-1/);
    await expect(page.locator('h1')).toContainText('Getting Started with Your Store');
    await expect(page.getByText('Maya, our home baker persona').first()).toBeVisible({ timeout: 10000 });
  });

  test('Interactive Walkthrough: Store Setup', async ({ page }) => {
    await page.click('button[aria-label="Help"]');
    await page.click('text=Tour: Set up your store');
    await page.waitForURL(/.*builder/);
    const bubble = page.getByTestId('walkthrough-bubble');
    await expect(bubble).toBeVisible({ timeout: 20000 });
    await expect(bubble).toContainText('Welcome Maya!');
  });

  test('Interactive Walkthrough: Payments', async ({ page }) => {
    await page.click('button[aria-label="Help"]');
    await page.click('text=Tour: Accept your first payment');
    await page.waitForURL(/.*checkout/);
    const bubble = page.getByTestId('walkthrough-bubble');
    await expect(bubble).toBeVisible({ timeout: 20000 });
    await expect(bubble).toContainText('Connect your bank');
  });

  test('Interactive Walkthrough: AI Support Agent', async ({ page }) => {
    await page.click('button[aria-label="Help"]');
    await page.click('text=Tour: Activate your AI Support Agent');
    await page.waitForURL(/.*dashboard/);
    const bubble = page.getByTestId('walkthrough-bubble');
    await expect(bubble).toBeVisible({ timeout: 20000 });
    await expect(bubble).toContainText('Meet your workforce!');
  });

  test('Changelog displays persona-driven updates', async ({ page }) => {
    await page.goto('http://localhost:3000/changelog');
    await page.waitForLoadState('networkidle');
    await expect(page.locator('h1')).toContainText('Release Notes & Changelog');
    await expect(page.getByText('Persona-Driven Help Center').first()).toBeVisible({ timeout: 10000 });
  });

  test('AI Help Chat responds correctly', async ({ page }) => {
    await page.click('button[aria-label="Help"]');
    await page.click('button:has-text("Ask AI")');
    const chatInput = page.locator('input[placeholder="Ask anything..."]').first();
    await chatInput.fill('getting started');
    await chatInput.press('Enter');

    await expect(page.locator('text=I am your AI Help Agent!').first()).toBeVisible({ timeout: 20000 });
    await expect(page.locator('text=Read the full article →').first()).toBeVisible();
  });

  test('Advanced API Documentation loads', async ({ page }) => {
    await page.goto('http://localhost:3000/api-docs');
    await expect(page.locator('text=OHC Advanced API Reference')).toBeVisible({ timeout: 15000 });
    await expect(page.locator('.swagger-ui')).toBeVisible();
  });
});
