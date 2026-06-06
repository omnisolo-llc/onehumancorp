import { expect, test } from './fixtures';

export function currentAppSmoke(label: string) {
  test(`current embedded app smoke: ${label}`, async ({ page, request }) => {
    test.setTimeout(180000);

    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });
    await expect(page.getByText('Welcome back, Human.')).toBeVisible({ timeout: 5000 });

    // Verify glassmorphism style drift on dashboard panels
    const panel = page.locator('.app-panel').first();
    await expect(panel).toBeVisible();
    await expect(panel).toHaveCSS('backdrop-filter', /blur\(20px\)/);
    await expect(panel).toHaveCSS('border-radius', '16px');

    // Verify glassmorphism style drift on dashboard cards
    const card = page.locator('.app-card').first();
    await expect(card).toBeVisible();
    await expect(card).toHaveCSS('backdrop-filter', /blur\(20px\)/);
    await expect(card).toHaveCSS('border-radius', '16px');

    await page.goto('/agents');
    await expect(page.locator('h1', { hasText: 'AI Departments' }).first()).toBeVisible({ timeout: 5000 });

    await page.goto('/website-builder');
    await expect(page.getByRole('heading', { name: '10-Minute Setup Wizard' }).first()).toBeVisible({ timeout: 5000 });

    await page.goto('/integrations');
    await expect(page.getByRole('heading', { name: 'Tool Integrations' }).first()).toBeVisible({ timeout: 5000 });

    await page.goto('/referrals');
    await expect(page.getByRole('heading', { name: 'Referral Dashboard' }).first()).toBeVisible({ timeout: 5000 });

    await page.goto('/storefront-builder');
    await expect(page.getByRole('heading', { name: 'Welcome to OHC Smart Builder' }).first()).toBeVisible({ timeout: 5000 });

    const ogCard = await request.get('/api/v1/growth/storefront/og-card?tenant=e2e&product_name=Smoke');
    expect(ogCard.ok()).toBeTruthy();
  });
}
