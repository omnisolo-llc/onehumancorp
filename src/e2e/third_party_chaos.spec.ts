import { test, expect } from '@playwright/test';

test.describe('E2E Chaos - Third Party Integrations', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('chaos@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.locator('button:has-text("Sign In"), button:has-text("Login")').click();
    await page.waitForURL('**/dashboard**');
  });

  test('should load dashboard gracefully even if analytics telemetry fails', async ({ page }) => {
    // Intercept analytics / telemetry calls and force them to fail
    await page.route('**/api/telemetry/**', async route => {
        await route.abort('failed');
    });

    // Refresh dashboard
    await page.reload();

    // Core functionality (e.g. Sales, Tasks, Recent Orders) must remain visible
    await expect(page.locator('text=/Dashboard|Overview/i').first()).toBeVisible();
    await expect(page.locator('text=/Recent Orders|Tasks/i').first()).toBeVisible();

    // Verify there are no intrusive or blocking error popups covering the screen
    const blockingError = page.locator('.modal-error, .crash-screen');
    await expect(blockingError).not.toBeVisible();
  });

  test('should handle third-party notification failure without breaking workflow', async ({ page }) => {
    // Simulate failing email/SMS notification dispatch
    await page.route('**/api/notifications/send', async route => {
      await route.fulfill({
        status: 502,
        body: '{"error": "Bad Gateway from Notification Provider"}'
      });
    });

    // Go to a view that triggers notifications
    await page.locator('button:has-text("Settings"), button:has-text("Profile")').first().click();
    await page.locator('input[type="email"]').first().fill('newemail@example.com');
    await page.locator('button:has-text("Save"), button:has-text("Update")').first().click();

    // The primary action (saving settings) should succeed locally and display success
    await expect(page.locator('text=/Saved Successfully|Updated/i')).toBeVisible();

    // It is acceptable to show a non-intrusive warning about the notification failure, but not block
    const warning = page.locator('text=/Unable to send confirmation email/i');
    if (await warning.isVisible()) {
        await expect(warning).toBeVisible();
    }
  });

  test('should handle external avatar/image CDN failures securely', async ({ page }) => {
    // Abort image CDN requests
    await page.route('**/cdn.example.com/avatars/**', async route => {
        await route.abort('name_not_resolved');
    });

    await page.locator('button:has-text("Team"), button:has-text("Users")').first().click();

    // Page must not crash; fallback avatars (e.g., initials or SVG placeholders) should render
    await expect(page.locator('.fallback-avatar, .user-initials')).toBeVisible();
  });
});
