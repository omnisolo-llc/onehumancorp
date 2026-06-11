import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('agent-marketplace', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'agent-marketplace');
});
