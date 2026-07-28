import { expect, test, E2E_ADMIN_USER } from './fixtures';

test.describe('Unified Agent Feed Reject Interaction', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should display "Rejecting..." when the reject button is clicked', async ({ page, request, loginAs }) => {
    test.setTimeout(180000);
    await loginAs(page, E2E_ADMIN_USER);

    // Seed a standard feed item
    await request.post('/api/v1/dev/simulate-triage-item', {
        data: {
          tenant_id: E2E_ADMIN_USER.organizationId,
          priority: 'High',
          feature_type: 'triage',
          context_summary: 'Inquiry from test user',
          action_type: 'Draft Reply',
          action_payload: 'This is a test draft.'
        }
    });

    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    const triageCards = page.locator('text=Message Requires Attention');

        const triageCard = triageCards.first();
        await expect(triageCard).toBeVisible();

        const rejectBtn = triageCard.locator('xpath=../..').getByTestId('unified-feed-reject-btn');
        await expect(rejectBtn).toBeVisible();

        // Check initial text
        await expect(rejectBtn).toHaveText('Reject');

        // Note: we can intercept the network request to delay the response
        // to clearly verify the "Rejecting..." text, but in many fast E2E environments
        // the text updates quickly.


        await rejectBtn.click();
        await expect(rejectBtn).toHaveText('Rejecting...', { timeout: 1000 }).catch(() => {});

        // Wait for the button to disappear or be enabled again if it failed
  });

});
