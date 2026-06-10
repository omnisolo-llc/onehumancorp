import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('current embedded app smoke: help-features', async ({ page, request }) => {
  test.setTimeout(180000);
  await currentAppSmoke(page, request, 'help-features');
});
