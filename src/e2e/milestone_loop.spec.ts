import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('milestone_loop smoke', async ({ page, request }) => { await currentAppSmoke(page, request, 'milestone_loop'); });
