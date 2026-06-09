import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('integrations', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'integrations');
});
