import { test, expect } from './fixtures';

test.describe('Voice Receptionist deployment availability', () => {
  test('hosted admin sees unavailable voice settings and cannot start provisioning', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    const voiceWrites: string[] = [];
    page.on('request', request => {
      const path = new URL(request.url()).pathname;
      if (request.method() === 'POST' && (path === '/api/v1/settings/voice' || path === '/api/v1/settings/voice/provision')) voiceWrites.push(path);
    });

    // The hosted fixture has no tenant-bound provisioning authority. Assert its
    // real read contract before interacting; never attempt a phone purchase.
    const response = page.waitForResponse(result => new URL(result.url()).pathname === '/api/v1/settings/voice' && result.request().method() === 'GET');
    await page.goto('/settings');
    const settingsResponse = await response;
    expect(settingsResponse.status()).toBe(403);
    expect(await settingsResponse.json()).toMatchObject({ success: false, error: 'hosted_global_provisioning_unavailable', provisioning_available: false });
    await expect(page).toHaveTitle(/Settings/);
    await expect(page.getByText('Autonomous Voice Receptionist', { exact: true })).toBeVisible();
    await expect(page.getByText('Voice settings are unavailable in this deployment. No provider action can be started.', { exact: true })).toBeVisible();
    await expect(page.getByRole('checkbox', { name: 'Enable AI Voice Receptionist', exact: true })).toBeDisabled();
    await expect(page.getByRole('button', { name: 'Get Number', exact: true })).toHaveCount(0);
    await expect(page.getByRole('textbox', { name: 'Phone number', exact: true })).toHaveCount(0);
    expect(voiceWrites).toEqual([]);

    await page.reload();
    await expect(page.getByText('Voice settings are unavailable in this deployment. No provider action can be started.', { exact: true })).toBeVisible();
    await expect(page.getByRole('checkbox', { name: 'Enable AI Voice Receptionist', exact: true })).toBeDisabled();
    expect(voiceWrites).toEqual([]);
  });
});
