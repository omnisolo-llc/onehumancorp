import { test, expect } from '@playwright/test';

test.describe('Cross Device Onboarding CUJ', () => {
  test('Persona: Business Owner can save draft and resume cross device', async ({ page }) => {
    // 1. Owner starts onboarding directly from the current route.
    await page.goto('/onboarding');
    await expect(page.getByText('Welcome')).toBeVisible();
    await page.getByText('Start Onboarding').click();

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
  });
});
