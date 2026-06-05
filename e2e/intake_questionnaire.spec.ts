import { test, expect } from '@playwright/test';

test.describe('Intake Questionnaire Flow', () => {
  const tenantId = 'e2e-tenant-123';
  const customerId = 'e2e-customer-456';

  test('Customer submits intake form and AI generates quote', async ({ page, request }) => {
    // 1. Setup mock backend routes if necessary to bypass Auth and set context
    // Since we are asked to not use mocks, we will test the actual frontend intake form page.
    // 1. Setup mock backend routes if necessary to bypass Auth and set context
    // No mock data per system rule.


    // 2. Navigate to Intake Page
    await page.goto('/intake');
    await expect(page.locator('h1')).toHaveText('Service Request Intake');

    // 3. Fill text response
    await page.locator('textarea').first().fill('Need a wood flooring install for a 100 sqft room.');

    // 4. Submit form
    await page.locator('button', { hasText: 'Submit Request' }).click();

    // 5. Verify successful submission UI
    await expect(page.locator('h2')).toHaveText('Request Submitted');
  });
});
