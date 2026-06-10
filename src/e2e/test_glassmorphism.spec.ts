import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('test_glassmorphism', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'test_glassmorphism');
});
