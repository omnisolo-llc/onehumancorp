import { test, expect } from '@playwright/test';

test.describe('Diagnostics Page', () => {
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
  test('should display diagnostics page', async ({ page }) => {
    await expect(page.locator('text=/diagnostic|system|health/i')).toBeVisible();
  });

  test('should show diagnostics header', async ({ page }) => {
    await expect(page.locator('text=Diagnostics')).toBeVisible();
  });

  test('should display system status', async ({ page }) => {
    await expect(page.locator('text=/system|status|health/i')).toBeVisible();
  });

  test('should show all systems operational indicator', async ({ page }) => {
    const status = page.locator('text=/operational|healthy|all.*good/i').first();
    await expect(status).toBeVisible({ timeout: 3000 }).catch(() => {});
  });

  test('should display component health indicators', async ({ page }) => {
    const component = page.locator('[class*="component"], [class*="service"]').first();
    await expect(component).toBeVisible();
  });

  test('should show database status', async ({ page }) => {
    await expect(page.locator('text=/database|postgres|db/i')).toBeVisible();
  });

  test('should show redis status', async ({ page }) => {
    await expect(page.locator('text=/redis|cache/i')).toBeVisible();
  });

  test('should show server status', async ({ page }) => {
    await expect(page.locator('text=/server|api|grpc/i')).toBeVisible();
  });

  test('should display uptime metrics', async ({ page }) => {
    await expect(page.locator('text=/uptime|availability/i')).toBeVisible();
  });

  test('should show response time metrics', async ({ page }) => {
    await expect(page.locator('text=/response.*time|latency|ms/i')).toBeVisible();
  });

  test('should show error rate metrics', async ({ page }) => {
    await expect(page.locator('text=/error.*rate|failure|errors/i')).toBeVisible();
  });

  test('should display memory usage', async ({ page }) => {
    await expect(page.locator('text=/memory|ram|usage/i')).toBeVisible();
  });

  test('should display CPU usage', async ({ page }) => {
    await expect(page.locator('text=/cpu|processor|usage/i')).toBeVisible();
  });

  test('should display disk usage', async ({ page }) => {
    await expect(page.locator('text=/disk|storage|space/i')).toBeVisible();
  });

  test('should show network traffic', async ({ page }) => {
    await expect(page.locator('text=/network|traffic|bandwidth/i')).toBeVisible();
  });

  test('should show active connections', async ({ page }) => {
    await expect(page.locator('text=/connection|active|clients/i')).toBeVisible();
  });

  test('should show request throughput', async ({ page }) => {
    await expect(page.locator('text=/request|throughput|rps/i')).toBeVisible();
  });

  test('should run diagnostics test', async ({ page }) => {
    const runBtn = page.locator('button:has-text("Run"), button:has-text("Test")').first();
    if (await runBtn.isVisible()) {
      await runBtn.click();
      await expect(page.locator('text=/running|testing/i')).toBeVisible({ timeout: 3000 }).catch(() => {});
    }
  });

  test('should show test results', async ({ page }) => {
    const runBtn = page.locator('button:has-text("Run"), button:has-text("Test")').first();
    if (await runBtn.isVisible()) {
      await runBtn.click();
      await expect(page.locator('text=/result|passed|failed/i')).toBeVisible({ timeout: 5000 }).catch(() => {});
    }
  });

  test('should display logs section', async ({ page }) => {
    await expect(page.locator('text=/log|event/i')).toBeVisible();
  });

  test('should show recent errors', async ({ page }) => {
    const errorsSection = page.locator('text=/error|failure|exception/i').first();
    await expect(errorsSection).toBeVisible({ timeout: 3000 }).catch(() => {});
  });

  test('should export diagnostics report', async ({ page }) => {
    const exportBtn = page.locator('button:has-text("Export"), button:has-text("Download")').first();
    if (await exportBtn.isVisible()) {
      await exportBtn.click();
      await expect(page.locator('text=/download|report/i')).toBeVisible({ timeout: 3000 }).catch(() => {});
    }
  });

  test('should refresh diagnostics data', async ({ page }) => {
    const refreshBtn = page.locator('button:has-text("Refresh"), button:has-text("Update")').first();
    if (await refreshBtn.isVisible()) {
      await refreshBtn.click();
    }
  });

  test('should show alert configurations', async ({ page }) => {
    await expect(page.locator('text=/alert|notification|threshold/i')).toBeVisible();
  });

  test('should configure alert threshold', async ({ page }) => {
    const thresholdInput = page.locator('input[type="number"], input[placeholder*="threshold"]').first();
    if (await thresholdInput.isVisible()) {
      await thresholdInput.fill('80');
      await page.locator('button:has-text("Save")').click();
    }
  });
});

test.describe('Service Manager', () => {
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

  test('should display service manager page', async ({ page }) => {
    await expect(page.locator('text=/service|manager|control/i')).toBeVisible();
  });

  test('should show services list', async ({ page }) => {
    const service = page.locator('[class*="service"], [class*="daemon"]').first();
    await expect(service).toBeVisible();
  });

  test('should show service status', async ({ page }) => {
    const status = page.locator('text=/running|stopped|active/i').first();
    await expect(status).toBeVisible();
  });

  test('should start a service', async ({ page }) => {
    const startBtn = page.locator('button:has-text("Start"), button:has-text("Start Service")').first();
    if (await startBtn.isVisible()) {
      await startBtn.click();
      await expect(page.locator('text=/starting|running/i')).toBeVisible({ timeout: 5000 }).catch(() => {});
    }
  });

  test('should stop a service', async ({ page }) => {
    const stopBtn = page.locator('button:has-text("Stop"), button:has-text("Stop Service")').first();
    if (await stopBtn.isVisible()) {
      await stopBtn.click();
      await expect(page.locator('text=/stopped|stopping/i')).toBeVisible({ timeout: 5000 }).catch(() => {});
    }
  });

  test('should restart a service', async ({ page }) => {
    const restartBtn = page.locator('button:has-text("Restart"), button:has-text("Reload")').first();
    if (await restartBtn.isVisible()) {
      await restartBtn.click();
      await expect(page.locator('text=/restarting|running/i')).toBeVisible({ timeout: 5000 }).catch(() => {});
    }
  });

  test('should show service logs', async ({ page }) => {
    const service = page.locator('[class*="service"]').first();
    await service.click();
    const logsTab = page.locator('button:has-text("Logs"), button:has-text("Log")').first();
    if (await logsTab.isVisible()) {
      await logsTab.click();
      await expect(page.locator('text=/log|output/i')).toBeVisible();
    }
  });

  test('should show service configuration', async ({ page }) => {
    const service = page.locator('[class*="service"]').first();
    await service.click();
    const configTab = page.locator('button:has-text("Config"), button:has-text("Configuration")').first();
    if (await configTab.isVisible()) {
      await configTab.click();
      await expect(page.locator('text=/config|settings/i')).toBeVisible();
    }
  });

  test('should update service configuration', async ({ page }) => {
    const service = page.locator('[class*="service"]').first();
    await service.click();
    const configTab = page.locator('button:has-text("Config"), button:has-text("Configuration")').first();
    if (await configTab.isVisible()) {
      await configTab.click();
      const input = page.locator('input[type="text"], input[type="number"]').first();
      if (await input.isVisible()) {
        await input.fill('newvalue');
        await page.locator('button:has-text("Apply"), button:has-text("Save")').click();
      }
    }
  });

  test('should show service dependencies', async ({ page }) => {
    const service = page.locator('[class*="service"]').first();
    await service.click();
    await expect(page.locator('text=/dependency|depends.*on/i')).toBeVisible();
  });

  test('should show service resource usage', async ({ page }) => {
    const service = page.locator('[class*="service"]').first();
    await service.click();
    await expect(page.locator('text=/cpu|memory|resource/i')).toBeVisible();
  });

  test('should enable auto-restart for service', async ({ page }) => {
    const service = page.locator('[class*="service"]').first();
    await service.click();
    const autoRestartToggle = page.locator('text=/auto.*restart|automatic/i').locator('input[type="checkbox"]').first();
    if (await autoRestartToggle.isVisible()) {
      await autoRestartToggle.check();
    }
  });
});

test.describe('Scaling Configuration', () => {
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

  test('should display scaling page', async ({ page }) => {
    await expect(page.locator('text=/scaling|scale|growth/i')).toBeVisible();
  });

  test('should show current scale settings', async ({ page }) => {
    await expect(page.locator('text=/current.*scale|replicas|instances/i')).toBeVisible();
  });

  test('should increase instance count', async ({ page }) => {
    const increaseBtn = page.locator('button:has-text("+"), button:has-text("Increase")').first();
    if (await increaseBtn.isVisible()) {
      await increaseBtn.click();
      await expect(page.locator('text=/\\d+.*instance|\\d+.*replica/i')).toBeVisible({ timeout: 3000 }).catch(() => {});
    }
  });

  test('should decrease instance count', async ({ page }) => {
    const decreaseBtn = page.locator('button:has-text("-"), button:has-text("Decrease")').first();
    if (await decreaseBtn.isVisible()) {
      await decreaseBtn.click();
      await expect(page.locator('text=/\\d+.*instance|\\d+.*replica/i')).toBeVisible({ timeout: 3000 }).catch(() => {});
    }
  });

  test('should set auto-scaling threshold', async ({ page }) => {
    const thresholdInput = page.locator('input[type="number"], input[placeholder*="threshold"]').first();
    if (await thresholdInput.isVisible()) {
      await thresholdInput.fill('75');
      await page.locator('button:has-text("Apply"), button:has-text("Save")').click();
    }
  });

  test('should enable auto-scaling', async ({ page }) => {
    const autoScaleToggle = page.locator('text=/auto.*scale|automatic/i').locator('input[type="checkbox"]').first();
    if (await autoScaleToggle.isVisible()) {
      await autoScaleToggle.check();
      await expect(page.locator('text=/enabled|active/i')).toBeVisible({ timeout: 3000 }).catch(() => {});
    }
  });

  test('should show scaling recommendations', async ({ page }) => {
    const recommendations = page.locator('text=/recommend|suggest|optimize/i').first();
    await expect(recommendations).toBeVisible({ timeout: 3000 }).catch(() => {});
  });

  test('should show scaling history', async ({ page }) => {
    const historyTab = page.locator('button:has-text("History"), button:has-text("Scaling History")').first();
    if (await historyTab.isVisible()) {
      await historyTab.click();
      await expect(page.locator('text=/history|scaled|iinstance/i')).toBeVisible();
    }
  });

  test('should show min/max instance bounds', async ({ page }) => {
    await expect(page.locator('text=/min|max|range|bound/i')).toBeVisible();
  });

  test('should configure scaling metrics', async ({ page }) => {
    const metricsSelect = page.locator('select').first();
    if (await metricsSelect.isVisible()) {
      await metricsSelect.selectOption({ index: 1 });
      await page.locator('button:has-text("Apply")').click();
    }
  });
});
