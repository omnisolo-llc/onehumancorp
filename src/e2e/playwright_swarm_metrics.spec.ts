import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('Swarm Metrics and Memory State verify', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'playwright_swarm_metrics');
});
