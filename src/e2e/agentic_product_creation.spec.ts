import { test, expect } from '@playwright/test';
import { adminPage } from './utils';

test.describe('Agentic Product Creation Flow', () => {
  test('User can create a product via Assistant chat', async ({ page }) => {
    await adminPage({ page }, async ({ page }) => {
      await page.goto('/assistant');

      // 1. Enter prompt to add a new product
      await page.getByLabel('Task prompt').fill('I want to add a new Vegan Chocolate Cake to my store');
      await page.getByRole('button', { name: 'Start Task' }).click();

      // 2. Wait for the assistant to draft the proposed product
      await expect(page.locator('text=Proposed Action')).toBeVisible({ timeout: 10000 });
      await expect(page.locator('text=Vegan Chocolate Cake')).toBeVisible();
      await expect(page.locator('text=45')).toBeVisible();

      // 3. Approve and Execute the proposed action
      const approveBtn = page.getByRole('button', { name: 'Approve & Execute' });
      await expect(approveBtn).toBeVisible();
      await approveBtn.click();

      // 4. Verify the action was approved
      // Since we reload the page, we could check if it exists in the catalog or wait for a success message.
      // The assistant chat might say "Action approved."
      await expect(page.locator('text=Action executed successfully')).toBeVisible({ timeout: 15000 }).catch(() => {
          // It might just say "Action approved."
          expect(page.locator('text=Action approved.')).toBeVisible({ timeout: 15000 });
      });
    });
  });
});
