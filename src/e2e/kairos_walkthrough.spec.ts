import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('kairos_walkthrough smoke', async ({ page, request }) => { await currentAppSmoke(page, request, 'kairos_walkthrough'); });
