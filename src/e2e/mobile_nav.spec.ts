import { test, expect } from './fixtures';

test.describe('Mobile Bottom Navigation CUJ', () => {
  // Use a mobile viewport
  test.use({ viewport: { width: 375, height: 812 } });

  test('Persona: Business Owner uses bottom nav on mobile device', async ({ page }) => {
    // 1. Owner starts from the home page after user login via the UI
    await page.goto('/login');
    await page.getByPlaceholder(/Email/i).fill('test@example.com');
    await page.getByPlaceholder(/Password/i).fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();

    // Verify we landed on the Dashboard
    await expect(page.getByRole('heading', { name: /Dashboard/i }).first()).toBeVisible({ timeout: 15000 });

    // 2. Verify the mobile bottom nav is visible
    const bottomNav = page.locator('#mobile-bottom-nav');
    await expect(bottomNav).toBeVisible();

    // 3. Verify touch targets are at least 44x44px
    const homeBtn = bottomNav.getByRole('button', { name: /Home/i });
    const homeBox = await homeBtn.boundingBox();
    expect(homeBox).not.toBeNull();
    if (homeBox) {
        expect(homeBox.width).toBeGreaterThanOrEqual(44);
        expect(homeBox.height).toBeGreaterThanOrEqual(44);
    }

    // 4. Tap Messages
    const messagesBtn = bottomNav.getByRole('button', { name: /Messages/i });
    await messagesBtn.click();
    await expect(page.locator('#inbox-screen')).toBeVisible();
    await expect(page.locator('#dashboard-screen')).not.toBeVisible();

    // 5. Tap Calendar
    const calendarBtn = bottomNav.getByRole('button', { name: /Calendar/i });
    await calendarBtn.click();
    await expect(page.locator('#meetings-screen')).toBeVisible();
    await expect(page.locator('#inbox-screen')).not.toBeVisible();

    // 6. Tap Setup
    const setupBtn = bottomNav.getByRole('button', { name: /Setup/i });
    await setupBtn.click();
    await expect(page.locator('#storefront-builder-screen')).toBeVisible();
    await expect(page.locator('#meetings-screen')).not.toBeVisible();

    // 7. Tap Home to return
    await homeBtn.click();
    await expect(page.locator('#dashboard-screen')).toBeVisible();
    await expect(page.locator('#storefront-builder-screen')).not.toBeVisible();
  });
});

// NOTE: This test verifies the new mobile bottom nav.
// Local Playwright E2E execution via Bazel may fail with Docker overlayfs mounting
// errors in some sandbox environments. These tests rely on the real backend.
