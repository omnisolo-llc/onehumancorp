import { test, expect } from '@playwright/test';

test.describe('Autonomous Local SEO & Google Business Sync', () => {
  const tenantId = 'test-tenant-carlos-seo';

  test.beforeEach(async ({ context }) => {
    await context.addInitScript(() => {
      localStorage.setItem('has_onboarded', 'true');
    });
    // Mock successful Google connection by setting it directly since redirect breaks the test flow
    await context.addInitScript((tId) => {
      localStorage.setItem(`google_connected_${tId}`, 'true');
    }, tenantId);
  });

  test('should connect Google Business and approve an AI review reply', async ({ page, request }) => {
    // Navigate to Local Visibility directly
    await page.goto(`/local-visibility?tenant=${tenantId}`);

    // Check connection success text
    await expect(page.locator('text=Synced with Google Maps')).toBeVisible();

    // Trigger Review (simulate webhook event)
    const simulateBtn = page.locator('text=Simulate New Review');
    await expect(simulateBtn).toBeVisible();
    await simulateBtn.click();

    // The approval card should show up with AI drafted reply
    const approvalCard = page.locator('#review-approval-card');
    await expect(approvalCard).toBeVisible();

    await expect(approvalCard.locator('text=AI Drafted Reply')).toBeVisible();
    await expect(approvalCard.locator('text=Thank you for the 5-star review')).toBeVisible();

    // Approve the reply
    const approveBtn = page.locator('text=Approve & Reply');
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // Ensure it goes away or handles the state correctly
    await expect(approvalCard).not.toBeVisible();
  });
});
