import { test, expect } from './fixtures';
import { runtimeAcceptance, runtimeGateReason } from './generation-acceptance';
import { expectRuntimeUnavailable, runtimeUnavailableMessage } from './support/runtime_unavailable';

test.describe('Actor Model UI', () => {

  test('Actor Model UI works end to end via UI @runtime-acceptance', async ({ page, unlimitedAdminUser, loginAs }) => {
    test.skip(!runtimeAcceptance, runtimeGateReason);
    // Login first to satisfy real E2E standard
    await loginAs(page, unlimitedAdminUser);

    await page.goto('/actor-model');

    await expect(page.locator('h1')).toContainText('Actor-Model Message Passing');

    await page.fill('textarea[id="message"]', 'Test Actor Model Task');

    await page.click('text=Send Message to Swarm');

    await expect(page.getByTestId('success-message')).toBeVisible({ timeout: 60000 });
  });

  test('unconfigured swarm preserves the message without claiming execution', async ({ page, unlimitedAdminUser, loginAs }) => {
    test.skip(runtimeAcceptance, 'This contract requires an unconfigured runtime.');
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/actor-model');
    const message = 'Research this supplied business brief';
    await page.locator('#message').fill(message);
    const execution = page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/agents/actor-model' && response.request().method() === 'POST');
    await page.getByRole('button', { name: 'Send Message to Swarm' }).click();
    await expectRuntimeUnavailable(await execution);
    await expect(page.getByTestId('error-message')).toContainText(runtimeUnavailableMessage);
    await expect(page.getByTestId('success-message')).toHaveCount(0);
    await expect(page.locator('#message')).toHaveValue(message);
    await expect(page.getByRole('button', { name: 'Send Message to Swarm' })).toBeEnabled();
  });

  test('Verify the form disables the Send Message button when the message is empty', async ({ page, unlimitedAdminUser, loginAs }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/actor-model');

    await expect(page.locator('h1')).toContainText('Actor-Model Message Passing');
    await page.fill('textarea[id="message"]', '');

    const button = page.locator('button', { hasText: 'Send Message to Swarm' });
    await expect(button).toBeDisabled();
  });

  test('Verify the Actors are executing... loading state text when waiting for the Swarm to reply @runtime-acceptance', async ({ page, unlimitedAdminUser, loginAs }) => {
    test.skip(!runtimeAcceptance, runtimeGateReason);
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

  test('Verify that submitting a message successfully displays the Swarm Result section @runtime-acceptance', async ({ page, unlimitedAdminUser, loginAs }) => {
    test.skip(!runtimeAcceptance, runtimeGateReason);
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
