import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('playwright_kairos_mesh smoke', async ({ page, request }) => { await currentAppSmoke(page, request, 'playwright_kairos_mesh'); });
