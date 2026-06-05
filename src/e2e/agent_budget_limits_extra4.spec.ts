import { expect, test } from './fixtures';

test.describe('Agent Budgets - Extra 4', () => {
  test('CUJ: Check extra 4', async ({ page }) => {
<<<<<<< HEAD
=======
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'AI Departments' }).first()).toBeVisible();
  });
});
