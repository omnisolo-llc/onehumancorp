import { test, expect } from './fixtures';

test.describe('Actor Model UI', () => {

  test('Actor Model UI works end to end via UI', async ({ page, unlimitedAdminUser, loginAs }) => {
    // Login first to satisfy real E2E standard
    await loginAs(page, unlimitedAdminUser);

    await page.goto('/actor-model');

    await expect(page.locator('h1')).toContainText('Actor-Model Message Passing');

    await page.fill('textarea[id="message"]', 'Test Actor Model Task');

    await page.click('text=Send Message to Swarm');

    await expect(page.getByTestId('success-message')).toBeVisible({ timeout: 60000 });
  });

  test('Verify the form disables the Send Message button when the message is empty', async ({ page, unlimitedAdminUser, loginAs }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/actor-model');

    await expect(page.locator('h1')).toContainText('Actor-Model Message Passing');
    await page.fill('textarea[id="message"]', '');

    const button = page.locator('button', { hasText: 'Send Message to Swarm' });
    await expect(button).toBeDisabled();
  });

  test('Verify the Actors are executing... loading state text when waiting for the Swarm to reply', async ({ page, unlimitedAdminUser, loginAs }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/actor-model');

    await page.fill('textarea[id="message"]', 'Test Actor Model Task');
    await page.click('text=Send Message to Swarm');

    const button = page.locator('button', { hasText: 'Actors are executing...' });
    await expect(button).toBeVisible();
    await expect(button).toBeDisabled();

    // Wait for it to finish
    await expect(page.getByTestId('success-message')).toBeVisible({ timeout: 60000 });
  });

  test('Verify that submitting a message successfully displays the Swarm Result section', async ({ page, unlimitedAdminUser, loginAs }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/actor-model');

    await page.fill('textarea[id="message"]', 'Test Actor Model Task');
    await page.click('text=Send Message to Swarm');

    await expect(page.getByTestId('success-message')).toBeVisible({ timeout: 60000 });
    await expect(page.getByTestId('success-message')).toContainText('Swarm Result');
  });

  test('Verify the textarea focuses correctly and updates value', async ({ page, unlimitedAdminUser, loginAs }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/actor-model');

    const textarea = page.locator('textarea[id="message"]');
    await textarea.click();
    await expect(textarea).toBeFocused();

    await textarea.fill('Focus test content');
    await expect(textarea).toHaveValue('Focus test content');
  });

});
