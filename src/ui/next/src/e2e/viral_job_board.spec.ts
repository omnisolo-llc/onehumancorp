import { test, expect } from '../../../../e2e/fixtures';

test.describe('Viral Job Board Generator', () => {
  test('should render the generator and preview correctly', async ({ page }) => {
    await page.goto('/viral-job-board-generator');

    // Check title
    await expect(page.getByRole('heading', { name: 'Viral Job Board Generator', exact: true }).first()).toBeVisible();

    // Check default input values
    const titleInput = page.locator('input[placeholder="e.g. We are hiring!"]');
    await expect(titleInput).toHaveValue('We are hiring!');

    // Modify inputs and check preview updates
    await titleInput.fill('Join Our Startup');
    await expect(page.getByRole('heading', { name: 'Join Our Startup', exact: true })).toBeVisible();

    // Check referral block in preview
    await expect(page.getByText('Refer a friend and get $500 if they are hired!', { exact: true })).toBeVisible();
  });
});

test.describe('Viral Job Board Generator tests', () => {
  test('should clear the input fields properly', async ({ page }) => {
    await page.goto('/viral-job-board-generator');

    const titleInput = page.locator('input[placeholder="e.g. We are hiring!"]');
    await titleInput.fill('');
    await expect(page.getByRole('heading', { name: 'We are hiring!', exact: true })).toBeVisible();
  });

  test('should handle description changes', async ({ page }) => {
    await page.goto('/viral-job-board-generator');

    const descInput = page.locator('textarea[placeholder="e.g. Join our team and help us build the future."]');
    await descInput.fill('Join our amazing company');
    await expect(page.getByRole('paragraph').filter({ hasText: /^Join our amazing company$/ })).toBeVisible();
  });

  test('should handle empty description', async ({ page }) => {
    await page.goto('/viral-job-board-generator');

    const descInput = page.locator('textarea[placeholder="e.g. Join our team and help us build the future."]');
    await descInput.fill('');
    await expect(page.getByText('Join our team.', { exact: true })).toBeVisible();
  });

  test('should toggle theme correctly', async ({ page }) => {
    await page.goto('/viral-job-board-generator');

    await page.locator('button:has-text("Dark")').click();
    const preview = page.getByRole('heading', { name: 'We are hiring!', exact: true }).locator('..');
    await expect(preview).toHaveCSS('background-color', 'rgb(17, 24, 39)');

    await page.locator('button:has-text("Light")').click();
    await expect(preview).toHaveCSS('background-color', 'rgb(255, 255, 255)');
  });

  test('should verify empty fields default to expected text in preview', async ({ page }) => {
    await page.goto('/viral-job-board-generator');

    const titleInput = page.locator('input[placeholder="e.g. We are hiring!"]');
    await titleInput.fill('');
    await expect(page.getByRole('heading', { name: 'We are hiring!', exact: true })).toBeVisible();

    const descInput = page.locator('textarea[placeholder="e.g. Join our team and help us build the future."]');
    await descInput.fill('');
    await expect(page.getByText('Join our team.', { exact: true })).toBeVisible();
  });

});
