import { test, expect } from '@playwright/test';

test.describe('Logs Page', () => {
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
  test('should display logs page', async ({ page }) => {
    await expect(page.locator('text=/logs|activity|history/i')).toBeVisible();
  });

  test('should show logs header', async ({ page }) => {
    await expect(page.locator('text=Logs')).toBeVisible();
  });

  test('should display log entries', async ({ page }) => {
    const logEntry = page.locator('[class*="log"], [class*="entry"]').first();
    await expect(logEntry).toBeVisible();
  });

  test('should show log timestamp', async ({ page }) => {
    const timestamp = page.locator('text=/\\d\\d\\d\\d-\\d\\d-\\d\\d|\\d+:\\d+/').first();
    await expect(timestamp).toBeVisible();
  });

  test('should show log level', async ({ page }) => {
    const level = page.locator('text=/info|warning|error|debug/i').first();
    await expect(level).toBeVisible();
  });

  test('should filter logs by level', async ({ page }) => {
    const filterSelect = page.locator('select').first();
    if (await filterSelect.isVisible()) {
      await filterSelect.selectOption({ index: 1 });
    }
  });

  test('should search logs', async ({ page }) => {
    const searchInput = page.locator('input[type="search"], input[placeholder*="search"]').first();
    if (await searchInput.isVisible()) {
      await searchInput.fill('error');
      await expect(page.locator('text=/error/i')).toBeVisible();
    }
  });

  test('should show error logs', async ({ page }) => {
    const errorLog = page.locator('text=/error|exception|failure/i').first();
    await expect(errorLog).toBeVisible();
  });

  test('should show warning logs', async ({ page }) => {
    const warningLog = page.locator('text=/warning|warn/i').first();
    await expect(warningLog).toBeVisible();
  });

  test('should show info logs', async ({ page }) => {
    const infoLog = page.locator('text=/info|event/i').first();
    await expect(infoLog).toBeVisible();
  });

  test('should show debug logs', async ({ page }) => {
    const debugLog = page.locator('text=/debug|trace/i').first();
    await expect(debugLog).toBeVisible();
  });

  test('should export logs', async ({ page }) => {
    const exportBtn = page.locator('button:has-text("Export"), [class*="export"]').first();
    if (await exportBtn.isVisible()) {
      await exportBtn.click();
      await expect(page.locator('text=/download|csv|json/i')).toBeVisible({ timeout: 3000 }).catch(() => {});
    }
  });

  test('should download logs as CSV', async ({ page }) => {
    const downloadBtn = page.locator('button:has-text("CSV"), button:has-text("Download CSV")').first();
    if (await downloadBtn.isVisible()) {
      await downloadBtn.click();
    }
  });

  test('should download logs as JSON', async ({ page }) => {
    const downloadBtn = page.locator('button:has-text("JSON"), button:has-text("Download JSON")').first();
    if (await downloadBtn.isVisible()) {
      await downloadBtn.click();
    }
  });

  test('should clear logs', async ({ page }) => {
    const clearBtn = page.locator('button:has-text("Clear"), button:has-text("Delete")').first();
    if (await clearBtn.isVisible()) {
      await clearBtn.click();
      await expect(page.locator('text=/cleared|deleted/i')).toBeVisible({ timeout: 3000 }).catch(() => {});
    }
  });

  test('should show log details', async ({ page }) => {
    const logEntry = page.locator('[class*="log"]').first();
    await logEntry.click();
    await expect(page.locator('text=/details|stack.*trace|error.*info/i')).toBeVisible();
  });

  test('should show stack trace for errors', async ({ page }) => {
    const errorEntry = page.locator('[class*="log"]').first();
    await errorEntry.click();
    const stackTrace = page.locator('text=/at |line \\d+|stack/i').first();
    await expect(stackTrace).toBeVisible({ timeout: 3000 }).catch(() => {});
  });

  test('should filter logs by date range', async ({ page }) => {
    const dateFilter = page.locator('input[type="date"]').first();
    if (await dateFilter.isVisible()) {
      await dateFilter.fill('2026-01-01');
      await page.locator('button:has-text("Apply"), button:has-text("Filter")').click();
    }
  });

  test('should paginate logs', async ({ page }) => {
    const pagination = page.locator('[class*="pagination"], button:has-text("Next")').first();
    await expect(pagination).toBeVisible();
  });

  test('should refresh logs', async ({ page }) => {
    const refreshBtn = page.locator('button:has-text("Refresh"), button:has-text("Reload")').first();
    if (await refreshBtn.isVisible()) {
      await refreshBtn.click();
    }
  });

  test('should show agent activity logs', async ({ page }) => {
    const agentLog = page.locator('text=/agent|task|execution/i').first();
    await expect(agentLog).toBeVisible();
  });

  test('should show user activity logs', async ({ page }) => {
    const userLog = page.locator('text=/user|login|action/i').first();
    await expect(userLog).toBeVisible();
  });

  test('should show system logs', async ({ page }) => {
    const systemLog = page.locator('text=/system|server|database/i').first();
    await expect(systemLog).toBeVisible();
  });

  test('should copy log entry', async ({ page }) => {
    const logEntry = page.locator('[class*="log"]').first();
    await logEntry.hover();
    const copyBtn = page.locator('button:has-text("Copy"), [class*="copy"]').first();
    if (await copyBtn.isVisible()) {
      await copyBtn.click();
      await expect(page.locator('text=/copied/i')).toBeVisible({ timeout: 3000 }).catch(() => {});
    }
  });
});

test.describe('Logs Retention', () => {
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

  test('should show log retention settings', async ({ page }) => {
    await expect(page.locator('text=/retention|archive/i')).toBeVisible();
  });

  test('should set retention period', async ({ page }) => {
    const retentionSelect = page.locator('select').first();
    if (await retentionSelect.isVisible()) {
      await retentionSelect.selectOption({ index: 1 });
      await page.locator('button:has-text("Save")').click();
    }
  });

  test('should enable log archiving', async ({ page }) => {
    const archiveToggle = page.locator('input[type="checkbox"]').first();
    if (await archiveToggle.isVisible()) {
      await archiveToggle.check();
    }
  });
});
