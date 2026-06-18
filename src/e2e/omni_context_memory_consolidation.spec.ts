import { test, expect } from './fixtures';

test.describe('Omni Context Memory Consolidation', () => {
  test('Agents page loads without errors', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/agents');

    // Basic smoke test to ensure the page doesn't crash
    await expect(page.getByRole('heading', { name: 'Agents', level: 1 })).toBeVisible();
  });

  test('Memory panel renders properly', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/agents');

    await page.evaluate(() => {
       const btns = Array.from(document.querySelectorAll('button'));
       const memBtn = btns.find(b => b.textContent?.trim() === 'Memory');
       if(memBtn) memBtn.click();
    });

    // Wait explicitly for the state update inside React to settle.
    await page.waitForFunction(() => {
      const h2s = Array.from(document.querySelectorAll('h2'));
      return h2s.some(el => el.textContent?.includes('Consolidated Memory'));
    }, { timeout: 15000 });
  });

  test('Memory list displays empty state', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/agents');
    await page.evaluate(() => {
       const btns = Array.from(document.querySelectorAll('button'));
       const memBtn = btns.find(b => b.textContent?.trim() === 'Memory');
       if(memBtn) memBtn.click();
    });

    await page.waitForFunction(() => {
      return document.documentElement.textContent?.includes('No consolidated memories found.');
    }, { timeout: 15000 });
  });

  test('Memory detail subtitle is visible', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/agents');
    await page.evaluate(() => {
       const btns = Array.from(document.querySelectorAll('button'));
       const memBtn = btns.find(b => b.textContent?.trim() === 'Memory');
       if(memBtn) memBtn.click();
    });

    await page.waitForFunction(() => {
      return document.documentElement.textContent?.includes('Review and override what AI agents remember about your business.');
    }, { timeout: 15000 });
  });

  test('Explore panel is visible', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/agents');

    await page.evaluate(() => {
       const btns = Array.from(document.querySelectorAll('button'));
       const memBtn = btns.find(b => b.textContent?.trim() === 'Templates');
       if(memBtn) memBtn.click();
    });

    await page.waitForFunction(() => {
      return document.documentElement.textContent?.includes('Explore Templates');
    }, { timeout: 15000 });
  });
});
