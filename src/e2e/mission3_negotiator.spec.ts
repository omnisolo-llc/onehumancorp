import { test, expect } from './fixtures';

test.describe('Agentic Negotiator & Booker E2E', () => {

  test('Persona: Carlos (Field Service) sees a pending negotiation and approves it', async ({ page, request, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    const tenantId = adminUser.organizationId;

    // 1. Simulate an incoming booking request
    await request.post('/api/agents/webhook', {
        data: {
            tenant_id: tenantId,
            source: 'instagram',
            sender_id: 'customer-carlos-1',
            message: 'Can you fix my sink tomorrow afternoon?',
            target_language: 'English'
        }
    });

    // 2. Go to dashboard and wait for the negotiation card
    await page.goto('/dashboard.html');

    // 3. Verify the "AI Negotiation" card is visible
    const negCard = page.locator('[data-testid="negotiation-card"]');
    await expect(negCard).toBeVisible({ timeout: 15000 });

    // 4. Verify content
    await expect(negCard).toContainText(/sink/i);
    await expect(negCard).toContainText(/Availability:/i);

    // 5. Approve the negotiation
    const approveBtn = negCard.locator('button', { hasText: 'Confirm Booking' });
    await approveBtn.click();

    // 6. Verify status update (toast or disappearance)
    await expect(page.getByText(/Approved!/i)).toBeVisible();
    await expect(negCard).not.toBeVisible();
  });
});
