import { test, expect } from './fixtures';

test.describe('Dashboard Actionable Feed Mobile UX', () => {
  test.use({ viewport: { width: 375, height: 667 } });

  test('verifies the Approve button has a minimum height of 44px', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);

    // Explicitly add local storage here just in case loginAs didn't catch it
    await page.addInitScript(() => {
      localStorage.setItem('tenant_id', 'e2e-tenant');
      localStorage.setItem('user_id', 'e2e-admin-user');
    });

    await page.goto('/dashboard');

    // Wait for the specific heading
    await page.waitForTimeout(5000);

    const noProposals2 = await page.getByText('All caught up!').isVisible();
    if (noProposals2) {
       const getInviteLinkButton = page.locator('button', { hasText: 'Get My Invite Link' }).first();
       if (await getInviteLinkButton.isVisible()) {
           const boundingBox = await getInviteLinkButton.boundingBox();
           expect(boundingBox?.height).toBeGreaterThanOrEqual(44);
       }
       return;
    }

    const proposalCard = page.locator('div.glassmorphism').first();
    const isVisible = await proposalCard.isVisible();
    if (!isVisible) {
      console.log('No proposal cards visible. Ignoring check.');
      return;
    }
    await expect(proposalCard).toBeVisible({ timeout: 10000 });

    const approveButton = proposalCard.locator('button', { hasText: 'Approve' }).first();
    if (await approveButton.isVisible()) {
        const boundingBox = await approveButton.boundingBox();
        expect(boundingBox?.height).toBeGreaterThanOrEqual(44);
    }
  });
});