import { test, expect } from './fixtures';

test('simple test', async ({ page }) => {
<<<<<<< HEAD
=======
  test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
  await page.goto('/login');
  await expect(page.getByRole('heading', { name: 'Login' }).first()).toBeVisible();
});
