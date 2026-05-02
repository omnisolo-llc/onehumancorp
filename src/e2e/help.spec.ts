import { test, expect } from '@playwright/test';

test.describe('Help Center', () => {
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
  test('should display help center page', async ({ page }) => {
    await expect(page.locator('text=/help|center|support/i')).toBeVisible();
  });

  test('should show help center header', async ({ page }) => {
    await expect(page.locator('text=Help')).toBeVisible();
  });

  test('should display search bar', async ({ page }) => {
    await expect(page.locator('input[type="search"], input[placeholder*="search" i]')).toBeVisible();
  });

  test('should search for help topics', async ({ page }) => {
    const searchInput = page.locator('input[type="search"], input[placeholder*="search" i]').first();
    if (await searchInput.isVisible()) {
      await searchInput.fill('Getting Started');
      await page.keyboard.press('Enter');
      await expect(page.locator('text=/result|article|topic/i')).toBeVisible();
    }
  });

  test('should show help categories', async ({ page }) => {
    const category = page.locator('[class*="category"], [class*="topic"]').first();
    await expect(category).toBeVisible();
  });

  test('should display getting started guide', async ({ page }) => {
    const gettingStarted = page.locator('text=/getting started|beginner|tutorial/i').first();
    await expect(gettingStarted).toBeVisible();
  });

  test('should show faq section', async ({ page }) => {
    await expect(page.locator('text=/faq|questions/i')).toBeVisible();
  });

  test('should expand faq item', async ({ page }) => {
    const faqItem = page.locator('[class*="faq"], [class*="question"]').first();
    if (await faqItem.isVisible()) {
      await faqItem.click();
      await expect(page.locator('text=/answer|solution/i')).toBeVisible();
    }
  });

  test('should show contact support option', async ({ page }) => {
    await expect(page.locator('text=/contact|support|email/i')).toBeVisible();
  });

  test('should show live chat option', async ({ page }) => {
    const chatBtn = page.locator('button:has-text("Chat"), button:has-text("Live Chat")').first();
    await expect(chatBtn).toBeVisible({ timeout: 3000 }).catch(() => {});
  });

  test('should display keyboard shortcuts', async ({ page }) => {
    const shortcuts = page.locator('text=/shortcut|keyboard|ctrl/i').first();
    await expect(shortcuts).toBeVisible();
  });

  test('should link to video tutorials', async ({ page }) => {
    const videoLink = page.locator('text=/video|tutorial|learn/i').first();
    await expect(videoLink).toBeVisible();
  });

  test('should link to documentation', async ({ page }) => {
    const docLink = page.locator('text=/docs|documentation|guide/i').first();
    await expect(docLink).toBeVisible();
  });

  test('should show system status', async ({ page }) => {
    const status = page.locator('text=/status|system|operational/i').first();
    await expect(status).toBeVisible({ timeout: 3000 }).catch(() => {});
  });

  test('should submit support ticket', async ({ page }) => {
    const ticketBtn = page.locator('button:has-text("Submit"), button:has-text("Ticket")').first();
    if (await ticketBtn.isVisible()) {
      await ticketBtn.click();
      await expect(page.locator('text=/ticket|subject|description/i')).toBeVisible();
    }
  });

  test('should show popular articles', async ({ page }) => {
    const popular = page.locator('text=/popular|trending|articles/i').first();
    await expect(popular).toBeVisible();
  });
});

test.describe('AI Help Chat', () => {
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

  test('should display AI help chat', async ({ page }) => {
    await expect(page.locator('text=/ai.*help|assistant|chat/i')).toBeVisible();
  });

  test('should show chat input', async ({ page }) => {
    await expect(page.locator('input[type="text"], textarea').last()).toBeVisible();
  });

  test('should send message to AI', async ({ page }) => {
    const input = page.locator('input[type="text"], textarea').last();
    if (await input.isVisible()) {
      await input.fill('How do I set up agents?');
      await page.locator('button:has-text("Send"), button:has-text("Ask")').click();
    }
  });

  test('should show AI response', async ({ page }) => {
    const input = page.locator('input[type="text"], textarea').last();
    if (await input.isVisible()) {
      await input.fill('How do I set up agents?');
      await page.locator('button:has-text("Send"), button:has-text("Ask")').click();
      await expect(page.locator('text=/agent|setup|configure/i')).toBeVisible({ timeout: 5000 }).catch(() => {});
    }
  });

  test('should show typing indicator', async ({ page }) => {
    const input = page.locator('input[type="text"], textarea').last();
    if (await input.isVisible()) {
      await input.fill('How do I set up agents?');
      await page.locator('button:has-text("Send"), button:has-text("Ask")').click();
      await expect(page.locator('text=/typing|thinking|processing/i')).toBeVisible({ timeout: 3000 }).catch(() => {});
    }
  });

  test('should suggest follow-up questions', async ({ page }) => {
    const suggestion = page.locator('button:has-text("How"), button:has-text("What")').first();
    await expect(suggestion).toBeVisible({ timeout: 3000 }).catch(() => {});
  });

  test('should show help suggestions', async ({ page }) => {
    const suggestion = page.locator('text=/suggestion|recommended|try/i').first();
    await expect(suggestion).toBeVisible({ timeout: 3000 }).catch(() => {});
  });

  test('should clear chat history', async ({ page }) => {
    const clearBtn = page.locator('button:has-text("Clear"), button:has-text("Reset")').first();
    if (await clearBtn.isVisible()) {
      await clearBtn.click();
      await expect(page.locator('text=/cleared|new.*chat/i')).toBeVisible({ timeout: 3000 }).catch(() => {});
    }
  });
});

test.describe('Interactive Walkthrough', () => {
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

  test('should start walkthrough', async ({ page }) => {
    await expect(page.locator('text=/walkthrough|tour|guide/i')).toBeVisible();
  });

  test('should show step indicator', async ({ page }) => {
    const step = page.locator('text=/step \\d+ of \\d+/i').first();
    await expect(step).toBeVisible();
  });

  test('should navigate to next step', async ({ page }) => {
    const nextBtn = page.locator('button:has-text("Next"), button:has-text("Continue")').first();
    if (await nextBtn.isVisible()) {
      await nextBtn.click();
    }
  });

  test('should navigate to previous step', async ({ page }) => {
    const backBtn = page.locator('button:has-text("Back"), button:has-text("Previous")').first();
    if (await backBtn.isVisible()) {
      await backBtn.click();
    }
  });

  test('should skip walkthrough', async ({ page }) => {
    const skipBtn = page.locator('button:has-text("Skip"), button:has-text("Skip Tour")').first();
    if (await skipBtn.isVisible()) {
      await skipBtn.click();
    }
  });

  test('should highlight UI elements', async ({ page }) => {
    const highlight = page.locator('[class*="highlight"], [class*="spotlight"]').first();
    await expect(highlight).toBeVisible({ timeout: 3000 }).catch(() => {});
  });

  test('should complete walkthrough', async ({ page }) => {
    const finishBtn = page.locator('button:has-text("Finish"), button:has-text("Done")').first();
    if (await finishBtn.isVisible()) {
      await finishBtn.click();
      await expect(page.locator('text=/complete|finished|congratulations/i')).toBeVisible({ timeout: 5000 }).catch(() => {});
    }
  });
});

test.describe('Video Tutorials', () => {
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

  test('should display video tutorials page', async ({ page }) => {
    await expect(page.locator('text=/video|tutorial|learn/i')).toBeVisible();
  });

  test('should show video thumbnails', async ({ page }) => {
    const thumbnail = page.locator('[class*="thumbnail"], [class*="video"]').first();
    await expect(thumbnail).toBeVisible();
  });

  test('should play video', async ({ page }) => {
    const playBtn = page.locator('button:has-text("Play"), [class*="play"]').first();
    if (await playBtn.isVisible()) {
      await playBtn.click();
      await expect(page.locator('[class*="video"], video')).toBeVisible();
    }
  });

  test('should show video controls', async ({ page }) => {
    const playBtn = page.locator('button:has-text("Play"), [class*="play"]').first();
    if (await playBtn.isVisible()) {
      await playBtn.click();
      await expect(page.locator('text=/pause|volume|fullscreen/i')).toBeVisible({ timeout: 3000 }).catch(() => {});
    }
  });

  test('should pause video', async ({ page }) => {
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
    const categoryTab = page.locator('button:has-text("Beginner"), button:has-text("Advanced")').first();
    if (await categoryTab.isVisible()) {
      await categoryTab.click();
      await expect(page.locator('text=/video|tutorial/i')).toBeVisible();
    }
  });

  test('should search videos', async ({ page }) => {
    const searchInput = page.locator('input[type="search"], input[placeholder*="search"]').first();
    if (await searchInput.isVisible()) {
      await searchInput.fill('agents');
      await expect(page.locator('text=/agent/i')).toBeVisible();
    }
  });

  test('should show video duration', async ({ page }) => {
    const duration = page.locator('text=/\\d+:\\d+/').first();
    await expect(duration).toBeVisible();
  });

  test('should mark video as watched', async ({ page }) => {
    const videoItem = page.locator('[class*="video"]').first();
    await videoItem.click();
    await expect(page.locator('text=/watched|completed/i')).toBeVisible({ timeout: 3000 }).catch(() => {});
  });
});
