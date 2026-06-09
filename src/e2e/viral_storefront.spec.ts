import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('viral_storefront', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'viral_storefront');
});
