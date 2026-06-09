import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('smoke test seasonal_promo_paywall', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'seasonal_promo_paywall');
});
