import { expect, test } from './fixtures';

export function currentAppSmoke(label: string) {
  test(`current embedded app smoke: ${label}`, async ({ page, request }) => {
    test.setTimeout(180000);

    await page.setViewportSize({ width: 375, height: 812 });

    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });
    await expect(page.getByText('Welcome back, Human.')).toBeVisible({ timeout: 5000 });

    // Verify glassmorphism style drift on dashboard panels
    const panel = page.locator('.app-panel').first();
    await expect(panel).toBeVisible();
    await expect(panel).toHaveCSS('backdrop-filter', /blur\(30px\)/);
    await expect(panel).toHaveCSS('border-radius', '16px');

    // Verify glassmorphism style drift on dashboard cards
    const card = page.locator('.app-card').first();
    await expect(card).toBeVisible();
    await expect(card).toHaveCSS('backdrop-filter', /blur\(30px\)/);
    await expect(card).toHaveCSS('border-radius', '16px');

    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'AI Departments' }).first()).toBeVisible({ timeout: 5000 });

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

    await page.goto('/cost-dashboard');
    await expect(page.locator('h1', { hasText: 'Business Advisory Dashboard' }).first()).toBeVisible({ timeout: 15000 });
    await expect(page.locator('h2', { hasText: 'Cost Transparency' })).toBeVisible();

    const totalCosts = page.locator('#cost-dashboard-total');
    await expect(totalCosts).toBeVisible();
    expect(await totalCosts.innerText()).toMatch(/^\$[\d,]+\.\d{2}$/);

    const totalRevenue = page.locator('#cost-dashboard-revenue');
    await expect(totalRevenue).toBeVisible();
    expect(await totalRevenue.innerText()).toMatch(/^\$[\d,]+\.\d{2}$/);

    const bandwidthSavings = page.locator('#cost-dashboard-total-savings');
    await expect(bandwidthSavings).toBeVisible();
    expect(await bandwidthSavings.innerText()).toMatch(/^\$[\d,]+\.\d{2}$/);

    await expect(page.locator('h2', { hasText: 'Cost Breakdown' })).toBeVisible();
    await expect(page.locator('h3', { hasText: 'Agent & Feature Costs' })).toBeVisible();

    const llmCost = page.locator('#cost-dashboard-llm');
    await expect(llmCost).toBeVisible();
    expect(await llmCost.innerText()).toMatch(/^\$[\d,]+\.\d{2}$/);

    const storageCost = page.locator('#cost-dashboard-storage');
    await expect(storageCost).toBeVisible();
    expect(await storageCost.innerText()).toMatch(/^\$[\d,]+\.\d{2}$/);

    const paymentFees = page.locator('#cost-dashboard-payment-fees');
    await expect(paymentFees).toBeVisible();
    expect(await paymentFees.innerText()).toMatch(/^\$[\d,]+\.\d{2}$/);

    const networkCost = page.locator('#cost-dashboard-network');
    await expect(networkCost).toBeVisible();
    expect(await networkCost.innerText()).toMatch(/^\$[\d,]+\.\d{2}$/);
  });
}
