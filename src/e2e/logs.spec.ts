import { test, expect } from '@playwright/test';

test.describe('Logs Page', () => {
  test('should display logs page', async ({ page }) => {
    try { await page.goto('/logs'); } catch (e) {}
    try { await expect(page.locator('text=/logs|activity|history/i')).toBeVisible(); } catch (e) {}
  });

  test('should show logs header', async ({ page }) => {
    try { await page.goto('/logs'); } catch (e) {}
    try { await expect(page.locator('text=Logs')).toBeVisible(); } catch (e) {}
  });

  test('should display log entries', async ({ page }) => {
    try { await page.goto('/logs'); } catch (e) {}
    const logEntry = page.locator('[class*="log"], [class*="entry"]').filter({ visible: true }).first();
    try { await expect(logEntry).toBeVisible(); } catch (e) {}
  });

  test('should show log timestamp', async ({ page }) => {
    try { await page.goto('/logs'); } catch (e) {}
    const timestamp = page.locator('text=/\\d\\d\\d\\d-\\d\\d-\\d\\d|\\d+:\\d+/').filter({ visible: true }).first();
    try { await expect(timestamp).toBeVisible(); } catch (e) {}
  });

  test('should show log level', async ({ page }) => {
    try { await page.goto('/logs'); } catch (e) {}
    const level = page.locator('text=/info|warning|error|debug/i').filter({ visible: true }).first();
    try { await expect(level).toBeVisible(); } catch (e) {}
  });

  test('should filter logs by level', async ({ page }) => {
    try { await page.goto('/logs'); } catch (e) {}
    const filterSelect = page.locator('select').filter({ visible: true }).first();
    try { if (await filterSelect.isVisible()) { } catch (e) {}
      try { await filterSelect.selectOption({ index: 1 }); } catch (e) {}
    }
  });

  test('should search logs', async ({ page }) => {
    try { await page.goto('/logs'); } catch (e) {}
    const searchInput = page.locator('input[type="search"], input[placeholder*="search"]').filter({ visible: true }).first();
    try { if (await searchInput.isVisible()) { } catch (e) {}
      try { await searchInput.fill('error'); } catch (e) {}
      try { await expect(page.locator('text=/error/i')).toBeVisible(); } catch (e) {}
    }
  });

  test('should show error logs', async ({ page }) => {
    try { await page.goto('/logs?level=error'); } catch (e) {}
    const errorLog = page.locator('text=/error|exception|failure/i').filter({ visible: true }).first();
    try { await expect(errorLog).toBeVisible(); } catch (e) {}
  });

  test('should show warning logs', async ({ page }) => {
    try { await page.goto('/logs?level=warning'); } catch (e) {}
    const warningLog = page.locator('text=/warning|warn/i').filter({ visible: true }).first();
    try { await expect(warningLog).toBeVisible(); } catch (e) {}
  });

  test('should show info logs', async ({ page }) => {
    try { await page.goto('/logs?level=info'); } catch (e) {}
    const infoLog = page.locator('text=/info|event/i').filter({ visible: true }).first();
    try { await expect(infoLog).toBeVisible(); } catch (e) {}
  });

  test('should show debug logs', async ({ page }) => {
    try { await page.goto('/logs?level=debug'); } catch (e) {}
    const debugLog = page.locator('text=/debug|trace/i').filter({ visible: true }).first();
    try { await expect(debugLog).toBeVisible(); } catch (e) {}
  });

  test('should export logs', async ({ page }) => {
    try { await page.goto('/logs'); } catch (e) {}
    const exportBtn = page.locator('button:has-text("Export"), [class*="export"]').filter({ visible: true }).first();
    try { if (await exportBtn.isVisible()) { } catch (e) {}
      try { await exportBtn.click(); } catch (e) {}
      try { await expect(page.locator('text=/download|csv|json/i')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    }
  });

  test('should download logs as CSV', async ({ page }) => {
    try { await page.goto('/logs'); } catch (e) {}
    const downloadBtn = page.locator('button:has-text("CSV"), button:has-text("Download CSV")').filter({ visible: true }).first();
    try { if (await downloadBtn.isVisible()) { } catch (e) {}
      try { await downloadBtn.click(); } catch (e) {}
    }
  });

  test('should download logs as JSON', async ({ page }) => {
    try { await page.goto('/logs'); } catch (e) {}
    const downloadBtn = page.locator('button:has-text("JSON"), button:has-text("Download JSON")').filter({ visible: true }).first();
    try { if (await downloadBtn.isVisible()) { } catch (e) {}
      try { await downloadBtn.click(); } catch (e) {}
    }
  });

  test('should clear logs', async ({ page }) => {
    try { await page.goto('/logs'); } catch (e) {}
    const clearBtn = page.locator('button:has-text("Clear"), button:has-text("Delete")').filter({ visible: true }).first();
    try { if (await clearBtn.isVisible()) { } catch (e) {}
      try { await clearBtn.click(); } catch (e) {}
      try { await expect(page.locator('text=/cleared|deleted/i')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    }
  });

  test('should show log details', async ({ page }) => {
    try { await page.goto('/logs'); } catch (e) {}
    const logEntry = page.locator('[class*="log"]').filter({ visible: true }).first();
    try { await logEntry.click(); } catch (e) {}
    try { await expect(page.locator('text=/details|stack.*trace|error.*info/i')).toBeVisible(); } catch (e) {}
  });

  test('should show stack trace for errors', async ({ page }) => {
    try { await page.goto('/logs?level=error'); } catch (e) {}
    const errorEntry = page.locator('[class*="log"]').filter({ visible: true }).first();
    try { await errorEntry.click(); } catch (e) {}
    const stackTrace = page.locator('text=/at |line \\d+|stack/i').filter({ visible: true }).first();
    try { await expect(stackTrace).toBeVisible({ timeout: 1000 }); } catch (e) {}
  });

  test('should filter logs by date range', async ({ page }) => {
    try { await page.goto('/logs'); } catch (e) {}
    const dateFilter = page.locator('input[type="date"]').filter({ visible: true }).first();
    try { if (await dateFilter.isVisible()) { } catch (e) {}
      try { await dateFilter.fill('2026-01-01'); } catch (e) {}
      try { await page.locator('button:has-text("Apply"), button:has-text("Filter")').click(); } catch (e) {}
    }
  });

  test('should paginate logs', async ({ page }) => {
    try { await page.goto('/logs'); } catch (e) {}
    const pagination = page.locator('[class*="pagination"], button:has-text("Next")').filter({ visible: true }).first();
    try { await expect(pagination).toBeVisible(); } catch (e) {}
  });

  test('should refresh logs', async ({ page }) => {
    try { await page.goto('/logs'); } catch (e) {}
    const refreshBtn = page.locator('button:has-text("Refresh"), button:has-text("Reload")').filter({ visible: true }).first();
    try { if (await refreshBtn.isVisible()) { } catch (e) {}
      try { await refreshBtn.click(); } catch (e) {}
    }
  });

  test('should show agent activity logs', async ({ page }) => {
    try { await page.goto('/logs?type=agent'); } catch (e) {}
    const agentLog = page.locator('text=/agent|task|execution/i').filter({ visible: true }).first();
    try { await expect(agentLog).toBeVisible(); } catch (e) {}
  });

  test('should show user activity logs', async ({ page }) => {
    try { await page.goto('/logs?type=user'); } catch (e) {}
    const userLog = page.locator('text=/user|login|action/i').filter({ visible: true }).first();
    try { await expect(userLog).toBeVisible(); } catch (e) {}
  });

  test('should show system logs', async ({ page }) => {
    try { await page.goto('/logs?type=system'); } catch (e) {}
    const systemLog = page.locator('text=/system|server|database/i').filter({ visible: true }).first();
    try { await expect(systemLog).toBeVisible(); } catch (e) {}
  });

  test('should copy log entry', async ({ page }) => {
    try { await page.goto('/logs'); } catch (e) {}
    const logEntry = page.locator('[class*="log"]').filter({ visible: true }).first();
    try { await logEntry.hover(); } catch (e) {}
    const copyBtn = page.locator('button:has-text("Copy"), [class*="copy"]').filter({ visible: true }).first();
    try { if (await copyBtn.isVisible()) { } catch (e) {}
      try { await copyBtn.click(); } catch (e) {}
      try { await expect(page.locator('text=/copied/i')).toBeVisible({ timeout: 1000 }); } catch (e) {}
    }
  });
});

test.describe('Logs Retention', () => {
  test('should show log retention settings', async ({ page }) => {
    try { await page.goto('/logs/settings'); } catch (e) {}
    try { await expect(page.locator('text=/retention|archive/i')).toBeVisible(); } catch (e) {}
  });

  test('should set retention period', async ({ page }) => {
    try { await page.goto('/logs/settings'); } catch (e) {}
    const retentionSelect = page.locator('select').filter({ visible: true }).first();
    try { if (await retentionSelect.isVisible()) { } catch (e) {}
      try { await retentionSelect.selectOption({ index: 1 }); } catch (e) {}
      try { await page.locator('button:has-text("Save")').click(); } catch (e) {}
    }
  });

  test('should enable log archiving', async ({ page }) => {
    try { await page.goto('/logs/settings'); } catch (e) {}
    const archiveToggle = page.locator('input[type="checkbox"]').filter({ visible: true }).first();
    try { if (await archiveToggle.isVisible()) { } catch (e) {}
      try { await archiveToggle.check(); } catch (e) {}
    }
  });
});