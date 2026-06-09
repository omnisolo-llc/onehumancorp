import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('lens_audit', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'lens_audit');
});
