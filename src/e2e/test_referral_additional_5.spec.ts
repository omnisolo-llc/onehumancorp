import { test, expect } from '@playwright/test';
import { memberPage as page } from './fixtures';

test('powered by OHC link is updated dynamically', async ({ page, context }) => {
  await page.goto('/');
  await page.evaluate(() => localStorage.setItem('tenant_id', 'powered-by-tenant'));
  // Trigger rendering logic for storefront preview which contains the powered by link
  await page.evaluate(() => {
    if (typeof (window as any).renderStorefrontPreview === 'function') {
        // Mock draft state to force render
        window.storefrontDraftState = [{ type: 'Hero', content: { title: 'T', subtitle: 'S', cta: 'C'} }];
        (window as any).renderStorefrontPreview();
    }
  });

  // Since storefront preview appends to container
  const poweredByLink = page.locator('.powered-by-footer a').first();
  await expect(poweredByLink).toHaveAttribute('href', 'ohc://join?ref=powered-by-tenant');
});
