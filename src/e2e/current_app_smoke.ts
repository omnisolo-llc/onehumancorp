import { expect, test } from './fixtures';

export function currentAppSmoke(label: string) {
  test(`current embedded app smoke: ${label}`, async ({ page, request }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();
    await expect(page.getByText('Business Snapshot').first()).toBeVisible();

    // Verify Advanced Developer Settings Toggle and Persistence
    await expect(page.getByText('Advanced Developer Settings').first()).toBeVisible();
    await expect(page.getByText('Technical Payload:').first()).not.toBeVisible();

    // Check initial state
    let isAdvancedActive = await page.evaluate(() => localStorage.getItem('advanced_developer_settings') === 'true');
    expect(isAdvancedActive).toBeFalsy();

    // Click the toggle button which is next to the text
    await page.locator('span:text("Advanced Developer Settings") + button').click();

    // Verify state updated in localStorage
    isAdvancedActive = await page.evaluate(() => localStorage.getItem('advanced_developer_settings') === 'true');
    expect(isAdvancedActive).toBeTruthy();

    // Reload page to check persistence
    await page.reload();
    await expect(page.getByText('Advanced Developer Settings').first()).toBeVisible();

    // Verify the state remains true in localStorage
    isAdvancedActive = await page.evaluate(() => localStorage.getItem('advanced_developer_settings') === 'true');
    expect(isAdvancedActive).toBeTruthy();

    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'AI Departments' }).first()).toBeVisible();
    await expect(page.getByRole('button', { name: /The Ambassador/ }).first()).toBeVisible();

    await page.goto('/website-builder');
    await expect(page.getByRole('heading', { name: '10-Minute Setup Wizard' }).first()).toBeVisible();

    await page.goto('/integrations');
    await expect(page.getByRole('heading', { name: 'Connect Custom Software' }).first()).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Social Media Accounts' }).first()).toBeVisible();

    await page.goto('/referrals');
    await expect(page.getByRole('heading', { name: 'Referral Dashboard' }).first()).toBeVisible();

    await page.goto('/storefront-builder');
    await expect(page.locator('.builder-block').first()).toBeVisible();

    const ogCard = await request.get('/api/v1/growth/storefront/og-card?tenant=e2e&product_name=Smoke');
    expect(ogCard.ok()).toBeTruthy();
    expect(ogCard.headers()['content-type']).toContain('image/svg+xml');
  });
}
