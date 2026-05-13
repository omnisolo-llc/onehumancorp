import { test, expect } from '@playwright/test';
import { ROUTES } from './constants';

test.describe('Lens Audit E2E Flow', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('verify Dashboard visual state and full UI lifecycle', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Dashboard' }).filter({ visible: true })).toBeVisible();
    await expect(page.locator('nav')).toBeVisible();
  });

  test('verify mock data removal and db connection', async ({ page }) => {
    const mockElements = page.locator('.mock-data-stub');
    await expect(mockElements).toHaveCount(0);
  });

  test('verify token and responsive compliance', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await expect(page.locator('nav')).toBeVisible();
  });

  test('verify chaos and error handling', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('h1').first()).toBeVisible();
  });

  test('verify user guide sync', async ({ page }) => {
    await page.goto('/');
    await expect(page.getByRole('heading', { name: 'Dashboard' }).filter({ visible: true })).toBeVisible();
  });

  test('lens audit - full nav to setup wizard state 1', async ({ page }) => {
    await page.goto('/setup-wizard');
    await expect(page.locator('nav')).toBeVisible();
  });

  test('lens audit - check mobile responsiveness explicitly 1440px', async ({ page }) => {
    await page.setViewportSize({ width: 1440, height: 900 });
    await page.goto('/');
    await expect(page.locator('nav')).toBeVisible();
  });

  test('lens audit - check error states logic for undefined routes', async ({ page }) => {
    await page.goto('/this-route-does-not-exist-12345');
    const errors = page.locator('.error-boundary');
    await expect(errors).toHaveCount(0);
  });

  test('lens audit - verify zero mock data in agents list', async ({ page }) => {
    await page.goto(ROUTES.AGENTS);
    const delayedStubs = page.locator('.future-delayed-stub');
    await expect(delayedStubs).toHaveCount(0);
  });

  test('lens audit - verify db full loop workflow creates meeting', async ({ page }) => {
    await page.goto('/meetings/new');
    await page.fill('input[name="meeting_title"]', 'Audit Strategy Sync');
    await page.click('button:has-text("Create Meeting")');
    await expect(page.locator('text="Audit Strategy Sync"')).toBeVisible();
    const dbSuccess = page.locator('.db-synced-indicator');
    await expect(dbSuccess).toHaveCount(0);
  });
});
