import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('autonomous_ops', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'autonomous_ops');
});
