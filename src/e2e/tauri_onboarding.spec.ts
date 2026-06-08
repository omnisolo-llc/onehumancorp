import { test, expect } from '@playwright/test';

test.describe('Tauri Onboarding Wizard Flow', () => {
  test.beforeEach(async ({ page }) => {
    await page.addInitScript(() => {
      window.localStorage.clear();

      // Inject mock __TAURI__ so the test can simulate the Tauri environment.
      // This forces the UI to take the real integration path during the Playwright test
      // and not use the generic browser fallback if we removed it.
      // E2E test runs the UI through an http server, not via Tauri context.
      window.__TAURI__ = {
        core: {
          invoke: async (cmd, args) => {
            if (cmd === 'submit_onboarding') {
              const res = await fetch('/api/onboarding/start', {
                method: 'POST',
                headers: {
                  'Content-Type': 'application/json'
                },
                body: JSON.stringify(args.request)
              });
              const data = await res.json();
              if (res.ok) {
                return data;
              } else {
                throw new Error(data.message || 'Failed to start onboarding');
              }
            }
          }
        }
      };
    });
  });

  test('Completes the onboarding flow', async ({ page }) => {
    await page.goto('/index.html');

    await expect(page.getByRole('heading', { name: "Welcome to OHC" })).toBeVisible();
    await page.getByRole('button', { name: 'Start Onboarding' }).click();

    // Setup page - Step 1
    await expect(page.getByRole('heading', { name: "What's the name of your business?" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya's Custom Cake").fill("Test Business");
    await page.getByPlaceholder("e.g. I bake custom vegan cakes").fill("Making tests pass");
    await page.getByPlaceholder("e.g. Portland, OR").fill("Remote");
    await page.getByRole('button', { name: 'Generate My Business' }).click();

    // Setup page - Step 2 (Admin)
    await expect(page.getByRole('heading', { name: "Create Admin Account" })).toBeVisible();
    await page.getByPlaceholder("e.g. Maya Smith").fill("Admin Test");
    await page.getByPlaceholder("you@example.com").fill("admin@test.com");
    await page.getByPlaceholder("••••••••").fill("password123");
    await page.getByRole('button', { name: 'Launch Store' }).click();

    // Loading State
    await expect(page.getByRole('heading', { name: "Building Your Business..." })).toBeVisible();

    // Success page
    await expect(page.getByRole('heading', { name: "You're all set!" })).toBeVisible();
    await expect(page.getByText('Workspace created for Test Business.')).toBeVisible();

    const storedTenantId = await page.evaluate(() => window.localStorage.getItem('tenant_id'));
    expect(storedTenantId).not.toBeNull();
  });

  test('Validation errors prevent launching without complete info', async ({ page }) => {
    await page.goto('/setup.html');

    // Step 1 validation
    await page.getByRole('button', { name: 'Generate My Business' }).click();
    await expect(page.getByText('Business Name must be at least 3 characters.')).toBeVisible();
    await expect(page.getByText('Please tell us what you sell.')).toBeVisible();
    await expect(page.getByText('Please tell us your location.')).toBeVisible();

    await page.getByPlaceholder("e.g. Maya's Custom Cake").fill("Test Business");
    await page.getByPlaceholder("e.g. I bake custom vegan cakes").fill("Testing");
    await page.getByPlaceholder("e.g. Portland, OR").fill("Local");
    await page.getByRole('button', { name: 'Generate My Business' }).click();

    // Step 2 validation
    await page.getByRole('button', { name: 'Launch Store' }).click();
    await expect(page.getByText('Name is required.')).toBeVisible();
    await expect(page.getByText('Please enter a valid email address')).toBeVisible();
    await expect(page.getByText('Password must be at least 8 characters and contain a number')).toBeVisible();

    // Fill invalid data
    await page.getByPlaceholder("e.g. Maya Smith").fill("Admin Test");
    await page.getByPlaceholder("you@example.com").fill("invalid-email");
    await page.getByPlaceholder("••••••••").fill("password");
    await page.getByRole('button', { name: 'Launch Store' }).click();

    await expect(page.getByText('Please enter a valid email address')).toBeVisible();
    await expect(page.getByText('Password must be at least 8 characters and contain a number')).toBeVisible();
  });

  test('Instant Build uses generic defaults and launches', async ({ page }) => {
    await page.goto('/index.html');
    await page.getByRole('button', { name: 'Instant Build' }).click();

    await expect(page.getByRole('heading', { name: "You're all set!" })).toBeVisible();
    await expect(page.getByText('Workspace created for My Instant Business.')).toBeVisible();
  });
});
