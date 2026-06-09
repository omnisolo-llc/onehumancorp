import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('regression audit', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'regression_audit');
});
