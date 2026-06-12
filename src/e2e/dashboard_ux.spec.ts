import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('dashboard_ux', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'dashboard_ux');
});
