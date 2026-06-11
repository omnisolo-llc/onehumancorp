import { test, expect } from '@playwright/test';

const BASE_URL = 'http://localhost:3000';

test.describe('Viral Growth Features', () => {

  test('Viral Badge Visibility and Link on Quote Page', async ({ page }) => {
    // Set tenant in localStorage
    await page.goto(BASE_URL + '/dashboard.html');
    await page.evaluate(() => localStorage.setItem('tenant_id', 'maya-cakes'));

    await page.goto(BASE_URL + '/quote.html?id=123&tenant=maya-cakes');

    const badge = page.locator('#viral-badge');
    await expect(badge).toBeVisible();

    const link = page.locator('#viral-badge-link');
    await expect(link).toContainText('Get OHC');

    const href = await link.getAttribute('href');
    expect(href).toContain('ref=maya-cakes');
    expect(href).toContain('source=quote_viewer');
  });

  test('Referral Card on Success Page (Deposit Flow)', async ({ page }) => {
    await page.goto(BASE_URL + '/dashboard.html');
    await page.evaluate(() => localStorage.setItem('tenant_id', 'carlos-repairs'));

    // Simulate successful deposit
    await page.goto(BASE_URL + '/success.html?type=booking_deposit&tenant=carlos-repairs');

    const referralCard = page.locator('#referral-success-card');
    await expect(referralCard).toBeVisible();
    await expect(referralCard).toContainText('Start Your AI Business');

    const referralLink = page.locator('#referral-success-link');
    const href = await referralLink.getAttribute('href');
    expect(href).toContain('ref=carlos-repairs');
    expect(href).toContain('source=success_page');
  });

  test('Success Page does NOT show Referral Card for non-deposit types', async ({ page }) => {
    await page.goto(BASE_URL + '/success.html?type=general');
    const referralCard = page.locator('#referral-success-card');
    await expect(referralCard).not.toBeVisible();
  });

  test('Dashboard Footer Viral Link', async ({ page }) => {
    await page.goto(BASE_URL + '/dashboard.html');
    await page.evaluate(() => localStorage.setItem('tenant_id', 'nora-agency'));
    // Reload to apply script
    await page.reload();

    const footerLink = page.locator('#dashboard-footer-viral-link');
    await expect(footerLink).toBeVisible();
    await expect(footerLink).toContainText('Powered by OneHumanCorp');

    const href = await footerLink.getAttribute('href');
    expect(href).toContain('ref=nora-agency');
    expect(href).toContain('source=footer_widget');
  });

  test('Mobile Responsiveness: Viral Badge Touch Target', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto(BASE_URL + '/quote.html?id=123');

    const badge = page.locator('#viral-badge');
    await expect(badge).toBeVisible();

    const box = await badge.boundingBox();
    expect(box).not.toBeNull();
    if (box) {
      expect(box.width).toBeGreaterThanOrEqual(44);
      expect(box.height).toBeGreaterThanOrEqual(44);
    }

    // Check it doesn't overlap critical content (e.g. title)
    const title = page.locator('h1');
    const titleBox = await title.boundingBox();
    if (box && titleBox) {
      // It's fixed bottom, so it shouldn't overlap a top title on a short page unless page is very small
      expect(box.y).toBeGreaterThan(titleBox.y + titleBox.height);
    }
  });

});
