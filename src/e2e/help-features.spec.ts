import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('current embedded app smoke: help-features', async ({ page, request }) => {
  test.setTimeout(180000);
  await test('smoke', async ({ page, request }) => {
  await currentAppSmoke(page, request, page, request, 'help-features');
});
});
