import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('unified_catalog smoke', async ({ page, request }) => { await currentAppSmoke(page, request, 'unified_catalog'); });
