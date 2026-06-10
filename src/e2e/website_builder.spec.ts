import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('website_builder smoke', async ({ page, request }) => { await currentAppSmoke(page, request, 'website_builder'); });
