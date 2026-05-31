import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('twilio_omnichannel');

import { test, expect } from './fixtures';

test('Twilio Omnichannel Integration - setup SMS and Notifications', async ({ page }) => {
  await page.goto('/operations/settings');

  // Look for SMS Notifications settings
  const smsToggle = page.locator('input[name="sms_notifications"]');
  if (await smsToggle.isVisible()) {
    await smsToggle.check();
    await page.getByRole('button', { name: /Save/i }).click();
    await expect(page.getByText(/Settings saved/i)).toBeVisible();
  }
});
