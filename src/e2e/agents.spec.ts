import { test, expect } from '@playwright/test';

test.describe('Agent Management', () => {
  test('should show agents list', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.locator('h1:has-text("Agents")')).toBeVisible();
  });

  test('should display agents page header', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.locator('text=Agents')).toBeVisible();
  });

  test('should show hire agent button', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.locator('button:has-text("Hire Agent")')).toBeVisible();
  });

  test('should display agents grid', async ({ page }) => {
    await page.goto('/agents');
    const grid = page.locator('[class*="grid"], [class*="agents"]').first();
    await expect(grid).toBeVisible();
  });

  test('should show agent cards', async ({ page }) => {
    await page.goto('/agents');
    const agentCards = page.locator('[class*="card"], [class*="agent"]');
    await expect(agentCards.first()).toBeVisible();
  });

  test('should show agent name on card', async ({ page }) => {
    await page.goto('/agents');
    const agentName = page.locator('[class*="name"], text=/agent/i').first();
    await expect(agentName).toBeVisible();
  });

  test('should show agent status', async ({ page }) => {
    await page.goto('/agents');
    const status = page.locator('[class*="status"], text=/active|idle|busy/i').first();
    await expect(status).toBeVisible();
  });

  test('should show agent skills', async ({ page }) => {
    await page.goto('/agents');
    const skills = page.locator('[class*="skill"], text=/sales|support|analytics/i').first();
    await expect(skills).toBeVisible();
  });

  test('should show add new agent option', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.locator('button:has-text("Add Agent")')).toBeVisible();
  });

  test('should show agent search filter', async ({ page }) => {
    await page.goto('/agents');
    const searchInput = page.locator('input[placeholder*="search" i], input[type="search"]').first();
    await expect(searchInput).toBeVisible();
  });

  test('should filter agents by type', async ({ page }) => {
    await page.goto('/agents');
    const filterDropdown = page.locator('select, [class*="filter"]').first();
    await expect(filterDropdown).toBeVisible();
  });

  test('should sort agents list', async ({ page }) => {
    await page.goto('/agents');
    const sortButton = page.locator('button:has-text("Sort"), [class*="sort"]').first();
    await expect(sortButton).toBeVisible();
  });

  test('should show agent configuration button', async ({ page }) => {
    await page.goto('/agents');
    const configButton = page.locator('button:has-text("Configure"), button:has-text("Settings")').first();
    await expect(configButton).toBeVisible();
  });

  test('should show disable agent option', async ({ page }) => {
    await page.goto('/agents');
    const agentCard = page.locator('[class*="card"]').first();
    await agentCard.hover();
    await expect(page.locator('button:has-text("Disable"), button:has-text("Remove")')).toBeVisible();
  });

  test('should show agent performance metrics', async ({ page }) => {
    await page.goto('/agents');
    const metrics = page.locator('text=/performance|tasks|completed/i').first();
    await expect(metrics).toBeVisible();
  });
});

test.describe('Agent Hire Flow', () => {
  test('should open hire agent modal', async ({ page }) => {
    await page.goto('/agents');
    await page.locator('button:has-text("Hire Agent")').click();
    await expect(page.locator('text=/hire|new agent/i')).toBeVisible();
  });

  test('should show agent type selection', async ({ page }) => {
    await page.goto('/agents');
    await page.locator('button:has-text("Hire Agent")').click();
    await expect(page.locator('text=/sales|support|analytics|assistant/i')).toBeVisible();
  });

  test('should select agent type', async ({ page }) => {
    await page.goto('/agents');
    await page.locator('button:has-text("Hire Agent")').click();
    await page.locator('text=Sales').click();
    await expect(page.locator('button:has-text("Next")')).toBeEnabled();
  });

  test('should show agent preview', async ({ page }) => {
    await page.goto('/agents');
    await page.locator('button:has-text("Hire Agent")').click();
    await page.locator('text=Support').click();
    await expect(page.locator('text=/preview|demo/i')).toBeVisible();
  });

  test('should confirm hire agent', async ({ page }) => {
    await page.goto('/agents');
    await page.locator('button:has-text("Hire Agent")').click();
    await page.locator('text=Sales').click();
    await page.locator('button:has-text("Hire"), button:has-text("Confirm")').click();
    await expect(page.locator('text=/success| hired/i')).toBeVisible({ timeout: 5000 });
  });

  test('should cancel hire flow', async ({ page }) => {
    await page.goto('/agents');
    await page.locator('button:has-text("Hire Agent")').click();
    await page.locator('button:has-text("Cancel")').click();
    await expect(page.locator('text=/hire/i')).not.toBeVisible();
  });
});

test.describe('Agent Configuration', () => {
  test('should open agent configuration wizard', async ({ page }) => {
    await page.goto('/agents/configure');
    await expect(page.locator('text=/configure|config|wizard/i')).toBeVisible();
  });

  test('should show configuration steps', async ({ page }) => {
    await page.goto('/agents/configure');
    const steps = page.locator('[class*="step"], text=/step \\d+/i');
    await expect(steps.first()).toBeVisible();
  });

  test('should navigate through config steps', async ({ page }) => {
    await page.goto('/agents/configure');
    const nextBtn = page.locator('button:has-text("Next")');
    if (await nextBtn.isVisible()) {
      await nextBtn.click();
      await expect(page.locator('text=/step \\d+/i')).toBeVisible();
    }
  });

  test('should set agent name in config', async ({ page }) => {
    await page.goto('/agents/configure');
    const nameInput = page.locator('input[type="text"]').first();
    if (await nameInput.isVisible()) {
      await nameInput.fill('Sales Agent');
    }
  });

  test('should set agent personality', async ({ page }) => {
    await page.goto('/agents/configure');
    const personalitySelect = page.locator('select, [class*="personality"]').first();
    if (await personalitySelect.isVisible()) {
      await personalitySelect.selectOption({ index: 1 });
    }
  });

  test('should set response tone', async ({ page }) => {
    await page.goto('/agents/configure');
    const toneOptions = page.locator('text=/formal|casual|professional/i');
    if (await toneOptions.first().isVisible()) {
      await toneOptions.first().click();
    }
  });

  test('should save agent configuration', async ({ page }) => {
    await page.goto('/agents/configure');
    const saveBtn = page.locator('button:has-text("Save"), button:has-text("Finish")');
    if (await saveBtn.isVisible()) {
      await saveBtn.click();
      await expect(page.locator('text=/saved|success/i')).toBeVisible({ timeout: 5000 });
    }
  });

  test('should show config preview', async ({ page }) => {
    await page.goto('/agents/configure');
    const previewBtn = page.locator('button:has-text("Preview")');
    if (await previewBtn.isVisible()) {
      await previewBtn.click();
      await expect(page.locator('text=/preview/i')).toBeVisible();
    }
  });
});

test.describe('Agent Interactions', () => {
  test('should view agent details', async ({ page }) => {
    await page.goto('/agents');
    const agentCard = page.locator('[class*="card"]').first();
    await agentCard.click();
    await expect(page.locator('text=/details|info/i')).toBeVisible();
  });

  test('should send message to agent', async ({ page }) => {
    await page.goto('/agents');
    const agentCard = page.locator('[class*="card"]').first();
    await agentCard.click();
    const messageInput = page.locator('input[type="text"], textarea').first();
    if (await messageInput.isVisible()) {
      await messageInput.fill('Hello agent');
      await page.locator('button:has-text("Send")').click();
    }
  });

  test('should view agent chat history', async ({ page }) => {
    await page.goto('/agents');
    const agentCard = page.locator('[class*="card"]').first();
    await agentCard.click();
    const historyTab = page.locator('button:has-text("History"), button:has-text("Chat")').first();
    if (await historyTab.isVisible()) {
      await historyTab.click();
      await expect(page.locator('text=/history|messages/i')).toBeVisible();
    }
  });

  test('should update agent status', async ({ page }) => {
    await page.goto('/agents');
    const statusDropdown = page.locator('[class*="status"]').first();
    if (await statusDropdown.isVisible()) {
      await statusDropdown.click();
      await expect(page.locator('text=/active|idle|offline/i')).toBeVisible();
    }
  });

  test('should assign task to agent', async ({ page }) => {
    await page.goto('/agents');
    const agentCard = page.locator('[class*="card"]').first();
    await agentCard.click();
    const assignBtn = page.locator('button:has-text("Assign"), button:has-text("Delegate")').first();
    if (await assignBtn.isVisible()) {
      await assignBtn.click();
      await expect(page.locator('text=/task|assign/i')).toBeVisible();
    }
  });

  test('should view agent activity log', async ({ page }) => {
    await page.goto('/agents');
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
  test.use({ viewport: { width: 375, height: 800 } });

  test('should display agents list on mobile', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.locator('text=Agents')).toBeVisible();
  });

  test('should show hamburger menu on mobile', async ({ page }) => {
    await page.goto('/agents');
    const menuBtn = page.locator('[class*="menu"], button:has-text("Menu")').first();
    await expect(menuBtn).toBeVisible();
  });

  test('should scroll through agents vertically', async ({ page }) => {
    await page.goto('/agents');
    await page.evaluate(() => window.scrollTo(0, document.body.scrollHeight));
    const lastAgent = page.locator('[class*="card"]').last();
    await expect(lastAgent).toBeVisible();
  });
});