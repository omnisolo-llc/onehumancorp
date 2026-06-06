import { test } from '@playwright/test';
import { currentAppSmoke } from './current_app_smoke';

test('currentAppSmoke: agent_audit_dashboard', async ({ page, request }) => { await currentAppSmoke(page, request, 'agent_audit_dashboard'); });
