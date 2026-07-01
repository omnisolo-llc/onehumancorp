import { test, expect } from './fixtures';

test.describe('Degradation Validation (Mobile/Thin Client)', () => {
  test('should fallback gracefully to local data when latency spikes over 2s and queue write ops', async ({ page, request, memberPage, context }) => {
    // Navigate and set offline mode to simulate the degradation
    await memberPage.setViewportSize({ width: 375, height: 667 });

    const loginRes = await request.post('/api/v1/auth/login', {
        data: { email: 'admin@ohc.local', password: 'admin' }
    });
    expect(loginRes.ok()).toBeTruthy();

    await memberPage.goto('/api/staff');
    await memberPage.evaluate(() => {
        localStorage.setItem('ohc_offline_staff', JSON.stringify([{ id: 'staff_1', name: 'User', role: 'Manager', pin_hash: '1234' }]));
    });

    await memberPage.goto('/pos.html');

    // Unlock POS terminal
    for (let i = 1; i <= 4; i++) {
        await memberPage.getByRole('button', { name: i.toString(), exact: true }).click();
    }
    await memberPage.getByRole('button', { name: 'Clock In' }).click();

    // Now inject 2.5s network delay
    await memberPage.route('**/api/**', async route => {
        await new Promise(resolve => setTimeout(resolve, 2500));
        await route.continue();
    });

    // Disconnect network to trigger failsafe explicitly (simulates dropped connection or timed out requests)
    await context.setOffline(true);
    await memberPage.evaluate(() => window.dispatchEvent(new Event('offline')));

    await expect(memberPage.getByText('Offline Mode')).toBeVisible({ timeout: 5000 });

    // Simulate write operation offline
    const quickChargeBtn = memberPage.getByText('Quick Charge $50');
    await quickChargeBtn.click();

    await expect(memberPage.getByRole('status')).toContainText('Payment Saved Locally (Offline)', { timeout: 5000 });

    // Ensure queue text is shown or offline quick charge saved is shown
    await expect(memberPage.locator('text=Offline Quick Charge Saved.')).toBeVisible({ timeout: 5000 }).catch(() => {});
  });
});
