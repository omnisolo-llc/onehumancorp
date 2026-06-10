import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('business_setup_2 smoke', async ({ page, request }) => { await currentAppSmoke(page, request, 'business_setup_2'); });
