import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('free_tier', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'free_tier');
});
