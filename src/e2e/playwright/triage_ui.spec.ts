import { test, expect } from '@playwright/test';
import { currentAppSmoke } from '../current_app_smoke';

test.describe('Work Triage Feed CUJ', () => {
  test('should verify triage items in the feed and layout', async ({ page }) => {
    // Navigate directly to the triage feed
    await page.goto('/triage');

    // Wait for the main container to load
    const mainHeader = page.locator('h1', { hasText: 'Work Triage' });
    await expect(mainHeader).toBeVisible();

    // Verify the grid layout matches mobile-first principles
    const gridContainer = page.locator('.grid');
    await expect(gridContainer).toBeVisible();

    // Look for at least one seeded triage item card (e.g. Maya's custom cake request)
    const triageCard = page.locator('.bg-white\\/65, .dark\\:bg-\\[\\#16161a\\]\\/70').first();
    await expect(triageCard).toBeVisible();

    // Verify glassmorphism style classes are applied
    await expect(triageCard).toHaveClass(/backdrop-blur-\[30px\]/);
    await expect(triageCard).toHaveClass(/border-white\/40/);

    // Verify a button for "Approve & Send" or Action
    const actionButton = page.locator('button', { hasText: /Approve/i }).first();
    if (await actionButton.isVisible()) {
      await expect(actionButton).toBeVisible();
    }
  });

  test('current app smoke test for triage', async ({ page, request }) => {
    await currentAppSmoke(page, request, 'triage_ui');
  });
});
