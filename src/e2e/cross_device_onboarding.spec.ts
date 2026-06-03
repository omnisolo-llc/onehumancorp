// NOTE: E2E test runs for this flow are skipped locally/in sandbox due to a Docker/PGVector permission issue.
// They will be run manually in CI or when the sandbox issue is resolved.

import { test, expect } from '@playwright/test';

test.describe('Cross Device Onboarding CUJ', () => {
  test('Persona: Business Owner can save draft and resume cross device', async ({ page, context }) => {
    // 1. Owner starts from the home page
    await page.goto('/login');
    await page.getByPlaceholder(/Email/i).fill('test@example.com');
    await page.getByPlaceholder(/Password/i).fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();

    await expect(page.getByRole('heading', { name: /Welcome/i })).toBeVisible({ timeout: 15000 });
    await page.getByRole('link', { name: /Start Onboarding/i }).click();

    // Verify it landed on the Onboarding page
    await expect(page.getByText('Tell us about your business')).toBeVisible();

    // 2. Owner enters business name
    const nameInput = page.getByPlaceholder(/e.g. Maya's Custom Cakes/i);
    await nameInput.fill('Cross Device Bakery');

    // 3. Click Save Draft
    const saveDraftBtn = page.getByRole('button', { name: /Save Draft/i }).first();
    await saveDraftBtn.click();
    await expect(page.getByText('Draft Saved!')).toBeVisible();

    // 4. Simulate a cross-device session or reload
    await page.reload();

    // 5. Verify the business name was properly restored
    await expect(page.getByPlaceholder(/e.g. Maya's Custom Cakes/i)).toHaveValue('Cross Device Bakery', { timeout: 10000 });

    // 6. Test Step 3 Validation
    // Navigate through the chat steps to reach Step 3
    await page.getByRole('button', { name: /Next/i }).click(); // Go to chat step 2
    await page.getByPlaceholder(/I bake custom vegan cakes/i).fill('Cakes');
    await page.getByRole('button', { name: /Next/i }).click(); // Go to chat step 3
    await page.getByPlaceholder(/e.g. Portland, OR/i).fill('Seattle, WA');

    // Trigger intake
    const generateBtn = page.getByRole('button', { name: /Generate My Business/i });
    await generateBtn.click();

    // Verify transition to Step 2
    await expect(page.getByRole('heading', { name: /Review Details/i })).toBeVisible({ timeout: 15000 });
    await page.getByRole('button', { name: /Next/i }).click(); // Go to Step 3

    // Verify transition to Step 3
    await expect(page.getByRole('heading', { name: /Style & Team/i })).toBeVisible({ timeout: 10000 });

    // Test email validation
    const emailInput = page.getByPlaceholder('you@example.com');
    await emailInput.fill('invalid-email');
    await page.getByRole('button', { name: /Launch Store/i }).click();
    await expect(page.getByText('Invalid email address.')).toBeVisible();

    // Test password validation
    await emailInput.fill('admin@test.com'); // Fix email
    const passwordInput = page.getByPlaceholder('••••••••');
    await passwordInput.fill('short');
    await page.getByRole('button', { name: /Launch Store/i }).click();
    await expect(page.getByText('Must be at least 8 characters.')).toBeVisible();

    // Verify button is disabled when errors exist
    await expect(page.getByRole('button', { name: /Launch Store/i })).toBeDisabled();

    // Fix password
    await passwordInput.fill('password123');

    // Verify button is enabled
    await expect(page.getByRole('button', { name: /Launch Store/i })).toBeEnabled();
  });
});
