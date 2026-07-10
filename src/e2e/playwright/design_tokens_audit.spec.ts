import { test, expect } from '@playwright/test';

test.describe('Premium Design Token Compliance Audit', () => {
  const checkGlassmorphism = async (page, selector) => {
    const el = page.locator(selector).first();
    await expect(el).toBeVisible({ timeout: 15000 });

    // Playwright evaluates the computed style which normalizes the filter string
    // e.g., "blur(30px) saturate(210%)" usually becomes "blur(30px) saturate(2.1)" in computed style.
    // Let's check the inline style string directly to be safe, or just check that it doesn't contain "40px"
    const style = await el.getAttribute('style') || await el.evaluate((e) => window.getComputedStyle(e).backdropFilter);
    expect(style).toContain('blur(30px)');
    expect(style).toContain('saturate(210%)');
    expect(style).not.toContain('blur(40px)');
  };

  test('dashboard feed card uses correct glassmorphism tokens', async ({ page }) => {
    await page.goto('/dashboard');
    // Ensure page loads and a triage item is visible
    // We can inject a mock triage item if necessary, or just check the setup elements
    const welcomeCard = page.getByTestId('onboarding-welcome-card');
    await expect(welcomeCard).toBeVisible({ timeout: 15000 });

    const style = await welcomeCard.getAttribute('style');
    expect(style).toContain('blur(30px) saturate(210%)');
    expect(style).not.toContain('blur(40px) saturate(220%)');
  });

  test('triage page layout uses correct glassmorphism tokens', async ({ page }) => {
    await page.goto('/api/ui/triage.html');
    const header = page.locator('.triage-header').first();
    // In triage.html, .glassmorphism is applied to multiple elements
    // Let's wait for the empty state card which has .glassmorphism
    const emptyCard = page.locator('.glassmorphism').first();
    await expect(emptyCard).toBeVisible({ timeout: 15000 });

    const style = await emptyCard.evaluate((e) => window.getComputedStyle(e).backdropFilter);
    // getComputedStyle usually returns e.g. "blur(30px) saturate(2.1)"
    expect(style).toMatch(/blur\(30px\)/);
    expect(style).toMatch(/saturate\(2\.1\)/);
  });

  test('inbox page uses correct glassmorphism tokens on container', async ({ page }) => {
    await page.goto('/api/ui/inbox.html');
    const container = page.locator('.container.glassmorphism').first();
    await expect(container).toBeVisible({ timeout: 15000 });

    const style = await container.evaluate((e) => window.getComputedStyle(e).backdropFilter);
    expect(style).toMatch(/blur\(30px\)/);
    expect(style).toMatch(/saturate\(2\.1\)/);
  });

  test('setup wizard form uses correct glassmorphism tokens', async ({ page }) => {
    await page.goto('/setup.html');
    const container = page.locator('#form-container').first();
    await expect(container).toBeVisible({ timeout: 15000 });

    const style = await container.evaluate((e) => window.getComputedStyle(e).backdropFilter);
    expect(style).toMatch(/blur\(30px\)/);
    expect(style).toMatch(/saturate\(2\.1\)/);
  });

  test('viral newsletter generator uses correct glassmorphism tokens', async ({ page }) => {
    await page.goto('/api/ui/viral-newsletter-generator.html');
    const container = page.locator('.container.glassmorphism').first();
    await expect(container).toBeVisible({ timeout: 15000 });

    const style = await container.evaluate((e) => window.getComputedStyle(e).backdropFilter);
    expect(style).toMatch(/blur\(30px\)/);
    expect(style).toMatch(/saturate\(2\.1\)/);
  });
});
