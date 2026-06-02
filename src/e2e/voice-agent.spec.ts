import { test, expect } from '@playwright/test';

// CUJ: A non-technical business owner, Fatima (Food Cart Operator), wants to enable the
// AI Voice Receptionist to take phone pre-orders while she is cooking.
// She navigates to the Voice Agent settings, toggles the receptionist on,
// sets her language to Arabic, adds custom instructions about her food cart location,
// and saves the settings successfully.

test('Fatima configures her AI Voice Receptionist', async ({ page }) => {

  // Navigate to Voice Agent Tab
  await page.goto('http://localhost:3000/voice-agent');

  // Verify page loaded successfully
  await expect(page.getByRole('heading', { name: 'Voice AI Receptionist' })).toBeVisible();

  // Verify initial mock data is displayed
  await expect(page.getByText('(555) 123-4567')).toBeVisible();

  // Fatima toggles the AI Receptionist ON
  const enableCheckbox = page.getByRole('checkbox');
  await expect(enableCheckbox).not.toBeChecked();
  await enableCheckbox.check();
  await expect(enableCheckbox).toBeChecked();

  // Fatima changes her primary language to Arabic
  const languageSelect = page.getByRole('combobox');
  await languageSelect.selectOption('Arabic');
  await expect(languageSelect).toHaveValue('Arabic');

  // Fatima adds custom instructions
  const instructionsBox = page.getByPlaceholder("e.g., 'Tell callers to park in the back'");
  await instructionsBox.fill('I am located at the corner of 5th and Main. Wait time is 15 minutes.');
  await expect(instructionsBox).toHaveValue('I am located at the corner of 5th and Main. Wait time is 15 minutes.');

  // Fatima clicks Save
  const saveButton = page.getByRole('button', { name: 'Save Settings' });
  await saveButton.click();

  // Verify success message is shown
  await expect(page.getByText('Voice settings saved successfully.')).toBeVisible();
});
