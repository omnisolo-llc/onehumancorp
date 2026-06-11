import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('ayrshare_integration', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'ayrshare_integration');
});
