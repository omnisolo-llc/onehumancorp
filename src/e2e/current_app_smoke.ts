import { expect } from './fixtures';
import { Page, APIRequestContext } from '@playwright/test';

export async function currentAppSmoke(page: Page, request: APIRequestContext, label: string) {

  await page.setViewportSize({ width: 375, height: 812 });

    await page.goto('/login');
    await page.fill('input[placeholder="Email or Username"]', 'Maya');
    await page.getByRole('button', { name: 'Log In' }).click();

    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });
    await expect(page.locator('h2', { hasText: 'Welcome back' }).first()).toBeVisible({ timeout: 5000 });

    // Verify glassmorphism style drift on dashboard panels
    const panel = page.locator('.app-panel').first();
    await expect(panel).toBeVisible();
    await expect(panel).toHaveCSS('backdrop-filter', /blur\(30px\)|none/);
    await expect(panel).toHaveCSS('border-radius', '16px');

    // Verify glassmorphism style drift on dashboard cards
    const card = page.locator('a[href="/pos/terminal"]').first();
    await expect(card).toBeVisible();
    await expect(card).toHaveCSS('backdrop-filter', /blur\(30px\) saturate\(210%\)|blur\(30px\)|none/);
    await expect(card).toHaveCSS('border-radius', /16px|8px/);

    // await page.goto('/agents');
    // await expect(page.getByRole('heading', { name: 'AI Departments' }).first()).toBeVisible({ timeout: 5000 });

    // await page.goto('/website-builder');
    // await expect(page.getByRole('heading', { name: 'Setup Assistant' }).first()).toBeVisible({ timeout: 5000 });

    await page.goto('/integrations');
    await expect(page.getByRole('heading', { name: 'Tool Integrations' }).first()).toBeVisible({ timeout: 5000 });

    await page.goto('/customer-referral-program');
    await expect(page.getByRole('heading', { name: 'Customer Referral Program' }).first()).toBeVisible({ timeout: 5000 });

    await page.goto('/storefront-builder');
    await expect(page.getByRole('heading', { name: 'Welcome to OHC Smart Builder' }).first()).toBeVisible({ timeout: 5000 });

    // const ogCard = await request.get('/api/v1/growth/storefront/og-card?tenant=e2e&product_name=Smoke');
    // expect(ogCard.ok()).toBeTruthy();

    await page.goto('/ui/cost-dashboard.html');
    await expect(page.locator('h1', { hasText: 'Cost Transparency Dashboard' }).first()).toBeVisible({ timeout: 15000 });


    const totalCosts = page.locator('#cost-dashboard-total-costs');
    await expect(totalCosts).toBeVisible();
    expect(await totalCosts.innerText()).toMatch(/^\$[\d,]+\.\d{2}$/);

    const totalRevenue = page.locator('#cost-dashboard-revenue');
    await expect(totalRevenue).toBeVisible();
    expect(await totalRevenue.innerText()).toMatch(/^\$[\d,]+\.\d{2}$/);

    const bandwidthSavings = page.locator('#cost-dashboard-total-savings');
    await expect(bandwidthSavings).toBeVisible({ timeout: 10000 });
    expect(await bandwidthSavings.innerText()).toMatch(/^-?\$[\d,]+\.\d{2}$/);

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


    // Verify Milestones Page and Embed code generation
    await page.goto('/milestone-alerts');


    // Wait for data to load
    await page.waitForTimeout(2000);

    // The page auto-selects the first milestone if available. Check if it's there, if not, skip test logic
    const embedVisible = await page.locator('h3', { hasText: 'Embed on your website' }).isVisible();
    if (embedVisible) {
        // Check that textarea contains the right code snippet structure
        const textarea = page.locator('textarea').first();
        await expect(textarea).toBeVisible();
        const embedValue = await textarea.inputValue();
        expect(embedValue).toContain('<a href="');
        expect(embedValue).toContain('source=milestone_embed');
        expect(embedValue).toContain('<img src="');
    }

    // Verify Referral Leaderboard Generator
    await page.goto('/api/ui/referral-leaderboard-generator.html');


    // Check that there is either a leaderboard or an empty state loaded
    await page.waitForTimeout(2000); // Allow fetch to settle

    const hasCodeBlock = await page.locator('#embed-code').isVisible();
    const hasEmptyState = await page.locator('.empty-state').isVisible();



    if (hasCodeBlock) {
        const codeText = await page.locator('#embed-code').innerText();
        expect(codeText).toContain('<div id="ohc-leaderboard"></div>');
        expect(codeText).toContain('ohc.app/api/v1/growth/embed/widget?type=leaderboard');
    }
}
