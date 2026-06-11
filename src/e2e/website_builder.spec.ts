import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('website_builder', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'website_builder');
});
