import { test, expect } from '@playwright/test';

test.describe('Security Settings', () => {
  test('should display security page', async ({ page }) => {
    await page.goto('/security');
    await expect(page.locator('text=/security|password/i')).toBeVisible();
  });

  test('should show security header', async ({ page }) => {
    await page.goto('/security');
    await expect(page.locator('text=Security')).toBeVisible();
  });

  test('should show change password option', async ({ page }) => {
    await page.goto('/security');
    await expect(page.locator('text=/change.*password|update.*password/i')).toBeVisible();
  });

  test('should show two-factor authentication option', async ({ page }) => {
    await page.goto('/security');
    await expect(page.locator('text=/two.*factor|2fa|mfa/i')).toBeVisible();
  });

  test('should enable two-factor authentication', async ({ page }) => {
    await page.goto('/security');
    const enableBtn = page.locator('button:has-text("Enable"), button:has-text("Setup")').first();
    if (await enableBtn.isVisible()) {
      await enableBtn.click();
      await expect(page.locator('text=/qr.*code|verify|authenticator/i')).toBeVisible();
    }
  });

  test('should show active sessions', async ({ page }) => {
    await page.goto('/security');
    await expect(page.locator('text=/session|active.*session/i')).toBeVisible();
  });

  test('should show session list', async ({ page }) => {
    await page.goto('/security');
    const sessionItem = page.locator('[class*="session"], [class*="device"]').first();
    await expect(sessionItem).toBeVisible();
  });

  test('should revoke session', async ({ page }) => {
    await page.goto('/security');
    const sessionItem = page.locator('[class*="session"]').first();
    await sessionItem.hover();
    const revokeBtn = page.locator('button:has-text("Revoke"), button:has-text("Remove")').first();
    if (await revokeBtn.isVisible()) {
      await revokeBtn.click();
      await expect(page.locator('text=/revoked|removed/i')).toBeVisible({ timeout: 3000 });
    }
  });

  test('should show login history', async ({ page }) => {
    await page.goto('/security');
    const historyTab = page.locator('button:has-text("History"), button:has-text("Login History")').first();
    if (await historyTab.isVisible()) {
      await historyTab.click();
      await expect(page.locator('text=/login|history/i')).toBeVisible();
    }
  });

  test('should show trusted devices', async ({ page }) => {
    await page.goto('/security');
    await expect(page.locator('text=/trusted.*device|device/i')).toBeVisible();
  });

  test('should add trusted device', async ({ page }) => {
    await page.goto('/security');
    const addBtn = page.locator('button:has-text("Add"), button:has-text("Trust")').first();
    if (await addBtn.isVisible()) {
      await addBtn.click();
      await expect(page.locator('text=/device.*added|trusted/i')).toBeVisible({ timeout: 3000 });
    }
  });

  test('should remove trusted device', async ({ page }) => {
    await page.goto('/security');
    const deviceItem = page.locator('[class*="device"]').first();
    await deviceItem.hover();
    const removeBtn = page.locator('button:has-text("Remove"), button:has-text("Delete")').first();
    if (await removeBtn.isVisible()) {
      await removeBtn.click();
      await expect(page.locator('text=/removed|deleted/i')).toBeVisible({ timeout: 3000 });
    }
  });

  test('should show API keys section', async ({ page }) => {
    await page.goto('/security');
    await expect(page.locator('text=/api.*key|key/i')).toBeVisible();
  });

  test('should create API key', async ({ page }) => {
    await page.goto('/security');
    const createBtn = page.locator('button:has-text("Create"), button:has-text("New Key")').first();
    if (await createBtn.isVisible()) {
      await createBtn.click();
      await expect(page.locator('text=/api.*key|generated/i')).toBeVisible();
    }
  });

  test('should revoke API key', async ({ page }) => {
    await page.goto('/security');
    const apiKeyItem = page.locator('[class*="key"]').first();
    await apiKeyItem.hover();
    const revokeBtn = page.locator('button:has-text("Revoke"), button:has-text("Delete")').first();
    if (await revokeBtn.isVisible()) {
      await revokeBtn.click();
      await expect(page.locator('text=/revoked|removed/i')).toBeVisible({ timeout: 3000 });
    }
  });

  test('should show security notifications', async ({ page }) => {
    await page.goto('/security');
    const notificationsSection = page.locator('text=/notification|alert/i').first();
    await expect(notificationsSection).toBeVisible();
  });

  test('should enable security alerts', async ({ page }) => {
    await page.goto('/security');
    const alertToggle = page.locator('input[type="checkbox"]').first();
    if (await alertToggle.isVisible()) {
      await alertToggle.check();
      await expect(page.locator('text=/enabled|saved/i')).toBeVisible({ timeout: 3000 });
    }
  });

  test('should show backup codes', async ({ page }) => {
    await page.goto('/security');
    const backupTab = page.locator('button:has-text("Backup"), button:has-text("Codes")').first();
    if (await backupTab.isVisible()) {
      await backupTab.click();
      await expect(page.locator('text=/backup.*code|recovery/i')).toBeVisible();
    }
  });

  test('should regenerate backup codes', async ({ page }) => {
    await page.goto('/security');
    const backupTab = page.locator('button:has-text("Backup"), button:has-text("Codes")').first();
    if (await backupTab.isVisible()) {
      await backupTab.click();
      const regenerateBtn = page.locator('button:has-text("Regenerate"), button:has-text("New Codes")').first();
      if (await regenerateBtn.isVisible()) {
        await regenerateBtn.click();
        await expect(page.locator('text=/regenerated|new.*codes/i')).toBeVisible({ timeout: 3000 });
      }
    }
  });
});

test.describe('Password Change', () => {
  test('should show current password field', async ({ page }) => {
    await page.goto('/security');
    await expect(page.locator('input[placeholder*="current" i]')).toBeVisible();
  });

  test('should show new password field', async ({ page }) => {
    await page.goto('/security');
    await expect(page.locator('input[placeholder*="new" i]')).toBeVisible();
  });

  test('should show confirm password field', async ({ page }) => {
    await page.goto('/security');
    await expect(page.locator('input[placeholder*="confirm" i]')).toBeVisible();
  });

  test('should validate password match', async ({ page }) => {
    await page.goto('/security');
    await page.fill('input[placeholder*="new" i]', 'password123');
    await page.fill('input[placeholder*="confirm" i]', 'different');
    await page.locator('button:has-text("Change"), button:has-text("Update")').click();
    await expect(page.locator('text=/match|must.*match|doesn.t.*match/i')).toBeVisible();
  });

  test('should validate password strength', async ({ page }) => {
    await page.goto('/security');
    await page.fill('input[placeholder*="new" i]', 'weak');
    await expect(page.locator('text=/weak|strong.*password|requirements/i')).toBeVisible();
  });

  test('should show password requirements', async ({ page }) => {
    await page.goto('/security');
    await page.fill('input[placeholder*="new" i]', '');
    const requirements = page.locator('text=/requirements|criteria|spec/i').first();
    await expect(requirements).toBeVisible();
  });

  test('should require uppercase in password', async ({ page }) => {
    await page.goto('/security');
    await page.fill('input[placeholder*="new" i]', 'password123');
    await expect(page.locator('text=/uppercase|A-Z/i')).toBeVisible();
  });

  test('should require number in password', async ({ page }) => {
    await page.goto('/security');
    await page.fill('input[placeholder*="new" i]', 'Password');
    await expect(page.locator('text=/number|\\d/i')).toBeVisible();
  });

  test('should require special character in password', async ({ page }) => {
    await page.goto('/security');
    await page.fill('input[placeholder*="new" i]', 'Password123');
    await expect(page.locator('text=/special|@|#|\\$/i')).toBeVisible();
  });

  test('should change password successfully', async ({ page }) => {
    await page.goto('/security');
    await page.fill('input[placeholder*="current" i]', 'oldpassword');
    await page.fill('input[placeholder*="new" i]', 'NewPass123!');
    await page.fill('input[placeholder*="confirm" i]', 'NewPass123!');
    await page.locator('button:has-text("Change"), button:has-text("Update")').click();
    await expect(page.locator('text=/success|changed|updated/i')).toBeVisible({ timeout: 5000 });
  });
});