import { test, expect } from './fixtures';

test.describe('Client Intake Proposal', () => {
  test('should display client intake proposal draft and allow approval', async ({ page }) => {
    // Explicitly set e2e-tenant to ensure we load the seeded data
    await page.addInitScript(() => {
      localStorage.setItem('tenant_id', 'e2e-tenant');
      localStorage.setItem('user_id', 'e2e-admin-user');
    });

    // Go to dashboard
    await page.goto('/dashboard');

    // Wait for the approvals to load
    await expect(page.locator('button', { hasText: 'Proposals' }).first()).toBeVisible();

    // Verify the proposal card appears
    await expect(page.getByText('Draft proposal for ACME Corp Branding')).toBeVisible();

    // Verify card content
    await expect(page.getByText('Intake Request:')).toBeVisible();
    await expect(page.getByText('"ACME wants a logo refresh and 3-page site"')).toBeVisible();
    await expect(page.getByText('Proposed Scope & Pricing')).toBeVisible();
    await expect(page.getByText('Logo refresh and 3-page site including mobile responsive design.')).toBeVisible();
    await expect(page.getByText('Timeline:')).toBeVisible();
    await expect(page.getByText('Next Monday')).toBeVisible();
    await expect(page.getByText('Estimated Price:')).toBeVisible();
    await expect(page.getByText('$1200.00')).toBeVisible();
    await expect(page.getByText('Draft Reply:')).toBeVisible();
    await expect(page.getByText('Hi ACME, I have reviewed your request for a logo refresh and 3-page site...')).toBeVisible();

    // Verify primary action button
    const approveBtn = page.getByTestId('approve-send-proposal');
    await expect(approveBtn).toBeVisible();

    // Approve the proposal
    await approveBtn.click();

    // Verify the card disappears (after clicking, we expect it to be gone from approvals feed)
    // Note: It might take a moment to refresh after click. Let's wait for it to be hidden.
    await expect(page.getByText('Draft proposal for ACME Corp Branding')).toBeHidden({ timeout: 10000 });
  });
});
