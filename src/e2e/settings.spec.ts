import { test, expect } from './fixtures';
import { createGrowthOwner } from './growth_owner';

test.describe('Settings Page', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/settings');
    await expect(page.locator('#settings-screen')).toBeVisible();
  });

  test('shows general notification settings', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Settings' })).toBeVisible();
    await expect(page.getByText('Enable Email Notifications')).toBeVisible();
    await expect(page.getByText('Enable Push Notifications')).toBeVisible();
    await expect(page.getByText('Timezone')).toBeVisible();
    await expect(page.getByText('Language', { exact: true })).toBeVisible();
  });

  test('toggles delivery settings', async ({ page }) => {
    await page.getByLabel('Enable Email Notifications').check();
    await page.getByLabel('Enable Push Notifications').check();
    await page.getByLabel('Enable Local Delivery').check();

    await expect(page.getByLabel('Enable Email Notifications')).toBeChecked();
    await expect(page.getByLabel('Enable Push Notifications')).toBeChecked();
    await expect(page.getByLabel('Enable Local Delivery')).toBeChecked();
    await expect(page.getByLabel('Delivery Radius (miles)')).toBeEnabled();
    await expect(page.getByLabel('Flat Delivery Fee ($)')).toBeEnabled();
  });

  test('shows SMS alert and delivery settings fields', async ({ page, baseURL, adminUser, loginAs }) => {
    const owner = await createGrowthOwner(page, baseURL);
    await loginAs(page, { ...adminUser, email: owner.email, organizationId: owner.tenantId });
    const origin = new URL(baseURL!).origin;
    const writes: string[] = [];
    page.on('request', request => {
      const url = new URL(request.url());
      if (url.origin === origin && request.method() === 'POST'
        && /^\/api\/v1\/settings\/sms-/.test(url.pathname)) writes.push(url.pathname);
    });
    const loaded = page.waitForResponse(response => new URL(response.url()).origin === origin
      && new URL(response.url()).pathname === '/api/v1/settings/sms-preferences'
      && response.request().method() === 'GET');
    await page.goto('/settings');
    const response = await loaded;
    expect(response.status()).toBe(200);
    expect(await response.json()).toMatchObject({
      success: true, organization_id: owner.tenantId, user_id: owner.userId,
      status: 'unverified', phone: null, verification_id: null, challenge: null,
      provider_configured: false,
      preferences: { urgent_booking: false, failed_payment: false, new_order: false },
    });
    await expect(page.getByText('Critical SMS Alerts')).toBeVisible();
    const phone = page.getByRole('textbox', { name: 'Mobile Number', exact: true });
    await expect(phone).toBeVisible();
    await expect(phone).toBeEnabled();
    await phone.fill('+15550102020');
    await expect(page.getByRole('button', { name: 'Verify Number', exact: true })).toBeDisabled();
    await expect(page.getByText('SMS verification is unavailable because the provider is not configured.', { exact: true })).toBeVisible();
    for (const name of ['Urgent Bookings', 'Failed Payments', 'New Orders']) {
      const preference = page.getByRole('checkbox', { name, exact: true });
      await expect(preference).toBeVisible();
      await expect(preference).not.toBeChecked();
      await expect(preference).toBeDisabled();
    }
    await expect(page.getByText('Local Delivery (DoorDash Drive)')).toBeVisible();
    await expect(page.getByText('Enable Local Delivery')).toBeVisible();
    await expect(page.getByText('Delivery Radius (miles)')).toBeVisible();
    await expect(page.getByText('Flat Delivery Fee ($)')).toBeVisible();
    expect(writes).toEqual([]);
  });
});
