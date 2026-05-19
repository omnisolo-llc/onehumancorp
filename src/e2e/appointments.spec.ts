import { test, expect } from '@playwright/test';

test.describe('🎨 Canvas: Unified Appointment Scheduling', () => {
  test('should allow user to view and interact with the Appointments dashboard', async ({ page }) => {
    await page.goto('/');
    await page.waitForTimeout(1000);

    const isLoginVisible = await page.isVisible('text="Login to your account"');
    const isSignupVisible = await page.isVisible('text="Create an account"');

    if (isSignupVisible && !isLoginVisible) {
       await page.click('text="Have an account? Sign In"');
    }

    await page.fill('input[placeholder="Email or Username"]', 'testuser@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');
    await page.click('button:has-text("Sign In")');

    await expect(page.locator('#dashboard-screen')).toBeVisible();
    await page.click('button:has-text("Appointments")');

    await expect(page.locator('#appointments-screen')).toBeVisible();
    await expect(page.locator('h1:has-text("Appointments")')).toBeVisible();

    const connectBtn = page.locator('button#connect-cal-btn');
    await expect(connectBtn).toBeVisible();
    await connectBtn.click();

    await expect(connectBtn).toHaveText('Cal.com Connected');
    await expect(page.locator('#cal-connected-state')).toBeVisible();

    await expect(page.locator('text="Consultation Call"')).toBeVisible();
    await expect(page.locator('text="Website Review"')).toBeVisible();

    await page.click('button:has-text("Advanced Settings")');

    const advancedPanel = page.locator('#advanced-settings-panel');
    await expect(advancedPanel).toBeVisible();
    await expect(advancedPanel.locator('h3:has-text("Advanced Developer Settings")')).toBeVisible();
    await expect(advancedPanel.locator('input[type="password"]')).toHaveValue('cal_live_xxxxxxxxxxxxxxxx');

    page.on('dialog', dialog => dialog.accept());
    await advancedPanel.locator('button:has-text("Save Advanced Configuration")').click();

    await expect(advancedPanel).toBeHidden();
  });
});
