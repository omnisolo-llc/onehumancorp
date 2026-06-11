import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('seasonal_promo_paywall', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'seasonal_promo_paywall');
});
