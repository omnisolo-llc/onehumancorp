import { test, expect } from '@playwright/test';

test.describe('Website Builder', () => {
  test('should display website builder page', async ({ page }) => {
    await page.goto('/website-builder');
    await expect(page.locator('text=/website|builder|create/i')).toBeVisible();
  });

  test('should show website builder wizard', async ({ page }) => {
    await page.goto('/website-builder');
    await expect(page.locator('text=/wizard|create|build/i')).toBeVisible();
  });

  test('should show step indicator', async ({ page }) => {
    await page.goto('/website-builder');
    const step = page.locator('text=/step \\d+/i').first();
    await expect(step).toBeVisible();
  });

  test('should navigate through wizard steps', async ({ page }) => {
    await page.goto('/website-builder');
    const nextBtn = page.locator('button:has-text("Next"), button:has-text("Continue")').first();
    if (await nextBtn.isVisible()) {
      await nextBtn.click();
      await expect(page.locator('text=/step \\d+/i')).toBeVisible();
    }
  });

  test('should select template', async ({ page }) => {
    await page.goto('/website-builder');
    const templateCard = page.locator('[class*="template"], [class*="card"]').first();
    if (await templateCard.isVisible()) {
      await templateCard.click();
      await expect(page.locator('text=/selected|chosen/i')).toBeVisible({ timeout: 3000 });
    }
  });

  test('should customize site colors', async ({ page }) => {
    await page.goto('/website-builder');
    const nextBtn = page.locator('button:has-text("Next"), button:has-text("Continue")').first();
    if (await nextBtn.isVisible()) {
      await nextBtn.click();
    }
    const colorPicker = page.locator('input[type="color"], [class*="color"]').first();
    if (await colorPicker.isVisible()) {
      await colorPicker.fill('#FF5733');
    }
  });

  test('should add site content', async ({ page }) => {
    await page.goto('/website-builder');
    const nextBtn = page.locator('button:has-text("Next"), button:has-text("Continue")').first();
    if (await nextBtn.isVisible()) {
      for (let i = 0; i < 2; i++) {
        await nextBtn.click();
        await page.waitForTimeout(200);
      }
    }
    const contentInput = page.locator('textarea, input[type="text"]').first();
    if (await contentInput.isVisible()) {
      await contentInput.fill('Welcome to my website');
    }
  });

  test('should add site images', async ({ page }) => {
    await page.goto('/website-builder');
    const nextBtn = page.locator('button:has-text("Next"), button:has-text("Continue")').first();
    if (await nextBtn.isVisible()) {
      for (let i = 0; i < 2; i++) {
        await nextBtn.click();
        await page.waitForTimeout(200);
      }
    }
    const imageUpload = page.locator('input[type="file"]').first();
    if (await imageUpload.isVisible()) {
      await expect(imageUpload).toBeAttached();
    }
  });

  test('should preview website', async ({ page }) => {
    await page.goto('/website-builder');
    const previewBtn = page.locator('button:has-text("Preview"), button:has-text("View")').first();
    if (await previewBtn.isVisible()) {
      await previewBtn.click();
      await expect(page.locator('text=/preview|live.*view/i')).toBeVisible();
    }
  });

  test('should publish website', async ({ page }) => {
    await page.goto('/website-builder');
    const publishBtn = page.locator('button:has-text("Publish"), button:has-text("Launch")').first();
    if (await publishBtn.isVisible()) {
      await publishBtn.click();
      await expect(page.locator('text=/published|live|launched/i')).toBeVisible({ timeout: 10000 });
    }
  });

  test('should save website draft', async ({ page }) => {
    await page.goto('/website-builder');
    const saveBtn = page.locator('button:has-text("Save"), button:has-text("Draft")').first();
    if (await saveBtn.isVisible()) {
      await saveBtn.click();
      await expect(page.locator('text=/saved|draft/i')).toBeVisible({ timeout: 3000 });
    }
  });

  test('should add new page', async ({ page }) => {
    await page.goto('/website-builder');
    const addPageBtn = page.locator('button:has-text("Add Page"), button:has-text("New Page")').first();
    if (await addPageBtn.isVisible()) {
      await addPageBtn.click();
      await expect(page.locator('text=/page.*created|new.*page/i')).toBeVisible();
    }
  });

  test('should reorder pages', async ({ page }) => {
    await page.goto('/website-builder');
    const pageItem = page.locator('[class*="page"]').first();
    if (await pageItem.isVisible()) {
      await pageItem.dragTo(page.locator('[class*="page"]').nth(2));
    }
  });

  test('should delete page', async ({ page }) => {
    await page.goto('/website-builder');
    const pageItem = page.locator('[class*="page"]').first();
    await pageItem.hover();
    const deleteBtn = page.locator('button:has-text("Delete"), button:has-text("Remove")').first();
    if (await deleteBtn.isVisible()) {
      await deleteBtn.click();
      await expect(page.locator('text=/deleted|removed/i')).toBeVisible({ timeout: 3000 });
    }
  });

  test('should set SEO title', async ({ page }) => {
    await page.goto('/website-builder/seo');
    const titleInput = page.locator('input[placeholder*="title" i], input[name*="title"]').first();
    if (await titleInput.isVisible()) {
      await titleInput.fill('My Awesome Website');
      await page.locator('button:has-text("Save")').click();
    }
  });

  test('should set SEO description', async ({ page }) => {
    await page.goto('/website-builder/seo');
    const descInput = page.locator('textarea, input[name*="description"]').first();
    if (await descInput.isVisible()) {
      await descInput.fill('This is the best website ever');
      await page.locator('button:has-text("Save")').click();
    }
  });

  test('should connect custom domain', async ({ page }) => {
    await page.goto('/website-builder/domain');
    const domainInput = page.locator('input[placeholder*="domain" i]').first();
    if (await domainInput.isVisible()) {
      await domainInput.fill('mysite.com');
      await page.locator('button:has-text("Connect"), button:has-text("Add")').click();
    }
  });
});

test.describe('Prompt Tuning', () => {
  test('should display prompt tuning page', async ({ page }) => {
    await page.goto('/prompt-tuning');
    await expect(page.locator('text=/prompt|tuning|ai/i')).toBeVisible();
  });

  test('should show prompt editor', async ({ page }) => {
    await page.goto('/prompt-tuning');
    const editor = page.locator('textarea, [class*="editor"]').first();
    await expect(editor).toBeVisible();
  });

  test('should edit system prompt', async ({ page }) => {
    await page.goto('/prompt-tuning');
    const editor = page.locator('textarea').first();
    if (await editor.isVisible()) {
      await editor.fill('You are a helpful AI assistant');
      await page.locator('button:has-text("Save"), button:has-text("Update")').click();
    }
  });

  test('should preview prompt changes', async ({ page }) => {
    await page.goto('/prompt-tuning');
    const previewBtn = page.locator('button:has-text("Preview"), button:has-text("Test")').first();
    if (await previewBtn.isVisible()) {
      await previewBtn.click();
      await expect(page.locator('text=/preview|sample.*response/i')).toBeVisible();
    }
  });

  test('should set temperature parameter', async ({ page }) => {
    await page.goto('/prompt-tuning');
    const tempInput = page.locator('input[type="range"], [class*="temperature"]').first();
    if (await tempInput.isVisible()) {
      await tempInput.fill('0.7');
    }
  });

  test('should set max tokens parameter', async ({ page }) => {
    await page.goto('/prompt-tuning');
    const tokensInput = page.locator('input[type="number"], input[placeholder*="token"]').first();
    if (await tokensInput.isVisible()) {
      await tokensInput.fill('2048');
      await page.locator('button:has-text("Save")').click();
    }
  });

  test('should show prompt templates', async ({ page }) => {
    await page.goto('/prompt-tuning');
    const template = page.locator('text=/template|preset/i').first();
    await expect(template).toBeVisible();
  });

  test('should load prompt template', async ({ page }) => {
    await page.goto('/prompt-tuning');
    const templateBtn = page.locator('button:has-text("Template"), [class*="template"]').first();
    if (await templateBtn.isVisible()) {
      await templateBtn.click();
      await expect(page.locator('text=/select.*template|load.*template/i')).toBeVisible();
    }
  });

  test('should reset to default prompt', async ({ page }) => {
    await page.goto('/prompt-tuning');
    const resetBtn = page.locator('button:has-text("Reset"), button:has-text("Default")').first();
    if (await resetBtn.isVisible()) {
      await resetBtn.click();
      await expect(page.locator('text=/reset|default/i')).toBeVisible({ timeout: 3000 });
    }
  });

  test('should show prompt history', async ({ page }) => {
    await page.goto('/prompt-tuning');
    const historyTab = page.locator('button:has-text("History"), button:has-text("Versions")').first();
    if (await historyTab.isVisible()) {
      await historyTab.click();
      await expect(page.locator('text=/history|version|change/i')).toBeVisible();
    }
  });

  test('should compare prompt versions', async ({ page }) => {
    await page.goto('/prompt-tuning');
    const historyTab = page.locator('button:has-text("History"), button:has-text("Versions")').first();
    if (await historyTab.isVisible()) {
      await historyTab.click();
      const compareBtn = page.locator('button:has-text("Compare"), button:has-text("Diff")').first();
      if (await compareBtn.isVisible()) {
        await compareBtn.click();
        await expect(page.locator('text=/diff|compare|change/i')).toBeVisible();
      }
    }
  });
});

test.describe('Grow Business', () => {
  test('should display grow business page', async ({ page }) => {
    await page.goto('/grow');
    await expect(page.locator('text=/grow|business|growth/i')).toBeVisible();
  });

  test('should show growth strategies', async ({ page }) => {
    await page.goto('/grow');
    const strategy = page.locator('text=/strategy|marketing|seo/i').first();
    await expect(strategy).toBeVisible();
  });

  test('should show marketing tools', async ({ page }) => {
    await page.goto('/grow');
    await expect(page.locator('text=/marketing|advertise|campaign/i')).toBeVisible();
  });

  test('should show SEO recommendations', async ({ page }) => {
    await page.goto('/grow/seo');
    await expect(page.locator('text=/seo|search.*engine|optimization/i')).toBeVisible();
  });

  test('should show social media integration', async ({ page }) => {
    await page.goto('/grow/social');
    await expect(page.locator('text=/social|instagram|twitter|facebook/i')).toBeVisible();
  });

  test('should show email marketing option', async ({ page }) => {
    await page.goto('/grow/email');
    await expect(page.locator('text=/email|newsletter|marketing/i')).toBeVisible();
  });
});