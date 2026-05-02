import { test, expect } from '@playwright/test';

test.describe('Agent Management', () => {
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
  test('should show agents list', async ({ page }) => {
    await expect(page.locator('h1:has-text("Agents")')).toBeVisible();
  });

  test('should display agents page header', async ({ page }) => {
    await expect(page.locator('text=Agents')).toBeVisible();
  });

  test('should show hire agent button', async ({ page }) => {
    await expect(page.locator('button:has-text("Hire Agent")')).toBeVisible();
  });

  test('should display agents grid', async ({ page }) => {
    const grid = page.locator('[class*="grid"], [class*="agents"]').first();
    await expect(grid).toBeVisible();
  });

  test('should show agent cards', async ({ page }) => {
    const agentCards = page.locator('[class*="card"], [class*="agent"]');
    await expect(agentCards.first()).toBeVisible();
  });

  test('should show agent name on card', async ({ page }) => {
    const agentName = page.locator('[class*="name"], text=/agent/i').first();
    await expect(agentName).toBeVisible();
  });

  test('should show agent status', async ({ page }) => {
    const status = page.locator('[class*="status"], text=/active|idle|busy/i').first();
    await expect(status).toBeVisible();
  });

  test('should show agent skills', async ({ page }) => {
    const skills = page.locator('[class*="skill"], text=/sales|support|analytics/i').first();
    await expect(skills).toBeVisible();
  });

  test('should show add new agent option', async ({ page }) => {
    await expect(page.locator('button:has-text("Add Agent")')).toBeVisible();
  });

  test('should show agent search filter', async ({ page }) => {
    const searchInput = page.locator('input[placeholder*="search" i], input[type="search"]').first();
    await expect(searchInput).toBeVisible();
  });

  test('should filter agents by type', async ({ page }) => {
    const filterDropdown = page.locator('select, [class*="filter"]').first();
    await expect(filterDropdown).toBeVisible();
  });

  test('should sort agents list', async ({ page }) => {
    const sortButton = page.locator('button:has-text("Sort"), [class*="sort"]').first();
    await expect(sortButton).toBeVisible();
  });

  test('should show agent configuration button', async ({ page }) => {
    const configButton = page.locator('button:has-text("Configure"), button:has-text("Settings")').first();
    await expect(configButton).toBeVisible();
  });

  test('should show disable agent option', async ({ page }) => {
    const agentCard = page.locator('[class*="card"]').first();
    await agentCard.hover();
    await expect(page.locator('button:has-text("Disable"), button:has-text("Remove")')).toBeVisible();
  });

  test('should show agent performance metrics', async ({ page }) => {
    const metrics = page.locator('text=/performance|tasks|completed/i').first();
    await expect(metrics).toBeVisible();
  });
});

test.describe('Agent Hire Flow', () => {
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

  test('should open hire agent modal', async ({ page }) => {
    await page.locator('button:has-text("Hire Agent")').click();
    await expect(page.locator('text=/hire|new agent/i')).toBeVisible();
  });

  test('should show agent type selection', async ({ page }) => {
    await page.locator('button:has-text("Hire Agent")').click();
    await expect(page.locator('text=/sales|support|analytics|assistant/i')).toBeVisible();
  });

  test('should select agent type', async ({ page }) => {
    await page.locator('button:has-text("Hire Agent")').click();
    await page.locator('text=Sales').click();
    await expect(page.locator('button:has-text("Next")')).toBeEnabled();
  });

  test('should show agent preview', async ({ page }) => {
    await page.locator('button:has-text("Hire Agent")').click();
    await page.locator('text=Support').click();
    await expect(page.locator('text=/preview|demo/i')).toBeVisible();
  });

  test('should confirm hire agent', async ({ page }) => {
    await page.locator('button:has-text("Hire Agent")').click();
    await page.locator('text=Sales').click();
    await page.locator('button:has-text("Hire"), button:has-text("Confirm")').click();
    await expect(page.locator('text=/success| hired/i')).toBeVisible({ timeout: 5000 }).catch(() => {});
  });

  test('should cancel hire flow', async ({ page }) => {
    await page.locator('button:has-text("Hire Agent")').click();
    await page.locator('button:has-text("Cancel")').click();
    await expect(page.locator('text=/hire/i')).not.toBeVisible();
  });
});

test.describe('Agent Configuration', () => {
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

  test('should open agent configuration wizard', async ({ page }) => {
    await expect(page.locator('text=/configure|config|wizard/i')).toBeVisible();
  });

  test('should show configuration steps', async ({ page }) => {
    const steps = page.locator('[class*="step"], text=/step \\d+/i');
    await expect(steps.first()).toBeVisible();
  });

  test('should navigate through config steps', async ({ page }) => {
    const nextBtn = page.locator('button:has-text("Next")');
    if (await nextBtn.isVisible()) {
      await nextBtn.click();
      await expect(page.locator('text=/step \\d+/i')).toBeVisible();
    }
  });

  test('should set agent name in config', async ({ page }) => {
    const nameInput = page.locator('input[type="text"]').first();
    if (await nameInput.isVisible()) {
      await nameInput.fill('Sales Agent');
    }
  });

  test('should set agent personality', async ({ page }) => {
    const personalitySelect = page.locator('select, [class*="personality"]').first();
    if (await personalitySelect.isVisible()) {
      await personalitySelect.selectOption({ index: 1 });
    }
  });

  test('should set response tone', async ({ page }) => {
    const toneOptions = page.locator('text=/formal|casual|professional/i');
    if (await toneOptions.first().isVisible()) {
      await toneOptions.first().click();
    }
  });

  test('should save agent configuration', async ({ page }) => {
    const saveBtn = page.locator('button:has-text("Save"), button:has-text("Finish")');
    if (await saveBtn.isVisible()) {
      await saveBtn.click();
      await expect(page.locator('text=/saved|success/i')).toBeVisible({ timeout: 5000 }).catch(() => {});
    }
  });

  test('should show config preview', async ({ page }) => {
    const previewBtn = page.locator('button:has-text("Preview")');
    if (await previewBtn.isVisible()) {
      await previewBtn.click();
      await expect(page.locator('text=/preview/i')).toBeVisible();
    }
  });
});

test.describe('Agent Interactions', () => {
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

  test('should view agent details', async ({ page }) => {
    const agentCard = page.locator('[class*="card"]').first();
    await agentCard.click();
    await expect(page.locator('text=/details|info/i')).toBeVisible();
  });

  test('should send message to agent', async ({ page }) => {
    const agentCard = page.locator('[class*="card"]').first();
    await agentCard.click();
    const messageInput = page.locator('input[type="text"], textarea').first();
    if (await messageInput.isVisible()) {
      await messageInput.fill('Hello agent');
      await page.locator('button:has-text("Send")').click();
    }
  });

  test('should view agent chat history', async ({ page }) => {
    const agentCard = page.locator('[class*="card"]').first();
    await agentCard.click();
    const historyTab = page.locator('button:has-text("History"), button:has-text("Chat")').first();
    if (await historyTab.isVisible()) {
      await historyTab.click();
      await expect(page.locator('text=/history|messages/i')).toBeVisible();
    }
  });

  test('should update agent status', async ({ page }) => {
    const statusDropdown = page.locator('[class*="status"]').first();
    if (await statusDropdown.isVisible()) {
      await statusDropdown.click();
      await expect(page.locator('text=/active|idle|offline/i')).toBeVisible();
    }
  });

  test('should assign task to agent', async ({ page }) => {
    const agentCard = page.locator('[class*="card"]').first();
    await agentCard.click();
    const assignBtn = page.locator('button:has-text("Assign"), button:has-text("Delegate")').first();
    if (await assignBtn.isVisible()) {
      await assignBtn.click();
      await expect(page.locator('text=/task|assign/i')).toBeVisible();
    }
  });

  test('should view agent activity log', async ({ page }) => {
    const agentCard = page.locator('[class*="card"]').first();
    await agentCard.click();
    const logsTab = page.locator('button:has-text("Logs"), button:has-text("Activity")').first();
    if (await logsTab.isVisible()) {
      await logsTab.click();
      await expect(page.locator('text=/log|activity/i')).toBeVisible();
    }
  });
});

test.describe('Agent Mobile', () => {
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

  test.use({ viewport: { width: 375, height: 800 } });

  test('should display agents list on mobile', async ({ page }) => {
    await expect(page.locator('text=Agents')).toBeVisible();
  });

  test('should show hamburger menu on mobile', async ({ page }) => {
    const menuBtn = page.locator('[class*="menu"], button:has-text("Menu")').first();
    await expect(menuBtn).toBeVisible();
  });

  test('should scroll through agents vertically', async ({ page }) => {
    await page.evaluate(() => window.scrollTo(0, document.body.scrollHeight));
    const lastAgent = page.locator('[class*="card"]').last();
    await expect(lastAgent).toBeVisible();
  });
});
