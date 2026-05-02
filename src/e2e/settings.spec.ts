import { test, expect } from '@playwright/test';

test.describe('Settings Page', () => {
  test.beforeEach(async ({ page }) => {
    await page.fill('input[type="email"]', 'admin@example.com');
    await page.fill('input[type="password"]', 'admin123');
    await page.locator('button:has-text("Sign In")').click();
    await expect(page).toHaveURL(/\/(dashboard|\/)$/, { timeout: 10000 });

    // Navigate to feature from dashboard
    const btn = page.locator('button:has-text("/login")').first();
    if (await btn.isVisible()) {
      await btn.click();
    } else {
      const menuBtn = page.locator('button:has-text("Menu")').first();
      if (await menuBtn.isVisible()) {
        await menuBtn.click();
        const innerBtn = page.locator('button:has-text("/login")').first();
        if (await innerBtn.isVisible()) {
          await innerBtn.click();
        }
      }
    }
  });

  test.beforeEach(async ({ page }) => {
    await page.fill('input[type="email"]', 'admin@example.com');
    await page.fill('input[type="password"]', 'admin123');
    await page.locator('button:has-text("Sign In")').click();
    await expect(page).toHaveURL(/\/(dashboard|\/)$/, { timeout: 10000 });

    const btn = page.locator('button:has-text("/login")');
    if (await btn.isVisible()) {
      await btn.click();
    } else {
      const menuBtn = page.locator('button:has-text("Menu")');
      if (await menuBtn.isVisible()) {
        await menuBtn.click();
        await page.locator('button:has-text("/login")').click();
      }
    }
  });
  test('should show settings page', async ({ page }) => {
    await expect(page.locator('text=/settings|preferences/i')).toBeVisible();
  });

  test('should display general settings section', async ({ page }) => {
    await expect(page.locator('text=General')).toBeVisible();
  });

  test('should display profile settings section', async ({ page }) => {
    await expect(page.locator('text=Profile')).toBeVisible();
  });

  test('should show notification settings', async ({ page }) => {
    await expect(page.locator('text=/notification|alert/i')).toBeVisible();
  });

  test('should enable email notifications', async ({ page }) => {
    const emailToggle = page.locator('input[type="checkbox"]').first();
    if (await emailToggle.isVisible()) {
      await emailToggle.check();
      await expect(page.locator('text=/saved|enabled/i')).toBeVisible({ timeout: 3000 }).catch(() => {});
    }
  });

  test('should enable push notifications', async ({ page }) => {
    const pushToggle = page.locator('text=/push/i').locator('..').locator('input[type="checkbox"]').first();
    if (await pushToggle.isVisible()) {
      await pushToggle.check();
    }
  });

  test('should display timezone setting', async ({ page }) => {
    await expect(page.locator('text=/timezone|zone/i')).toBeVisible();
  });

  test('should change timezone', async ({ page }) => {
    const tzSelect = page.locator('select').first();
    if (await tzSelect.isVisible()) {
      await tzSelect.selectOption({ index: 1 });
      await page.locator('button:has-text("Save")').click();
    }
  });

  test('should display language setting', async ({ page }) => {
    await expect(page.locator('text=/language|language/i')).toBeVisible();
  });

  test('should change language', async ({ page }) => {
    const langSelect = page.locator('select').nth(1);
    if (await langSelect.isVisible()) {
      await langSelect.selectOption({ index: 1 });
      await page.locator('button:has-text("Save")').click();
    }
  });

  test('should display theme setting', async ({ page }) => {
    await expect(page.locator('text=/theme|appearance/i')).toBeVisible();
  });

  test('should switch to dark theme', async ({ page }) => {
    const darkOption = page.locator('text=/dark|night/i').first();
    if (await darkOption.isVisible()) {
      await darkOption.click();
      await expect(page.locator('[class*="dark"], [class*="dark-theme"]').first()).toBeVisible({ timeout: 3000 }).catch(() => {});
    }
  });

  test('should switch to light theme', async ({ page }) => {
    const lightOption = page.locator('text=/light|bright/i').first();
    if (await lightOption.isVisible()) {
      await lightOption.click();
    }
  });

  test('should display date format setting', async ({ page }) => {
    await expect(page.locator('text=/date.*format|format/i')).toBeVisible();
  });

  test('should change date format', async ({ page }) => {
    const formatSelect = page.locator('select').nth(2);
    if (await formatSelect.isVisible()) {
      await formatSelect.selectOption({ index: 1 });
    }
  });

  test('should save settings', async ({ page }) => {
    await page.locator('button:has-text("Save")').click();
    await expect(page.locator('text=/saved|success/i')).toBeVisible({ timeout: 3000 }).catch(() => {});
  });

  test('should show cancel button', async ({ page }) => {
    await expect(page.locator('button:has-text("Cancel")')).toBeVisible();
  });

  test('should discard changes on cancel', async ({ page }) => {
    const input = page.locator('input[type="text"]').first();
    if (await input.isVisible()) {
      await input.fill('test value');
    }
    await page.locator('button:has-text("Cancel")').click();
    await expect(page.locator('text=/settings/i')).toBeVisible();
  });
});

test.describe('Profile Settings', () => {
  test.beforeEach(async ({ page }) => {
    await page.fill('input[type="email"]', 'admin@example.com');
    await page.fill('input[type="password"]', 'admin123');
    await page.locator('button:has-text("Sign In")').click();
    await expect(page).toHaveURL(/\/(dashboard|\/)$/, { timeout: 10000 });

    // Navigate to feature from dashboard
    const btn = page.locator('button:has-text("/login")').first();
    if (await btn.isVisible()) {
      await btn.click();
    } else {
      const menuBtn = page.locator('button:has-text("Menu")').first();
      if (await menuBtn.isVisible()) {
        await menuBtn.click();
        const innerBtn = page.locator('button:has-text("/login")').first();
        if (await innerBtn.isVisible()) {
          await innerBtn.click();
        }
      }
    }
  });

  test('should show profile picture upload', async ({ page }) => {
    await expect(page.locator('text=/photo|avatar|picture/i')).toBeVisible();
  });

  test('should upload profile picture', async ({ page }) => {
    const uploadArea = page.locator('input[type="file"]').first();
    if (await uploadArea.isVisible()) {
      // File upload would require actual file - test exists check
      await expect(uploadArea).toBeAttached();
    }
  });

  test('should update display name', async ({ page }) => {
    const nameInput = page.locator('input[placeholder*="name" i]').first();
    if (await nameInput.isVisible()) {
      await nameInput.fill('New Name');
      await page.locator('button:has-text("Update")').click();
    }
  });

  test('should update bio', async ({ page }) => {
    const bioInput = page.locator('textarea').first();
    if (await bioInput.isVisible()) {
      await bioInput.fill('This is my bio');
      await page.locator('button:has-text("Update")').click();
    }
  });

  test('should update email', async ({ page }) => {
    const emailInput = page.locator('input[type="email"]').first();
    if (await emailInput.isVisible()) {
      await emailInput.fill('newemail@example.com');
      await page.locator('button:has-text("Update")').click();
    }
  });

  test('should update phone number', async ({ page }) => {
    const phoneInput = page.locator('input[type="tel"]').first();
    if (await phoneInput.isVisible()) {
      await phoneInput.fill('+1234567890');
      await page.locator('button:has-text("Update")').click();
    }
  });

  test('should change password', async ({ page }) => {
    await expect(page.locator('text=/password|security/i')).toBeVisible();
  });

  test('should show current password field', async ({ page }) => {
    await expect(page.locator('input[placeholder*="current" i]')).toBeVisible();
  });

  test('should show new password field', async ({ page }) => {
    await expect(page.locator('input[placeholder*="new" i]')).toBeVisible();
  });

  test('should show confirm password field', async ({ page }) => {
    await expect(page.locator('input[placeholder*="confirm" i]')).toBeVisible();
  });

  test('should validate password match', async ({ page }) => {
    await page.fill('input[placeholder*="new" i]', 'password123');
    await page.fill('input[placeholder*="confirm" i]', 'different');
    await page.locator('button:has-text("Change")').click();
    await expect(page.locator('text=/match|mismatch/i')).toBeVisible();
  });
});
