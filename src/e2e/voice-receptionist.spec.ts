import { test, expect } from './fixtures';

test.describe('Voice Receptionist', () => {
  test('Admin can enable and configure the AI Voice Receptionist', async ({ page }) => {
    // Navigate to settings page
    await page.goto('/settings');
    await expect(page).toHaveTitle(/Settings/);

    // Assert panel is visible
    const panelTitle = page.locator('text=Autonomous Voice Receptionist');
    await expect(panelTitle).toBeVisible();

    // Enable the voice receptionist
    const enableCheckbox = page.locator('label:has-text("Enable AI Voice Receptionist") >> input[type="checkbox"]');

    // Check if it's already checked (to make test idempotent)
    const isChecked = await enableCheckbox.isChecked();
    if (!isChecked) {
      await enableCheckbox.check();
    }

    // Wait for the dependent fields to appear
    const personaSelect = page.locator('select', { hasText: 'Friendly & Casual' });
    await expect(personaSelect).toBeVisible();

    // Select a persona
    await personaSelect.selectOption({ label: 'Professional & Crisp' });

    // Click "Get Number" if number is not assigned yet
    const getNumberBtn = page.locator('button:has-text("Get Number")');
    if (await getNumberBtn.isVisible()) {
      await getNumberBtn.click();
    }

    // Verify a number is assigned
    const numberInput = page.locator('text=Assigned Phone Number >> xpath=..//input');
    await expect(numberInput).not.toHaveValue('Not assigned');
    const assignedNumber = await numberInput.inputValue();
    expect(assignedNumber).toMatch(/^\+1555123\d{4}$/);

    // Refresh page and verify settings persisted
    await page.reload();
    await expect(enableCheckbox).toBeChecked();
    await expect(personaSelect).toHaveValue('Professional');
    await expect(numberInput).toHaveValue(assignedNumber);
  });
});
