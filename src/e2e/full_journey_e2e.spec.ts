import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('full_journey_e2e', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'full_journey_e2e');
});
