import { expect, test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test.describe('Autonomous Voice AI Phone Attendant Engine', () => {

    test('Carlos activates and configures AI Voice Attendant', async ({ page }) => {
        // 1. Carlos logs in to the dashboard
        await page.goto('/dashboard');
        await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

        // 2. Carlos navigates to the Agents section to configure his AI Receptionist
        await page.goto('/agents');
        await expect(page.getByRole('heading', { name: 'AI Departments' }).first()).toBeVisible();

        await expect(page.getByRole('button', { name: /The Ambassador/ }).first()).toBeVisible();
        await expect(page.getByRole('button', { name: /The Manager/ }).first()).toBeVisible();
    });

    test('smoke', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'voice_attendant_regression_check');
});
});
