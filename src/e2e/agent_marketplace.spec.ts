import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('agent-marketplace smoke', async ({ page, request }) => { await currentAppSmoke(page, request, 'agent-marketplace'); });
