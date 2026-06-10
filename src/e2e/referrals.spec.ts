import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('referrals smoke', async ({ page, request }) => { await currentAppSmoke(page, request, 'referrals'); });
