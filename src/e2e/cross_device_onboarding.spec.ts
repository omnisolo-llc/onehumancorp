import { test, expect } from './fixtures';

test.describe('Cross Device Onboarding CUJ', () => {
  test('Persona: Business Owner can save draft and resume cross device', async ({ page }) => {
    // 1. Navigate to onboarding using the actual route
    await page.goto('/onboarding');

    // Make sure we are on the welcome step or wizard step
    await expect(page.getByText(/Welcome|Tell us about your business/)).toBeVisible();

    // Handle Start Onboarding button if present
    const startButton = page.getByRole('link', { name: 'Start Onboarding' });
    if (await startButton.isVisible()) {
      await startButton.click({ force: true });
    }

    // Verify we are at the business setup step
    await expect(page.getByText('Tell us about your business')).toBeVisible();

    // 2. Owner enters a unique business name to ensure tests don't overlap
    const uniqueBusinessName = `Cross Device Bakery ${Date.now()}`;
    const nameInput = page.getByPlaceholder(/e.g. Maya's Custom Cakes/i);
    await nameInput.fill(uniqueBusinessName);

    // 3. User clicks Save Draft
    const saveDraftBtn = page.getByRole('button', { name: /Save Draft/i }).first();
    await saveDraftBtn.click();

    // Check for save confirmation toast or text
    await expect(page.getByText('Draft Saved!')).toBeVisible();

    // 4. Simulate a cross-device session or reload
    await page.reload();

    // Ensure we are back on the wizard setup
    await expect(page.getByText('Tell us about your business')).toBeVisible();

    // 5. Verify the business name was properly restored from the backend
    await expect(page.getByPlaceholder(/e.g. Maya's Custom Cakes/i)).toHaveValue(uniqueBusinessName, { timeout: 10000 });
  });
});
