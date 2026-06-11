import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('milestone_loop', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'milestone_loop');
});
