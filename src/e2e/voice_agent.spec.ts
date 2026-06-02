import { test, expect } from '@playwright/test';

test.describe('Voice Agent Dashboard Configuration', () => {
  test.beforeEach(async ({ page }) => {
    // Add mock token to bypass auth if required by the test framework
    await page.addInitScript(() => {
      localStorage.setItem('token', 'e2e-test-token');
      localStorage.setItem('tenant', 'e2e-tenant');
    });

    // Mock API response for voice config GET
    await page.route('**/api/v1/voice/config', async (route) => {
      if (route.request().method() === 'GET') {
        await route.fulfill({
          status: 200,
          json: {
            phone_number: '(555) 123-4567',
            is_enabled: false,
            primary_language: 'English',
            custom_instructions: ''
          }
        });
      } else if (route.request().method() === 'POST') {
        const payload = JSON.parse(route.request().postData() || '{}');
        expect(payload.is_enabled).toBe(true);
        expect(payload.primary_language).toBe('Arabic');
        expect(payload.custom_instructions).toContain('halal items only');
        await route.fulfill({
          status: 200,
          json: { status: 'success' }
        });
      }
    });

    // Mock metrics response to render dashboard
    await page.route('**/api/v1/dashboard/metrics', async (route) => {
      await route.fulfill({
        status: 200,
        json: {
          metrics: { today_sales: 0, active_orders: 0, pending_tasks: 0, unread_messages: 0 }
        }
      });
    });
  });

  test('Owner configures AI Voice Receptionist successfully', async ({ page }) => {
    await page.goto('/dashboard');

    // Check that section is visible
    await expect(page.locator('h2:has-text("AI Voice Receptionist")')).toBeVisible();

    // Verify default state
    const toggle = page.locator('input[type="checkbox"]').first();
    await expect(toggle).not.toBeChecked();

    // Toggle on
    await toggle.click({ force: true });

    // Change language
    const languageSelect = page.locator('select').first();
    await languageSelect.selectOption('Arabic');

    // Add custom instructions
    const instructionsTextarea = page.locator('textarea[placeholder*="park in the back"]').first();
    await instructionsTextarea.fill('Please tell callers we serve halal items only. Pre-orders take 15 mins.');

    // Save
    const saveButton = page.locator('button:has-text("Save Voice Settings")').first();
    await saveButton.click({ force: true });

    // Assert success message is displayed
    await expect(page.locator('text=Voice settings updated successfully')).toBeVisible({ timeout: 5000 });
  });
});
