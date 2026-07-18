import { test, expect } from '../fixtures';

test.describe('Quote Edit E2E', () => {
  test('verify editing a quote works naturally', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);

    // To follow the Real Owner/Operator E2E Standard, the UI must navigate as a user
    // Start from Dashboard after login
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');

    // Make sure we have a mock quote created so we can interact with it in the dashboard.
    // Navigate to create a quote or rely on deterministic fixtures.
    const createQuoteRes = await page.request.post('/api/v1/quotes', {
      headers: {
        'x-tenant-id': 'tenant-1'
      },
      data: {
        tenant_id: 'tenant-1',
        customer_id: '00000000-0000-0000-0000-000000000000',
        total_amount: 50000,
        required_deposit: 10000,
        line_items: [
          {
            description: "Plumbing work",
            unit_price_cents: 50000,
            quantity: 1,
            is_optional: false
          }
        ]
      }
    });

    expect(createQuoteRes.ok()).toBeTruthy();

    // The mock data setup above should trigger a UI update on dashboard if needed,
    // but the actual action should start purely via navigation. Let's make sure dashboard is fresh.
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');

    // Switch to proposals tab naturally
    const proposalsTab = page.locator('button', { hasText: /Proposals/ }).first();
    await expect(proposalsTab).toBeVisible({ timeout: 15000 });
    await proposalsTab.click();

    // Verify draft quote is visible
    const editQuoteBtn = page.getByTestId('edit-quote-draft').first();
    await expect(editQuoteBtn).toBeVisible({ timeout: 15000 });

    // Click edit quote to navigate
    await editQuoteBtn.click();
    await page.waitForLoadState('networkidle');

    // Open edit sheet
    const editBtn = page.locator('#edit-quote-btn');
    await expect(editBtn).toBeVisible({ timeout: 10000 });
    await editBtn.click();

    // Save edits
    const saveBtn = page.locator('#btn-save-edits');
    await expect(saveBtn).toBeVisible({ timeout: 5000 });
    await saveBtn.click();

    // Make sure edit sheet closes
    await expect(saveBtn).not.toBeVisible({ timeout: 5000 });
  });
});
