import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('Current app smoke test', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'test_services_billing');
});
