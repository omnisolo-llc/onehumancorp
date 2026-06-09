import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('echo_navigation', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'echo_navigation');
});
