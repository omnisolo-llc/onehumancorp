import { test, expect } from '@playwright/test';

test.describe('Automated Invoicing & Cash Flow Feed interaction', () => {
  test.use({ storageState: 'e2e/.auth/admin.json' });
  test.setTimeout(180000);

  test('Project milestone completion triggers draft invoice card and approval', async ({ page, request }) => {
    // 1. Fire a test event simulating project milestone completed
    const webhookRes = await request.post('/api/agents/webhook', {
      data: {
        id: `evt-${Date.now()}`,
        type: 'project_milestone_completed',
        data: {
          project_id: `proj-${Date.now()}`,
          project_title: 'Website Redesign',
          customer_id: `cust-${Date.now()}`,
          customer_name: 'Test Customer',
          amount: 2500.0,
        },
      },
    });

    // 2. Navigate to Dashboard (Unified Feed)
    await page.goto('/dashboard');

    // 3. Verify the ReviewDraftInvoiceCard appears
    const draftInvoiceCard = page.locator('[data-testid="draft-invoice-card"]');
    await expect(draftInvoiceCard).toBeVisible({ timeout: 60000 });

    // 4. Verify text content
    await expect(page.locator('text=Draft Invoice Ready')).toBeVisible();
    await expect(page.locator('text=Website Redesign')).toBeVisible();
    await expect(page.locator('text=Test Customer')).toBeVisible();
    await expect(page.locator('text=$2500.00')).toBeVisible();

    // 5. Click "Approve & Send"
    await page.getByRole('button', { name: 'Approve & Send' }).click();

    // 6. Verify card dismisses
    await expect(draftInvoiceCard).not.toBeVisible({ timeout: 20000 });
  });
});
