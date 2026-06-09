import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('smoke test viral_storefront', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'viral_storefront');
});
