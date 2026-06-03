import { test, expect } from './fixtures';

test.describe('My Plan Page E2E', () => {

  test('Persona: Business Owner can view their plan details and limits', async ({ page }) => {
    // 1. Owner Logs In
    await page.goto('/login');
    await page.getByPlaceholder(/Email/i).fill('maya@example.com');
    await page.getByPlaceholder(/Password/i).fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();

    // 2. Navigate to My Plan
    await page.goto('/plan');

    // 3. Verify Page Title
    await expect(page.getByRole('heading', { name: /^My Plan$/i })).toBeVisible();

    // 4. Verify Plan Snapshot
    await expect(page.getByText('Plan:')).toBeVisible();
    await expect(page.getByText('Estimated Next Bill:')).toBeVisible();

    // 5. Verify Current Usage is displayed
    await expect(page.getByRole('heading', { name: /Your Current Usage/i })).toBeVisible();
    await expect(page.getByText('AI Actions Used')).toBeVisible();
    await expect(page.getByText('Storage Used')).toBeVisible();

    // 6. Verify Quick Actions
    const costDetailsBtn = page.getByRole('button', { name: /View Cost Details/i });
    await expect(costDetailsBtn).toBeVisible();

    const changePlanBtn = page.getByRole('button', { name: /Change Plan/i });
    await expect(changePlanBtn).toBeVisible();

    // 7. Verify navigation to pricing via 'Change Plan' button
    await changePlanBtn.click();
    await expect(page).toHaveURL(/\/pricing/);

    // Navigate back to plan
    await page.goto('/plan');

    // 8. Verify navigation to cost dashboard via 'View Cost Details' button
    const viewCostsBtn = page.getByRole('button', { name: /View Cost Details/i });
    await expect(viewCostsBtn).toBeVisible();
    await viewCostsBtn.click();
    await expect(page).toHaveURL(/\/cost-dashboard/);
  });
});
