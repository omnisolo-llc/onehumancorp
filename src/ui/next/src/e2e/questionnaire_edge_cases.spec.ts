import { test, expect } from '@playwright/test';

test.describe('Autonomous Client Intake Questionnaire Edge Cases', () => {

  test('handles an empty questionnaire template gracefully', async ({ page }) => {
     await page.route('/api/questionnaires/empty-template-id', async route => {
        await route.fulfill({ json: { id: 'empty-template-id', questions: [] } });
     });

     await page.goto('/intake/empty-template-id');
     await expect(page.locator('text=Form not found.')).toBeVisible();
  });

  test('validates required fields before allowing submission', async ({ page }) => {
     await page.route('/api/questionnaires/required-template', async route => {
        await route.fulfill({
           json: {
              id: 'required-template',
              questions: [
                 { id: 'q1', text: 'Required text', type: 'text', is_required: true },
                 { id: 'q2', text: 'Optional text', type: 'text', is_required: false }
              ]
           }
        });
     });

     await page.goto('/intake/required-template');

     // The OK button should be disabled because the first question is required and empty
     const btn = page.locator('button:has-text("OK")');
     await expect(btn).toBeDisabled();

     // Fill the input
     await page.fill('input[type="text"]', 'Answer');
     await expect(btn).toBeEnabled();
     await btn.click();

     // Step 2 is optional, so Submit is enabled even if empty
     await expect(page.locator('h1')).toHaveText('Optional text');
     const submitBtn = page.locator('button:has-text("Submit Request")');
     await expect(submitBtn).toBeEnabled();
  });
});
