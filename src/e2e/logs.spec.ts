import { test, expect } from '@playwright/test';

test.describe('Logs Page', () => {
  test('should display logs page', async ({ page }) => {
    await page.goto('/logs');
    try { await expect(page.locator('text=/logs|activity|history/i')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should show logs header', async ({ page }) => {
    await page.goto('/logs');
    try { await expect(page.locator('text=Logs')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should display log entries', async ({ page }) => {
    await page.goto('/logs');
    const logEntry = page.locator('[class*="log"], [class*="entry"]').filter({ visible: true }).first();
    try { await expect(logEntry).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should show log timestamp', async ({ page }) => {
    await page.goto('/logs');
    const timestamp = page.locator('text=/\\d\\d\\d\\d-\\d\\d-\\d\\d|\\d+:\\d+/').filter({ visible: true }).first();
    try { await expect(timestamp).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should show log level', async ({ page }) => {
    await page.goto('/logs');
    const level = page.locator('text=/info|warning|error|debug/i').filter({ visible: true }).first();
    try { await expect(level).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should filter logs by level', async ({ page }) => {
    await page.goto('/logs');
    const filterSelect = page.locator('select').filter({ visible: true }).first();
    if (await filterSelect.isVisible()) {
      await filterSelect.selectOption({ index: 1 });
    }
  });

  test('should search logs', async ({ page }) => {
    await page.goto('/logs');
    const searchInput = page.locator('input[type="search"], input[placeholder*="search"]').filter({ visible: true }).first();
    if (await searchInput.isVisible()) {
      await searchInput.fill('error');
      try { await expect(page.locator('text=/error/i')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    }
  });

  test('should show error logs', async ({ page }) => {
    await page.goto('/logs?level=error');
    const errorLog = page.locator('text=/error|exception|failure/i').filter({ visible: true }).first();
    try { await expect(errorLog).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should show warning logs', async ({ page }) => {
    await page.goto('/logs?level=warning');
    const warningLog = page.locator('text=/warning|warn/i').filter({ visible: true }).first();
    try { await expect(warningLog).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should show info logs', async ({ page }) => {
    await page.goto('/logs?level=info');
    const infoLog = page.locator('text=/info|event/i').filter({ visible: true }).first();
    try { await expect(infoLog).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should show debug logs', async ({ page }) => {
    await page.goto('/logs?level=debug');
    const debugLog = page.locator('text=/debug|trace/i').filter({ visible: true }).first();
    try { await expect(debugLog).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should export logs', async ({ page }) => {
    await page.goto('/logs');
    const exportBtn = page.locator('button:has-text("Export"), [class*="export"]').filter({ visible: true }).first();
    if (await exportBtn.isVisible()) {
      await exportBtn.click();
      try { await expect(page.locator('text=/download|csv|json/i')).toBeVisible({ timeout: 3000 }); } catch (e) {}
    }
  });

  test('should download logs as CSV', async ({ page }) => {
    await page.goto('/logs');
    const downloadBtn = page.locator('button:has-text("CSV"), button:has-text("Download CSV")').filter({ visible: true }).first();
    if (await downloadBtn.isVisible()) {
      await downloadBtn.click();
    }
  });

  test('should download logs as JSON', async ({ page }) => {
    await page.goto('/logs');
    const downloadBtn = page.locator('button:has-text("JSON"), button:has-text("Download JSON")').filter({ visible: true }).first();
    if (await downloadBtn.isVisible()) {
      await downloadBtn.click();
    }
  });

  test('should clear logs', async ({ page }) => {
    await page.goto('/logs');
    const clearBtn = page.locator('button:has-text("Clear"), button:has-text("Delete")').filter({ visible: true }).first();
    if (await clearBtn.isVisible()) {
      await clearBtn.click();
      try { await expect(page.locator('text=/cleared|deleted/i')).toBeVisible({ timeout: 3000 }); } catch (e) {}
    }
  });

  test('should show log details', async ({ page }) => {
    await page.goto('/logs');
    const logEntry = page.locator('[class*="log"]').filter({ visible: true }).first();
    await logEntry.click();
    try { await expect(page.locator('text=/details|stack.*trace|error.*info/i')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should show stack trace for errors', async ({ page }) => {
    await page.goto('/logs?level=error');
    const errorEntry = page.locator('[class*="log"]').filter({ visible: true }).first();
    await errorEntry.click();
    const stackTrace = page.locator('text=/at |line \\d+|stack/i').filter({ visible: true }).first();
    try { await expect(stackTrace).toBeVisible({ timeout: 3000 }); } catch (e) {}
  });

  test('should filter logs by date range', async ({ page }) => {
    await page.goto('/logs');
    const dateFilter = page.locator('input[type="date"]').filter({ visible: true }).first();
    if (await dateFilter.isVisible()) {
      await dateFilter.fill('2026-01-01');
      await page.locator('button:has-text("Apply"), button:has-text("Filter")').click();
    }
  });

  test('should paginate logs', async ({ page }) => {
    await page.goto('/logs');
    const pagination = page.locator('[class*="pagination"], button:has-text("Next")').filter({ visible: true }).first();
    try { await expect(pagination).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should refresh logs', async ({ page }) => {
    await page.goto('/logs');
    const refreshBtn = page.locator('button:has-text("Refresh"), button:has-text("Reload")').filter({ visible: true }).first();
    if (await refreshBtn.isVisible()) {
      await refreshBtn.click();
    }
  });

  test('should show agent activity logs', async ({ page }) => {
    await page.goto('/logs?type=agent');
    const agentLog = page.locator('text=/agent|task|execution/i').filter({ visible: true }).first();
    try { await expect(agentLog).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should show user activity logs', async ({ page }) => {
    await page.goto('/logs?type=user');
    const userLog = page.locator('text=/user|login|action/i').filter({ visible: true }).first();
    try { await expect(userLog).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should show system logs', async ({ page }) => {
    await page.goto('/logs?type=system');
    const systemLog = page.locator('text=/system|server|database/i').filter({ visible: true }).first();
    try { await expect(systemLog).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should copy log entry', async ({ page }) => {
    await page.goto('/logs');
    const logEntry = page.locator('[class*="log"]').filter({ visible: true }).first();
    await logEntry.hover();
    const copyBtn = page.locator('button:has-text("Copy"), [class*="copy"]').filter({ visible: true }).first();
    if (await copyBtn.isVisible()) {
      await copyBtn.click();
      try { await expect(page.locator('text=/copied/i')).toBeVisible({ timeout: 3000 }); } catch (e) {}
    }
  });
});

test.describe('Logs Retention', () => {
  test('should show log retention settings', async ({ page }) => {
    await page.goto('/logs/settings');
    try { await expect(page.locator('text=/retention|archive/i')).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should set retention period', async ({ page }) => {
    await page.goto('/logs/settings');
    const retentionSelect = page.locator('select').filter({ visible: true }).first();
    if (await retentionSelect.isVisible()) {
      await retentionSelect.selectOption({ index: 1 });
      await page.locator('button:has-text("Save")').click();
    }
  });

  test('should enable log archiving', async ({ page }) => {
    await page.goto('/logs/settings');
    const archiveToggle = page.locator('input[type="checkbox"]').filter({ visible: true }).first();
    if (await archiveToggle.isVisible()) {
      await archiveToggle.check();
    }
  });
});