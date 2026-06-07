import { test, expect } from '@playwright/test';
import { adminPage } from '../../../e2e/fixtures';

/**
 * Persona: Fatima, Food Cart Operator
 * Concept: Agentic Food Pre-Order & Real-Time Pickup Workflow
 * CUJ:
 *   1. Customer places order via API directly (simulating storefront).
 *   2. Fatima opens the KDS page.
 *   3. Sees the new order with English notes translated to Arabic.
 *   4. Fatima changes status to Preparing, then Ready.
 *   5. Verifies UI updates optimistically.
 */

test.describe('Agentic Food Pre-Order & Pickup Workflow', () => {
  test('Fatima receives pre-order with translated notes and updates status', async ({ page, request }) => {
    // 1. Customer places an order with "no onions" via API
    const res = await request.post('/api/v1/food-pre-order/create', {
      headers: {
        'Authorization': 'Bearer test-admin-token' // Use local dev auth if needed, but the endpoint handles auth_info
      },
      data: {
        customer_name: 'Ahmed Customer',
        items: ['1x Lamb Combo'],
        notes: 'no onions',
        total_amount: 15.00
      }
    });

    // Wait a brief moment for the OperationsAgent to pick up the event and translate the note
    await page.waitForTimeout(1000);

    // 2. Fatima opens the KDS
    await page.goto('/pos/kds');

    // 3. Sees the order with translated notes
    await expect(page.locator('text=Ahmed Customer')).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text=no onions')).toBeVisible();
    await expect(page.locator('text=بدون بصل')).toBeVisible(); // Translated note from the OperationsAgent simulation

    // 4. Update status to Preparing
    const prepareBtn = page.getByText('Preparing', { exact: true }).first();
    await expect(prepareBtn).toBeVisible();
    await prepareBtn.click();

    // 5. Update status to Ready
    const readyBtn = page.getByText('Ready', { exact: true }).first();
    await expect(readyBtn).toBeVisible();
    await readyBtn.click();

    // 6. Verify optimistic state holds "Ready" button disabled
    await expect(readyBtn).toBeDisabled();
  });
});
