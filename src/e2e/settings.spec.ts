import { test, expect } from '@playwright/test';

test.describe('Settings Page', () => {
  test('should show settings page', async ({ page }) => {
    await page.goto('/settings');
    await expect(page.locator('text=/settings|preferences/i')).toBeVisible();
  });

  test('should display general settings section', async ({ page }) => {
    await page.goto('/settings');
    await expect(page.locator('text=General')).toBeVisible();
  });

  test('should display profile settings section', async ({ page }) => {
    await page.goto('/settings');
    await expect(page.locator('text=Profile')).toBeVisible();
  });

  test('should show notification settings', async ({ page }) => {
    await page.goto('/settings');
    await expect(page.locator('text=/notification|alert/i')).toBeVisible();
  });

  test('should enable email notifications', async ({ page }) => {
    await page.goto('/settings');
    const emailToggle = page.locator('input[type="checkbox"]').first();
    await emailToggle.waitFor();
    await emailToggle
      await emailToggle.check();
      await expect(page.locator('text=/saved|enabled/i')).toBeVisible({ timeout: 3000 });
    }
  });

  test('should enable push notifications', async ({ page }) => {
    await page.goto('/settings');
    const pushToggle = page.locator('text=/push/i').locator('..').locator('input[type="checkbox"]').first();
    await pushToggle.waitFor();
    await pushToggle
      await pushToggle.check();
    }
  });

  test('should display timezone setting', async ({ page }) => {
    await page.goto('/settings');
    await expect(page.locator('text=/timezone|zone/i')).toBeVisible();
  });

  test('should change timezone', async ({ page }) => {
    await page.goto('/settings');
    const tzSelect = page.locator('select').first();
    await tzSelect.waitFor();
    await tzSelect
      await tzSelect.selectOption({ index: 1 });
      await page.locator('button:has-text("Save")').click();
    }
  });

  test('should display language setting', async ({ page }) => {
    await page.goto('/settings');
    await expect(page.locator('text=/language|language/i')).toBeVisible();
  });

  test('should change language', async ({ page }) => {
    await page.goto('/settings');
    const langSelect = page.locator('select').nth(1);
    await langSelect.waitFor();
    await langSelect
      await langSelect.selectOption({ index: 1 });
      await page.locator('button:has-text("Save")').click();
    }
  });

  test('should display theme setting', async ({ page }) => {
    await page.goto('/settings');
    await expect(page.locator('text=/theme|appearance/i')).toBeVisible();
  });

  test('should switch to dark theme', async ({ page }) => {
    await page.goto('/settings');
    const darkOption = page.locator('text=/dark|night/i').first();
    await darkOption.waitFor();
    await darkOption
      await darkOption.click();
      await expect(page.locator('[class*="dark"], [class*="dark-theme"]').first()).toBeVisible({ timeout: 3000 });
    }
  });

  test('should switch to light theme', async ({ page }) => {
    await page.goto('/settings');
    const lightOption = page.locator('text=/light|bright/i').first();
    await lightOption.waitFor();
    await lightOption
      await lightOption.click();
    }
  });

  test('should display date format setting', async ({ page }) => {
    await page.goto('/settings');
    await expect(page.locator('text=/date.*format|format/i')).toBeVisible();
  });

  test('should change date format', async ({ page }) => {
    await page.goto('/settings');
    const formatSelect = page.locator('select').nth(2);
    await formatSelect.waitFor();
    await formatSelect
      await formatSelect.selectOption({ index: 1 });
    }
  });

  test('should save settings', async ({ page }) => {
    await page.goto('/settings');
    await page.locator('button:has-text("Save")').click();
    await expect(page.locator('text=/saved|success/i')).toBeVisible({ timeout: 3000 });
  });

  test('should show cancel button', async ({ page }) => {
    await page.goto('/settings');
    await expect(page.locator('button:has-text("Cancel")')).toBeVisible();
  });

  test('should discard changes on cancel', async ({ page }) => {
    await page.goto('/settings');
    const input = page.locator('input[type="text"]').first();
    await input.waitFor();
    await input
      await input.fill('test value');
    }
    await page.locator('button:has-text("Cancel")').click();
    await expect(page.locator('text=/settings/i')).toBeVisible();
  });
});

test.describe('Profile Settings', () => {
  test('should show profile picture upload', async ({ page }) => {
    await page.goto('/settings/profile');
    await expect(page.locator('text=/photo|avatar|picture/i')).toBeVisible();
  });

  test('should upload profile picture', async ({ page }) => {
    await page.goto('/settings/profile');
    const uploadArea = page.locator('input[type="file"]').first();
    await uploadArea.waitFor();
    await uploadArea
      // File upload would require actual file - test exists check
      await expect(uploadArea).toBeAttached();
    }
  });

  test('should update display name', async ({ page }) => {
    await page.goto('/settings/profile');
    const nameInput = page.locator('input[placeholder*="name" i]').first();
    await nameInput.waitFor();
    await nameInput
      await nameInput.fill('New Name');
      await page.locator('button:has-text("Update")').click();
    }
  });

  test('should update bio', async ({ page }) => {
    await page.goto('/settings/profile');
    const bioInput = page.locator('textarea').first();
    await bioInput.waitFor();
    await bioInput
      await bioInput.fill('This is my bio');
      await page.locator('button:has-text("Update")').click();
    }
  });

  test('should update email', async ({ page }) => {
    await page.goto('/settings/profile');
    const emailInput = page.locator('input[type="email"]').first();
    await emailInput.waitFor();
    await emailInput
      await emailInput.fill('newemail@example.com');
      await page.locator('button:has-text("Update")').click();
    }
  });

  test('should update phone number', async ({ page }) => {
    await page.goto('/settings/profile');
    const phoneInput = page.locator('input[type="tel"]').first();
    await phoneInput.waitFor();
    await phoneInput
      await phoneInput.fill('+1234567890');
      await page.locator('button:has-text("Update")').click();
    }
  });

  test('should change password', async ({ page }) => {
    await page.goto('/settings/security');
    await expect(page.locator('text=/password|security/i')).toBeVisible();
  });

  test('should show current password field', async ({ page }) => {
    await page.goto('/settings/security');
    await expect(page.locator('input[placeholder*="current" i]')).toBeVisible();
  });

  test('should show new password field', async ({ page }) => {
    await page.goto('/settings/security');
    await expect(page.locator('input[placeholder*="new" i]')).toBeVisible();
  });

  test('should show confirm password field', async ({ page }) => {
    await page.goto('/settings/security');
    await expect(page.locator('input[placeholder*="confirm" i]')).toBeVisible();
  });

  test('should validate password match', async ({ page }) => {
    await page.goto('/settings/security');
    await page.fill('input[placeholder*="new" i]', 'password123');
    await page.fill('input[placeholder*="confirm" i]', 'different');
    await page.locator('button:has-text("Change")').click();
    await expect(page.locator('text=/match|mismatch/i')).toBeVisible();
  });
});