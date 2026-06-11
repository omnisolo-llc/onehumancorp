import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('echo_ux_friction', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'echo_ux_friction');
});
