import { expect, test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test.describe('Autonomous Voice AI Phone Attendant Engine', () => {

    test('Carlos activates and configures AI Voice Attendant', async ({ page }) => {
        // 1. Carlos logs in to the dashboard
        await page.goto('/dashboard');
        await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

        // 2. Carlos navigates to the Agents section to configure his AI Receptionist
        await page.goto('/agents');
        await page.goto('/team');
        await expect(page.getByRole('heading', { name: 'Your Team' }).first()).toBeVisible();

        // Assert the voice config card exists in the UI
        const aiReceptionistConfig = page.locator('#voice-ai-config');
        await expect(aiReceptionistConfig).toBeVisible();
        await page.getByLabel('Activate AI Receptionist').check();
        await page.getByLabel('Allow AI to book appointments').check();
        await page.getByLabel('Allow AI to text callers links').check();
        await page.getByRole('button', { name: 'Save Voice Settings' }).click();
        await page.getByLabel('Primary Language').selectOption('Spanish');
        await page.getByLabel('Custom Instructions').fill('Ask if they have a ladder.');
        await page.getByRole('button', { name: 'Save Voice Settings' }).click();
        await expect(page.getByText('Voice settings updated successfully')).toBeVisible();

        // 3. Verify the Call logs exist
        const callLogsConfig = page.locator('#voice-call-logs');
        await expect(callLogsConfig).toBeVisible();
        await expect(page.getByText('No recent calls.')).toBeVisible();
    });

    currentAppSmoke('voice_attendant_regression_check');
});
