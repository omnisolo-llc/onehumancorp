import { test, expect } from '../fixtures';

test.describe('Voice Receptionist hosted settings', () => {
  test('shows the real deployment denial without attempting to provision a number', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    const writes: string[] = [];
    page.on('request', request => {
      if (request.method() === 'POST' && new URL(request.url()).pathname.startsWith('/api/v1/settings/voice')) writes.push(request.url());
    });
    const pending = page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/settings/voice' && response.request().method() === 'GET');
    await page.goto('/settings');
    const response = await pending;
    expect(response.status()).toBe(403);
    expect(await response.json()).toEqual({ success: false, error: 'hosted_global_provisioning_unavailable', provisioning_available: false, provisioning_block_reason: 'hosted_global_provisioning_unavailable' });
    await expect(page.getByText('Voice settings are unavailable in this deployment. No provider action can be started.', { exact: true })).toBeVisible();
    await expect(page.getByRole('checkbox', { name: 'Enable AI Voice Receptionist', exact: true })).toBeDisabled();
    await expect(page.getByRole('button', { name: 'Get Number', exact: true })).toHaveCount(0);
    await expect(page.getByRole('textbox', { name: 'Phone number', exact: true })).toHaveCount(0);
    expect(writes).toEqual([]);
  });
});
