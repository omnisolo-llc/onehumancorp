import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('playwright_kairos_mesh', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'playwright_kairos_mesh');
});
