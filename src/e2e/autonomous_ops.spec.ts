import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('autonomous_ops smoke', async ({ page, request }) => { await currentAppSmoke(page, request, 'autonomous_ops'); });
