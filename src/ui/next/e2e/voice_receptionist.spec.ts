import { test, expect } from '@playwright/test';

// CUJ: VoiceDesk Configuration Flow
// 1. Owner navigates to the Team Page.
// 2. Owner clicks on the "VoiceDesk AI" configuration link.
// 3. Owner arrives at the Phone Receptionist settings page.
// 4. Owner fills out the form (selects language, provides instructions).
// 5. Owner toggles the "Enable AI Receptionist" to ON.
// 6. Owner clicks "Save Settings".
// 7. Verify the "Saved" state appears.

test.describe('VoiceDesk AI Receptionist E2E', () => {
  test('Owner can configure and enable the VoiceDesk AI Receptionist', async ({ page }) => {
    // Navigate to the Team page
    await page.goto('http://localhost:3000/team');

    // Ensure the Team Page loaded
    await expect(page.getByRole('heading', { name: 'Your Team' })).toBeVisible();

    // Click on VoiceDesk AI
    // Depending on hydration, we might need to wait for it.
    await page.waitForSelector('text=VoiceDesk AI', { state: 'visible' });
    await page.locator('a', { hasText: 'VoiceDesk AI' }).first().click();

    // Wait for the Phone Receptionist page to load
    await expect(page.getByRole('heading', { name: 'Phone Receptionist' })).toBeVisible();

    // Toggle the "Let AI answer my missed calls" to ON FIRST to enable the form
    await page.getByText('Let AI answer my missed calls').locator('..').getByRole('button').click();

    // Fill out the instructions
    const instructionsInput = page.getByPlaceholder(/e.g., I'm Carlos/);
    await instructionsInput.fill('Always be extremely polite and offer them a 10% discount if they book today.');

    // Select Spanish language
    await page.locator('button:has-text("Español")').click({ force: true });

    // Select Professional voice
    await page.locator('button:has-text("Professional")').click({ force: true });

    // Click "Save Settings"
    const saveButton = page.getByRole('button', { name: /Save Settings|Saved/ });
    await saveButton.click();

    // Verify "Saved" state
    await expect(page.getByText('Saved')).toBeVisible();
  });
});
