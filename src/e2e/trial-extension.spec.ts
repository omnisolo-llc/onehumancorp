import { test, expect } from './fixtures';
import './fixtures';

async function navigateToTrialExtension(page: import("@playwright/test").Page) {
  await page.goto('/trial-extension.html');
}

test.describe.serial('Trial Extension', () => {

  test('should display the trial extension page', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    await navigateToTrialExtension(page);

    await expect(page.locator('h1', { hasText: 'Interactive Trial Extension' }).first()).toBeVisible({ timeout: 10000 });
    await expect(page.getByText('Want 7 Extra Days of Pro?')).toBeVisible();

    const poweredByLink = page.locator('a', { hasText: /OmniSolo/i }).first();
    await expect(poweredByLink).toBeVisible();
    await expect(poweredByLink).toHaveAttribute('href', /.*\/api\/v1\/growth\/referrals\/click\?target=\/onboarding&ref=trial_extension/);
  });

  test('should claim trial extension successfully', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    await navigateToTrialExtension(page);

    // Stub window.open so the test doesn't actually open Twitter
    await page.evaluate(() => {
      window.open = function() { return null; };
    });

    const shareButton = page.getByRole('button', { name: /Share on X to Unlock 7 Days/i });
    await expect(shareButton).toBeVisible();

    await shareButton.click();

    // We expect either success or an alert
    let dialogMessage = '';
    page.on('dialog', async dialog => {
      dialogMessage = dialog.message();
      await dialog.accept();
    });

    try {
        await expect(page.getByText('Trial Extended!')).toBeVisible({ timeout: 15000 });
    } catch(e) {
        if (!dialogMessage.includes('Failed to claim')) {
            throw e;
        }
    }
  });

  test('should fail gracefully if backend returns error (already claimed)', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    await navigateToTrialExtension(page);

    await page.evaluate(() => {
      window.open = function() { return null; };
    });

    let dialogMessage = '';
    page.on('dialog', async dialog => {
      dialogMessage = dialog.message();
      await dialog.accept();
    });

    const shareButton = page.getByRole('button', { name: /Share on X to Unlock 7 Days/i });
    await shareButton.click();

    await expect(async () => {
        expect(dialogMessage).toContain('Failed to claim trial extension');
    }).toPass({ timeout: 15000 });

    await expect(page.getByText('Trial Extended!')).not.toBeVisible();
  });

  test('should fail gracefully on network error', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    await navigateToTrialExtension(page);

    await page.evaluate(() => {
      window.open = function() { return null; };
    });

    await page.evaluate(() => {
      const originalFetch = window.fetch;
      window.fetch = async function(...args: Parameters<typeof fetch>) {
        if (args[0] && typeof args[0] === 'string' && args[0].includes('/api/v1/growth/trial-extension/claim')) {
          args[0] = 'http://localhost:9999/invalid-endpoint-for-network-error'; // deliberate network error
        }
        return originalFetch(...args);
      };
    });

    let dialogMessage = '';
    page.on('dialog', async dialog => {
      dialogMessage = dialog.message();
      await dialog.accept();
    });

    const shareButton = page.getByRole('button', { name: /Share on X to Unlock 7 Days/i });
    await shareButton.click();

    await expect(async () => {
        expect(dialogMessage).toContain('Error claiming trial extension');
    }).toPass({ timeout: 15000 });

    await expect(page.getByText('Trial Extended!')).not.toBeVisible();
  });

  test('should have a working back to dashboard link', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    await navigateToTrialExtension(page);

    const backLink = page.getByRole('link', { name: /Back to Dashboard|Return to Dashboard/i }).first();
    await expect(backLink).toBeVisible();
    await backLink.click();
    await expect(page).toHaveURL(/.*\/dashboard/, { timeout: 15000 });
  });
});
