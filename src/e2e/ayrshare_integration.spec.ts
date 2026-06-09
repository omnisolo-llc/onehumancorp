import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('ayrshare_integration', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'ayrshare_integration');
});
