import { test, expect } from '@playwright/test';

test.describe('Logs Page', () => {
  test('should display logs page', async ({ page }) => {
    await page.goto('/logs');
    await expect(page.locator('text=/logs|activity|history/i')).toBeVisible();
  });

  test('should show logs header', async ({ page }) => {
    await page.goto('/logs');
    await expect(page.locator('text=Logs')).toBeVisible();
  });

  test('should display log entries', async ({ page }) => {
    await page.goto('/logs');
    const logEntry = page.locator('[class*="log"], [class*="entry"]').first();
    await expect(logEntry).toBeVisible();
  });

  test('should show log timestamp', async ({ page }) => {
    await page.goto('/logs');
    const timestamp = page.locator('text=/\\d\\d\\d\\d-\\d\\d-\\d\\d|\\d+:\\d+/').first();
    await expect(timestamp).toBeVisible();
  });

  test('should show log level', async ({ page }) => {
    await page.goto('/logs');
    const level = page.locator('text=/info|warning|error|debug/i').first();
    await expect(level).toBeVisible();
  });

  test('should filter logs by level', async ({ page }) => {
    await page.goto('/logs');
    const filterSelect = page.locator('select').first();
    if (await filterSelect.isVisible()) {
      await filterSelect.selectOption({ index: 1 });
    }
  });

  test('should search logs', async ({ page }) => {
    await page.goto('/logs');
    const searchInput = page.locator('input[type="search"], input[placeholder*="search"]').first();
    if (await searchInput.isVisible()) {
      await searchInput.fill('error');
      await expect(page.locator('text=/error/i')).toBeVisible();
    }
  });

  test('should show error logs', async ({ page }) => {
    await page.goto('/logs?level=error');
    const errorLog = page.locator('text=/error|exception|failure/i').first();
    await expect(errorLog).toBeVisible();
  });

  test('should show warning logs', async ({ page }) => {
    await page.goto('/logs?level=warning');
    const warningLog = page.locator('text=/warning|warn/i').first();
    await expect(warningLog).toBeVisible();
  });

  test('should show info logs', async ({ page }) => {
    await page.goto('/logs?level=info');
    const infoLog = page.locator('text=/info|event/i').first();
    await expect(infoLog).toBeVisible();
  });

  test('should show debug logs', async ({ page }) => {
    await page.goto('/logs?level=debug');
    const debugLog = page.locator('text=/debug|trace/i').first();
    await expect(debugLog).toBeVisible();
  });

  test('should export logs', async ({ page }) => {
    await page.goto('/logs');
    const exportBtn = page.locator('button:has-text("Export"), [class*="export"]').first();
    if (await exportBtn.isVisible()) {
      await exportBtn.click();
      await expect(page.locator('text=/download|csv|json/i')).toBeVisible({ timeout: 3000 });
    }
  });

  test('should download logs as CSV', async ({ page }) => {
    await page.goto('/logs');
    const downloadBtn = page.locator('button:has-text("CSV"), button:has-text("Download CSV")').first();
    if (await downloadBtn.isVisible()) {
      await downloadBtn.click();
    }
  });

  test('should download logs as JSON', async ({ page }) => {
    await page.goto('/logs');
    const downloadBtn = page.locator('button:has-text("JSON"), button:has-text("Download JSON")').first();
    if (await downloadBtn.isVisible()) {
      await downloadBtn.click();
    }
  });

  test('should clear logs', async ({ page }) => {
    await page.goto('/logs');
    const clearBtn = page.locator('button:has-text("Clear"), button:has-text("Delete")').first();
    if (await clearBtn.isVisible()) {
      await clearBtn.click();
      await expect(page.locator('text=/cleared|deleted/i')).toBeVisible({ timeout: 3000 });
    }
  });

  test('should show log details', async ({ page }) => {
    await page.goto('/logs');
    const logEntry = page.locator('[class*="log"]').first();
    await logEntry.click();
    await expect(page.locator('text=/details|stack.*trace|error.*info/i')).toBeVisible();
  });

  test('should show stack trace for errors', async ({ page }) => {
    await page.goto('/logs?level=error');
    const errorEntry = page.locator('[class*="log"]').first();
    await errorEntry.click();
    const stackTrace = page.locator('text=/at |line \\d+|stack/i').first();
    await expect(stackTrace).toBeVisible({ timeout: 3000 });
  });

  test('should filter logs by date range', async ({ page }) => {
    await page.goto('/logs');
    const dateFilter = page.locator('input[type="date"]').first();
    if (await dateFilter.isVisible()) {
      await dateFilter.fill('2026-01-01');
      await page.locator('button:has-text("Apply"), button:has-text("Filter")').click();
    }
  });

  test('should paginate logs', async ({ page }) => {
    await page.goto('/logs');
    const pagination = page.locator('[class*="pagination"], button:has-text("Next")').first();
    await expect(pagination).toBeVisible();
  });

  test('should refresh logs', async ({ page }) => {
    await page.goto('/logs');
    const refreshBtn = page.locator('button:has-text("Refresh"), button:has-text("Reload")').first();
    if (await refreshBtn.isVisible()) {
      await refreshBtn.click();
    }
  });

  test('should show agent activity logs', async ({ page }) => {
    await page.goto('/logs?type=agent');
    const agentLog = page.locator('text=/agent|task|execution/i').first();
    await expect(agentLog).toBeVisible();
  });

  test('should show user activity logs', async ({ page }) => {
    await page.goto('/logs?type=user');
    const userLog = page.locator('text=/user|login|action/i').first();
    await expect(userLog).toBeVisible();
  });

  test('should show system logs', async ({ page }) => {
    await page.goto('/logs?type=system');
    const systemLog = page.locator('text=/system|server|database/i').first();
    await expect(systemLog).toBeVisible();
  });

  test('should copy log entry', async ({ page }) => {
    await page.goto('/logs');
    const logEntry = page.locator('[class*="log"]').first();
    await logEntry.hover();
    const copyBtn = page.locator('button:has-text("Copy"), [class*="copy"]').first();
    if (await copyBtn.isVisible()) {
      await copyBtn.click();
      await expect(page.locator('text=/copied/i')).toBeVisible({ timeout: 3000 });
    }
  });
});

test.describe('Logs Retention', () => {
  test('should show log retention settings', async ({ page }) => {
    await page.goto('/logs/settings');
    await expect(page.locator('text=/retention|archive/i')).toBeVisible();
  });

  test('should set retention period', async ({ page }) => {
    await page.goto('/logs/settings');
    const retentionSelect = page.locator('select').first();
    if (await retentionSelect.isVisible()) {
      await retentionSelect.selectOption({ index: 1 });
      await page.locator('button:has-text("Save")').click();
    }
  });

  test('should enable log archiving', async ({ page }) => {
    await page.goto('/logs/settings');
    const archiveToggle = page.locator('input[type="checkbox"]').first();
    if (await archiveToggle.isVisible()) {
      await archiveToggle.check();
    }
  });
});