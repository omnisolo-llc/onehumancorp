import { test, expect } from '@playwright/test';

test.describe('AI Voice Receptionist Onboarding', () => {
  test('should allow merchant to toggle voice receptionist and get a number', async ({ page }) => {
    // 1. Log in
    await page.goto('http://localhost:3000/login');
    await page.fill('input[name="email"]', 'merchant@example.com');
    await page.fill('input[name="password"]', 'password123');
    await page.click('button[type="submit"]');

    // Wait for dashboard to load
    await page.waitForURL('/dashboard');

    // 2. Navigate to Settings
    await page.goto('/settings');

    // 3. Find the AI Voice Receptionist section
    const voiceSection = page.locator('text=AI Voice Receptionist').locator('..').locator('..');

    // 4. Toggle "Enable AI Voice Receptionist"
    const toggle = voiceSection.locator('input[type="checkbox"]');

    // Check initial state, if not checked, check it
    const isChecked = await toggle.isChecked();
    if (!isChecked) {
      await toggle.click();
    }

    // 5. Verify the conditionally rendered UI is visible
    await expect(voiceSection.locator('text=Voice Persona')).toBeVisible();

    // 6. Click "Get Number" if it exists
    const getNumberBtn = voiceSection.locator('button:has-text("Get Number")');
    if (await getNumberBtn.isVisible()) {
      await getNumberBtn.click();

      // Verify a number is assigned (mock logic will generate something like +1555...)
      const numberInput = voiceSection.locator('input[aria-label="Assigned Phone Number"]');
      await expect(numberInput).not.toHaveValue('Not assigned');
      await expect(numberInput).toHaveValue(/\+1555/);
    }

    // 7. Verify the persona selector works
    const personaSelect = voiceSection.locator('select');
    await personaSelect.selectOption('Professional');
    await expect(personaSelect).toHaveValue('Professional');

    // 8. Refresh and ensure settings are persisted
    await page.reload();

    const refreshedToggle = voiceSection.locator('input[type="checkbox"]');
    await expect(refreshedToggle).toBeChecked();

    const refreshedPersona = voiceSection.locator('select');
    await expect(refreshedPersona).toHaveValue('Professional');

    const refreshedNumber = voiceSection.locator('input[aria-label="Assigned Phone Number"]');
    await expect(refreshedNumber).not.toHaveValue('Not assigned');
  });
});
