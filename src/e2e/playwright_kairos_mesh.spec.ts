import { test, expect } from '@playwright/test';

test('Broadcast KAIROS mesh message validates payload', async ({ request }) => {
    // Attempt invalid request without data
    let response = await request.post('/api/mesh/broadcast', {
        data: {
            agent_id: "agent-1",
            event_type: "TEST_EVENT",
            channel: "test_channel"
        }
    });
    expect(response.status()).toBe(400);

    // Attempt valid request
    response = await request.post('/api/mesh/broadcast', {
        data: {
            agent_id: "agent-1",
            event_type: "TEST_EVENT",
            channel: "test_channel",
            data: { key: "value" },
            action: "test",
            status: "ok"
        }
    });
    expect(response.status()).not.toBe(400);
});
