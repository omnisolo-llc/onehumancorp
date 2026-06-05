import { currentAppSmoke } from './current_app_smoke';
import { test, expect } from './fixtures';

currentAppSmoke('onboarding');

test('onboarding state resume works', async ({ page }) => {
  // Mock localStorage for the wizard state to persist across page reloads
  await page.addInitScript(() => {
    window.localStorage.setItem('tenant_id', 'test-tenant-123');
    window.localStorage.setItem('user_id', 'test-user-123');
  });

  await page.goto('/onboarding');

  // Start the wizard
  const startButton = page.getByRole('button', { name: 'Start Onboarding' });
  await startButton.waitFor({ state: 'visible', timeout: 30000 });
  await startButton.click();

  // Fill in business name
  const nameInput = page.getByPlaceholder("e.g. Maya's Custom Cakes");
  await nameInput.waitFor({ state: 'visible' });
  await nameInput.fill('My Awesome Resume Test Business');
  await page.getByRole('button', { name: 'Next' }).click();

  // We are now on step 1, chatStep 2 (What do you sell?)
  const whatInput = page.getByPlaceholder("e.g. I bake custom vegan cakes for weddings and parties...");
  await whatInput.waitFor({ state: 'visible' });

  // Reload the page
  await page.reload();

  // Verify that the state was resumed and we are still on chatStep 2
  const whatInputAfterReload = page.getByPlaceholder("e.g. I bake custom vegan cakes for weddings and parties...");
  await expect(whatInputAfterReload).toBeVisible();

  // We should be able to click back and see the name is still there
  await page.getByRole('button', { name: 'Back' }).click();
  const nameInputAfterReload = page.getByPlaceholder("e.g. Maya's Custom Cakes");
  await expect(nameInputAfterReload).toHaveValue('My Awesome Resume Test Business');
});
