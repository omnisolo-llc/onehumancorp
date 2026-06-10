import { test } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('success_milestones smoke', async ({ page, request }) => { await currentAppSmoke(page, request, 'success_milestones'); });
