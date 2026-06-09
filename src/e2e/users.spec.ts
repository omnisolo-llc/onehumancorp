import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('users', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'users');
});
