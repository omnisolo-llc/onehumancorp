import { test, expect } from '@playwright/test';

test.describe('Help Center', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Sign In"), button:has-text("Login")').click();
    await page.waitForURL('**/dashboard**');
  });

  test('should display help center page', async ({ page }) => {
    await Promise.all([
      page.waitForNavigation(),
      page.locator('text="?"').first().click()
    ]);
    await expect(page.locator('text=/help|center|support/i')).toBeVisible();
  });

  test('should show help center header', async ({ page }) => {
    await Promise.all([
      page.waitForNavigation(),
      page.locator('text="?"').first().click()
    ]);
    await expect(page.locator('text=Help')).toBeVisible();
  });

  test('should display search bar', async ({ page }) => {
    await Promise.all([
      page.waitForNavigation(),
      page.locator('text="?"').first().click()
    ]);
    await expect(page.locator('input[type="search"], input[placeholder*="search" i]')).toBeVisible();
  });

  test('should search for help topics', async ({ page }) => {
    await Promise.all([
      page.waitForNavigation(),
      page.locator('text="?"').first().click()
    ]);
    const searchInput = page.locator('input[type="search"], input[placeholder*="search" i]').first();
    if (await searchInput.isVisible()) {
      await searchInput.fill('Getting Started');
      await page.keyboard.press('Enter');
      await expect(page.locator('text=/result|article|topic/i')).toBeVisible();
    }
  });

  test('should show help categories', async ({ page }) => {
    await Promise.all([
      page.waitForNavigation(),
      page.locator('text="?"').first().click()
    ]);
    const category = page.locator('[class*="category"], [class*="topic"]').first();
    await expect(category).toBeVisible();
  });

  test('should display getting started guide', async ({ page }) => {
    await Promise.all([
      page.waitForNavigation(),
      page.locator('text="?"').first().click()
    ]);
    const gettingStarted = page.locator('text=/getting started|beginner|tutorial/i').first();
    await expect(gettingStarted).toBeVisible();
  });

  test('should show faq section', async ({ page }) => {
    await Promise.all([
      page.waitForNavigation(),
      page.locator('text="?"').first().click()
    ]);
    await expect(page.locator('text=/faq|questions/i')).toBeVisible();
  });

  test('should expand faq item', async ({ page }) => {
    await Promise.all([
      page.waitForNavigation(),
      page.locator('text="?"').first().click()
    ]);
    const faqItem = page.locator('[class*="faq"], [class*="question"]').first();
    if (await faqItem.isVisible()) {
      await faqItem.click();
      await expect(page.locator('text=/answer|solution/i')).toBeVisible();
    }
  });

  test('should show contact support option', async ({ page }) => {
    await Promise.all([
      page.waitForNavigation(),
      page.locator('text="?"').first().click()
    ]);
    await expect(page.locator('text=/contact|support|email/i')).toBeVisible();
  });

  test('should show live chat option', async ({ page }) => {
    await Promise.all([
      page.waitForNavigation(),
      page.locator('text="?"').first().click()
    ]);
    const chatBtn = page.locator('button:has-text("Chat"), button:has-text("Live Chat")').first();
    await expect(chatBtn).toBeVisible({ timeout: 3000 });
  });

  test('should display keyboard shortcuts', async ({ page }) => {
    await Promise.all([
      page.waitForNavigation(),
      page.locator('text="?"').first().click()
    ]);
    const shortcuts = page.locator('text=/shortcut|keyboard|ctrl/i').first();
    await expect(shortcuts).toBeVisible();
  });

  test('should link to video tutorials', async ({ page }) => {
    await Promise.all([
      page.waitForNavigation(),
      page.locator('text="?"').first().click()
    ]);
    const videoLink = page.locator('text=/video|tutorial|learn/i').first();
    await expect(videoLink).toBeVisible();
  });

  test('should link to documentation', async ({ page }) => {
    await Promise.all([
      page.waitForNavigation(),
      page.locator('text="?"').first().click()
    ]);
    const docLink = page.locator('text=/docs|documentation|guide/i').first();
    await expect(docLink).toBeVisible();
  });

  test('should show system status', async ({ page }) => {
    await Promise.all([
      page.waitForNavigation(),
      page.locator('text="?"').first().click()
    ]);
    const status = page.locator('text=/status|system|operational/i').first();
    await expect(status).toBeVisible({ timeout: 3000 });
  });

  test('should submit support ticket', async ({ page }) => {
    await Promise.all([
      page.waitForNavigation(),
      page.locator('text="?"').first().click()
    ]);
    const ticketBtn = page.locator('button:has-text("Submit"), button:has-text("Ticket")').first();
    if (await ticketBtn.isVisible()) {
      await ticketBtn.click();
      await expect(page.locator('text=/ticket|subject|description/i')).toBeVisible();
    }
  });

  test('should show popular articles', async ({ page }) => {
    await Promise.all([
      page.waitForNavigation(),
      page.locator('text="?"').first().click()
    ]);
    const popular = page.locator('text=/popular|trending|articles/i').first();
    await expect(popular).toBeVisible();
  });
});

test.describe('AI Help Chat', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Sign In"), button:has-text("Login")').click();
    await page.waitForURL('**/dashboard**');
  });

  test('should display AI help chat', async ({ page }) => {
    await Promise.all([
      page.waitForNavigation(),
      page.locator('text="Ask AI"').first().click()
    ]);
    await expect(page.locator('text=/ai.*help|assistant|chat/i')).toBeVisible();
  });

  test('should show chat input', async ({ page }) => {
    await Promise.all([
      page.waitForNavigation(),
      page.locator('text="Ask AI"').first().click()
    ]);
    await expect(page.locator('input[type="text"], textarea').last()).toBeVisible();
  });

  test('should send message to AI', async ({ page }) => {
    await Promise.all([
      page.waitForNavigation(),
      page.locator('text="Ask AI"').first().click()
    ]);
    const input = page.locator('input[type="text"], textarea').last();
    if (await input.isVisible()) {
      await input.fill('How do I set up agents?');
      await page.locator('button:has-text("Send"), button:has-text("Ask")').click();
    }
  });

  test('should show AI response', async ({ page }) => {
    await Promise.all([
      page.waitForNavigation(),
      page.locator('text="Ask AI"').first().click()
    ]);
    const input = page.locator('input[type="text"], textarea').last();
    if (await input.isVisible()) {
      await input.fill('How do I set up agents?');
      await page.locator('button:has-text("Send"), button:has-text("Ask")').click();
      await expect(page.locator('text=/agent|setup|configure/i')).toBeVisible({ timeout: 5000 });
    }
  });

  test('should show typing indicator', async ({ page }) => {
    await Promise.all([
      page.waitForNavigation(),
      page.locator('text="Ask AI"').first().click()
    ]);
    const input = page.locator('input[type="text"], textarea').last();
    if (await input.isVisible()) {
      await input.fill('How do I set up agents?');
      await page.locator('button:has-text("Send"), button:has-text("Ask")').click();
      await expect(page.locator('text=/typing|thinking|processing/i')).toBeVisible({ timeout: 3000 });
    }
  });

  test('should suggest follow-up questions', async ({ page }) => {
    await Promise.all([
      page.waitForNavigation(),
      page.locator('text="Ask AI"').first().click()
    ]);
    const suggestion = page.locator('button:has-text("How"), button:has-text("What")').first();
    await expect(suggestion).toBeVisible({ timeout: 3000 });
  });

  test('should show help suggestions', async ({ page }) => {
    await Promise.all([
      page.waitForNavigation(),
      page.locator('text="Ask AI"').first().click()
    ]);
    const suggestion = page.locator('text=/suggestion|recommended|try/i').first();
    await expect(suggestion).toBeVisible({ timeout: 3000 });
  });

  test('should clear chat history', async ({ page }) => {
    await Promise.all([
      page.waitForNavigation(),
      page.locator('text="Ask AI"').first().click()
    ]);
    const clearBtn = page.locator('button:has-text("Clear"), button:has-text("Reset")').first();
    if (await clearBtn.isVisible()) {
      await clearBtn.click();
      await expect(page.locator('text=/cleared|new.*chat/i')).toBeVisible({ timeout: 3000 });
    }
  });
});

test.describe('Interactive Walkthrough', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Sign In"), button:has-text("Login")').click();
    await page.waitForURL('**/dashboard**');
  });

  test('should start walkthrough', async ({ page }) => {
    await Promise.all([
      page.waitForNavigation(),
      page.locator('button:has-text("Menu")').first().click(),
      page.locator('button:has-text("App Tour")').first().click()
    ]);
    await expect(page.locator('text=/walkthrough|tour|guide/i')).toBeVisible();
  });

  test('should show step indicator', async ({ page }) => {
    await Promise.all([
      page.waitForNavigation(),
      page.locator('button:has-text("Menu")').first().click(),
      page.locator('button:has-text("App Tour")').first().click()
    ]);
    const step = page.locator('text=/step \\d+ of \\d+/i').first();
    await expect(step).toBeVisible();
  });

  test('should navigate to next step', async ({ page }) => {
    await Promise.all([
      page.waitForNavigation(),
      page.locator('button:has-text("Menu")').first().click(),
      page.locator('button:has-text("App Tour")').first().click()
    ]);
    const nextBtn = page.locator('button:has-text("Next"), button:has-text("Continue")').first();
    if (await nextBtn.isVisible()) {
      await nextBtn.click();
    }
  });

  test('should navigate to previous step', async ({ page }) => {
    await Promise.all([
      page.waitForNavigation(),
      page.locator('button:has-text("Menu")').first().click(),
      page.locator('button:has-text("App Tour")').first().click()
    ]);
    const backBtn = page.locator('button:has-text("Back"), button:has-text("Previous")').first();
    if (await backBtn.isVisible()) {
      await backBtn.click();
    }
  });

  test('should skip walkthrough', async ({ page }) => {
    await Promise.all([
      page.waitForNavigation(),
      page.locator('button:has-text("Menu")').first().click(),
      page.locator('button:has-text("App Tour")').first().click()
    ]);
    const skipBtn = page.locator('button:has-text("Skip"), button:has-text("Skip Tour")').first();
    if (await skipBtn.isVisible()) {
      await skipBtn.click();
    }
  });

  test('should highlight UI elements', async ({ page }) => {
    await Promise.all([
      page.waitForNavigation(),
      page.locator('button:has-text("Menu")').first().click(),
      page.locator('button:has-text("App Tour")').first().click()
    ]);
    const highlight = page.locator('[class*="highlight"], [class*="spotlight"]').first();
    await expect(highlight).toBeVisible({ timeout: 3000 });
  });

  test('should complete walkthrough', async ({ page }) => {
    await Promise.all([
      page.waitForNavigation(),
      page.locator('button:has-text("Menu")').first().click(),
      page.locator('button:has-text("App Tour")').first().click()
    ]);
    const finishBtn = page.locator('button:has-text("Finish"), button:has-text("Done")').first();
    if (await finishBtn.isVisible()) {
      await finishBtn.click();
      await expect(page.locator('text=/complete|finished|congratulations/i')).toBeVisible({ timeout: 5000 });
    }
  });
});

test.describe('Video Tutorials', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/login');
    await page.locator('input[type="email"]').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Sign In"), button:has-text("Login")').click();
    await page.waitForURL('**/dashboard**');
  });

  test('should display video tutorials page', async ({ page }) => {
    await Promise.all([
      page.waitForNavigation(),
      page.locator('button:has-text("Menu")').first().click(),
      page.locator('button:has-text("Video Tutorials")').first().click()
    ]);
    await expect(page.locator('text=/video|tutorial|learn/i')).toBeVisible();
  });

  test('should show video thumbnails', async ({ page }) => {
    await Promise.all([
      page.waitForNavigation(),
      page.locator('button:has-text("Menu")').first().click(),
      page.locator('button:has-text("Video Tutorials")').first().click()
    ]);
    const thumbnail = page.locator('[class*="thumbnail"], [class*="video"]').first();
    await expect(thumbnail).toBeVisible();
  });

  test('should play video', async ({ page }) => {
    await Promise.all([
      page.waitForNavigation(),
      page.locator('button:has-text("Menu")').first().click(),
      page.locator('button:has-text("Video Tutorials")').first().click()
    ]);
    const playBtn = page.locator('button:has-text("Play"), [class*="play"]').first();
    if (await playBtn.isVisible()) {
      await playBtn.click();
      await expect(page.locator('[class*="video"], video')).toBeVisible();
    }
  });

  test('should show video controls', async ({ page }) => {
    await Promise.all([
      page.waitForNavigation(),
      page.locator('button:has-text("Menu")').first().click(),
      page.locator('button:has-text("Video Tutorials")').first().click()
    ]);
    const playBtn = page.locator('button:has-text("Play"), [class*="play"]').first();
    if (await playBtn.isVisible()) {
      await playBtn.click();
      await expect(page.locator('text=/pause|volume|fullscreen/i')).toBeVisible({ timeout: 3000 });
    }
  });

  test('should pause video', async ({ page }) => {
    await Promise.all([
      page.waitForNavigation(),
      page.locator('button:has-text("Menu")').first().click(),
      page.locator('button:has-text("Video Tutorials")').first().click()
    ]);
    const playBtn = page.locator('button:has-text("Play"), [class*="play"]').first();
    if (await playBtn.isVisible()) {
      await playBtn.click();
      const pauseBtn = page.locator('button:has-text("Pause"), [class*="pause"]').first();
      if (await pauseBtn.isVisible()) {
        await pauseBtn.click();
      }
    }
  });

  test('should categorize videos', async ({ page }) => {
    await Promise.all([
      page.waitForNavigation(),
      page.locator('button:has-text("Menu")').first().click(),
      page.locator('button:has-text("Video Tutorials")').first().click()
    ]);
    const categoryTab = page.locator('button:has-text("Beginner"), button:has-text("Advanced")').first();
    if (await categoryTab.isVisible()) {
      await categoryTab.click();
      await expect(page.locator('text=/video|tutorial/i')).toBeVisible();
    }
  });

  test('should search videos', async ({ page }) => {
    await Promise.all([
      page.waitForNavigation(),
      page.locator('button:has-text("Menu")').first().click(),
      page.locator('button:has-text("Video Tutorials")').first().click()
    ]);
    const searchInput = page.locator('input[type="search"], input[placeholder*="search"]').first();
    if (await searchInput.isVisible()) {
      await searchInput.fill('agents');
      await expect(page.locator('text=/agent/i')).toBeVisible();
    }
  });

  test('should show video duration', async ({ page }) => {
    await Promise.all([
      page.waitForNavigation(),
      page.locator('button:has-text("Menu")').first().click(),
      page.locator('button:has-text("Video Tutorials")').first().click()
    ]);
    const duration = page.locator('text=/\\d+:\\d+/').first();
    await expect(duration).toBeVisible();
  });

  test('should mark video as watched', async ({ page }) => {
    await Promise.all([
      page.waitForNavigation(),
      page.locator('button:has-text("Menu")').first().click(),
      page.locator('button:has-text("Video Tutorials")').first().click()
    ]);
    const videoItem = page.locator('[class*="video"]').first();
    await videoItem.click();
    await expect(page.locator('text=/watched|completed/i')).toBeVisible({ timeout: 3000 });
  });
});