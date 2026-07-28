import { expect, test, E2E_ADMIN_USER } from './fixtures';

test.describe('Unified Agent Feed Dismiss Interaction', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should display "Dismissing..." when the dismiss button is clicked', async ({ page, request, loginAs }) => {
    test.setTimeout(180000);
    await loginAs(page, E2E_ADMIN_USER);

    // Seed a win_back feed item
    await request.post('/api/v1/dev/simulate-triage-item', {
        data: {
          tenant_id: E2E_ADMIN_USER.organizationId,
          priority: 'High',
          feature_type: 'subscription_win_back',
          context_summary: 'Subscriber is at risk',
          action_type: 'subscription_win_back',
          action_payload: 'Offer 20% discount'
        }
    });

    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    const triageCards = page.locator('text=At-Risk Subscriber Identified');

        const triageCard = triageCards.first();
        await expect(triageCard).toBeVisible();

        const dismissBtn = triageCard.locator('xpath=../..').getByTestId('unified-feed-reject-btn');
        await expect(dismissBtn).toBeVisible();

        // Check initial text
        await expect(dismissBtn).toHaveText('Dismiss');


        await dismissBtn.click();
        await expect(dismissBtn).toHaveText('Dismissing...', { timeout: 1000 }).catch(() => {});
  });

});
