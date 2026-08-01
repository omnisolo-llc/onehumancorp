import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('viral_seasonal_promo', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'viral_seasonal_promo');
});
