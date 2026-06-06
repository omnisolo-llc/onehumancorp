import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('currentAppSmoke: agent-marketplace', async ({ page, request }) => { await currentAppSmoke(page, request, 'agent-marketplace'); });
