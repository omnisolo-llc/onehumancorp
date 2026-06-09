import { test, expect } from '@playwright/test';

test.describe('Promoter Agent - Social Media Auto-Post', () => {
  // Use e2e-tenant since it gets seeded
  test.use({ storageState: { cookies: [], origins: [{ origin: 'http://localhost:3000', localStorage: [{ name: 'tenant_id', value: 'e2e-tenant' }, { name: 'tenant', value: 'e2e-tenant' }] }] } });

  test('Agent feed displays Promoter draft card and allows approval', async ({ page }) => {
    // 1. Log in
    await page.goto('/login');
    await page.fill('input[type="email"]', 'admin@ohc.local');
    await page.fill('input[type="password"]', 'changeme');
    await page.click('button:has-text("Log In")');

    // Wait for dashboard to settle
    await expect(page).toHaveURL(/\/dashboard/);

    // Switch to unified agent feed tab explicitly just in case
    await page.goto('/dashboard');
    const agentFeedTab = page.getByTestId('tab-agent-feed');
    if (await agentFeedTab.isVisible()) {
        await agentFeedTab.click();
    }

    // 2. Verify Promoter Agent Social Post Draft Card
    const socialPostCard = page.getByTestId('social-post-draft-card');
    await expect(socialPostCard).toBeVisible();

    // Verify content of the social post draft
    await expect(socialPostCard).toContainText('Promoter Agent Drafts');
    await expect(socialPostCard).toContainText('Check out our new product on TikTok!');

    // 3. Approve & Schedule
    const approveBtn = page.getByTestId('approve-social-post');
    await expect(approveBtn).toBeVisible();
    await expect(approveBtn).toHaveText('Approve & Schedule');
    await approveBtn.click();

    // 4. Verification: After clicking approve, it handles the decision and the card should eventually disappear from DRAFT view
    await expect(socialPostCard).not.toBeVisible({ timeout: 10000 });
  });
});
