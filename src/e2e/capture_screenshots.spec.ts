import { test, expect } from '@playwright/test';

test('capture dashboard and quote screenshots', async ({ page }) => {
  await page.setViewportSize({ width: 375, height: 812 });

  // Set some local storage to simulate a logged in user
  await page.addInitScript(() => {
      localStorage.setItem('tenant_id', 'e2e-tenant');
      localStorage.setItem('user_id', 'carlos-owner');
      localStorage.setItem('businessName', "Carlos' Repairs");
  });

  await page.goto('/dashboard.html');
  await page.screenshot({ path: 'dashboard_mobile.png' });

  await page.goto('/quote.html?id=123&mode=owner');
  await page.screenshot({ path: 'quote_owner_mobile.png' });

  await page.goto('/quote.html?id=123&mode=customer');
  await page.screenshot({ path: 'quote_customer_mobile.png' });
});
