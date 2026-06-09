import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('login_lens', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'login_lens');
});
