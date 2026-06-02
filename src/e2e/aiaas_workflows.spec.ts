import { test, expect } from '@playwright/test';

// Use the existing seeded data in the database rather than mocking the API.
// e2e-seed.sql inserts an agent_approval with:
// id: 'e2e-approval-1'
// tenant_id: 'e2e-tenant'
// department: 'customer_success'
// payload: { "feature_type": "ambassador_reply", "original_message": "Do you have vegan options for birthday cakes?", "generated_response": "Yes, we have several vegan options for birthday cakes. We would love to help you plan your special day!" }

test('AIaaS Core Capabilities - Review Draft and Regenerate flow', async ({ page }) => {
  // 1. Navigate to Team page directly (login should be handled by global setup or use auth bypass if supported)
  // Assuming the test harness creates a session for `e2e-user` -> `e2e-tenant` automatically based on test conventions
  await page.goto('/login');

  // Quick login if required by the test framework
  try {
    if (await page.getByPlaceholder('Email').isVisible({ timeout: 2000 })) {
       await page.getByPlaceholder('Email').fill('user@e2e.test');
       await page.getByPlaceholder('Password').fill('password123');
       await page.getByRole('button', { name: 'Sign in' }).click();
       await page.waitForURL('/dashboard');
    }
  } catch (e) {}

  await page.goto('/team');

  // 2. Find the Customer Success department card and click it
  await expect(page.getByText('The Ambassador')).toBeVisible();
  await page.getByText('The Ambassador').click();

  // 3. Verify the inbox shows the seeded approval
  await expect(page.getByText('Yes, we have several vegan options')).toBeVisible();

  // 4. Click Review
  await page.getByRole('button', { name: 'Review' }).first().click();

  // 5. Verify the Review Draft modal is visible with correct data
  await expect(page.getByText('Review Draft')).toBeVisible();
  await expect(page.getByText('Do you have vegan options for birthday cakes?').first()).toBeVisible();
  await expect(page.getByText('Yes, we have several vegan options for birthday cakes.').first()).toBeVisible();

  // 6. Click Regenerate (this will trigger onReject internally and close the modal)
  await page.getByRole('button', { name: 'Regenerate' }).click();

  // 7. Modal should close and the item should disappear
  await expect(page.getByText('Review Draft')).not.toBeVisible();
});
