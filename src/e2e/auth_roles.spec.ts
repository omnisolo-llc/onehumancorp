import { test, expect } from './fixtures';

test.describe('Database-seeded authentication', () => {
  test('admin user logs in through the real UI', async ({ page }) => {
<<<<<<< HEAD
=======
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
    await expect(page.getByText('Welcome back')).toBeVisible();
  });

  test('regular team member logs in through the real UI', async ({ memberPage }) => {
<<<<<<< HEAD
=======
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
    await expect(memberPage.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
    await expect(memberPage.getByText('Welcome back')).toBeVisible();
  });
});
