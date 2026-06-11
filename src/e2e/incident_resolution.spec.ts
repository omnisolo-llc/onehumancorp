import { test, expect } from '@playwright/test';

test.describe('AI Incident Resolution & Escalation Assistant Flow', () => {
  const MOCK_TENANT_ID = 'test_tenant_incident';

  test.beforeEach(async ({ page, request }) => {
    // Clear feed before starting the test to ensure a clean state
    try {
      await request.post('http://localhost:3000/api/v1/auth/mock-login', {
        data: {
          tenant_id: MOCK_TENANT_ID,
          role: 'owner',
        }
      });
    } catch(e) {}
  });

  test('Owner can log an incident and approve the AI resolution plan', async ({ page }) => {
    // 1. Log in / bypass login locally
    await page.goto('/dashboard');

    // We add local storage to simulate auth
    await page.evaluate(() => {
        localStorage.setItem('tenant_id', 'test_tenant_incident');
        localStorage.setItem('user_id', 'test_user');
        // If the dashboard uses a token
        localStorage.setItem('token', 'mock_token');
    });

    await page.reload();

    // Wait for the Dashboard
    await expect(page.getByTestId('report-incident-btn')).toBeVisible({ timeout: 15000 });

    // 2. Click the new "Report Incident" button (this navigates to /incidents)
    await page.getByTestId('report-incident-btn').click();
    await expect(page).toHaveURL(/\/incidents/);

    // 3. Fill in the incident description
    await page.getByTestId('incident-description').fill('Espresso machine down');
    await page.getByTestId('submit-incident').click();

    // 4. Returns to Dashboard. Check the Triage feed for the new incident.
    await expect(page).toHaveURL(/\/dashboard/);

    // It might take a few seconds to poll or fetch
    const incidentCard = page.getByTestId('incident-resolution-card').first();
    await expect(incidentCard).toBeVisible({ timeout: 15000 });

    // Verify content of the card
    await expect(incidentCard.locator('text=CRITICAL INCIDENT')).toBeVisible();
    await expect(incidentCard.locator('text=Espresso machine down')).toBeVisible();

    // Verify the resolution plan actions are listed
    await expect(page.locator('text=text_repair_tech').first()).toBeVisible();
    await expect(page.locator('text=refund_pending_orders').first()).toBeVisible();

    // 5. Approve the resolution plan
    await page.getByTestId('approve-incident-resolution').first().click();

    // 6. Verify it optimisticly disappears from feed
    await expect(page.getByTestId('incident-resolution-card')).toHaveCount(0);

    // Check if it appears in Activity tab
    await page.getByTestId('feed-tab-activity').click();
    // In activity, the feed item should be listed as "APPROVED"
    await expect(page.locator('text=APPROVED').first()).toBeVisible({ timeout: 10000 });
  });
});
