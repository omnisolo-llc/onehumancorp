import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('regression_audit', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'regression_audit');
});
