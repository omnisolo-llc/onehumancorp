import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('inbox', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'inbox');
});
