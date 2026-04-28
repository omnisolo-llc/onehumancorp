/**
 * Meetings API E2E Tests
 * Converted from e2e_meetings_tests.sh
 */

import { describe, it, expect, beforeAll } from 'vitest';
import { httpGet, httpPost, assertJsonField, waitForServer } from './test-utils';

describe('Meetings API', () => {
  beforeAll(async () => {
    await waitForServer();
  });

  describe('List Meetings', () => {
    it('should return meetings list', async () => {
      const resp = await httpGet('/api/meetings');
      assertJsonField(resp, '.meetings');
    });

    it('should return valid JSON for meetings list', async () => {
      const resp = await httpGet('/api/meetings');
      expect(() => JSON.parse(resp)).not.toThrow();
    });

    it('should handle meetings list with type filter', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/meetings?type=standup');
      expect([200, 400]).toContain(response.status);
    });

    it('should handle meetings list with limit', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/meetings?limit=20');
      expect([200, 400]).toContain(response.status);
    });

    it('should handle meetings list pagination', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/meetings?page=1&size=10');
      expect([200, 400]).toContain(response.status);
    });
  });

  describe('Create Meetings', () => {
    it('should create basic meeting', async () => {
      const resp = await httpPost('/api/meetings', {
        title: 'Test Meeting',
        attendees: ['agent1', 'agent2'],
      }, 201);
      assertJsonField(resp, '.id');
    });

    it('should create meeting with full details', async () => {
      const resp = await httpPost('/api/meetings', {
        title: 'Full Meeting',
        attendees: ['a1', 'a2', 'a3'],
        duration: 3600,
        scheduled_at: '2024-12-01T10:00:00Z',
        type: 'standup',
      }, 201);
      expect(resp).toBeTruthy();
    });

    it('should create minimal meeting', async () => {
      const resp = await httpPost('/api/meetings', {
        title: 'Minimal Meeting',
      }, 201);
      expect(resp).toBeTruthy();
    });

    it('should create meeting with agenda', async () => {
      const resp = await httpPost('/api/meetings', {
        title: 'Agenda Meeting',
        agenda: ['item1', 'item2', 'item3'],
      }, 201);
      expect(resp).toBeTruthy();
    });

    it('should create meeting with notes', async () => {
      const resp = await httpPost('/api/meetings', {
        title: 'Notes Meeting',
        notes: 'Initial meeting notes',
      }, 201);
      expect(resp).toBeTruthy();
    });

    it('should list meetings after create', async () => {
      await httpPost('/api/meetings', {
        title: 'Listed Meeting',
      }, 201);
      const resp = await httpGet('/api/meetings');
      expect(() => JSON.parse(resp)).not.toThrow();
    });

    it('should handle concurrent meeting creation', async () => {
      const promises = Array.from({ length: 5 }, (_, i) =>
        httpPost('/api/meetings', {
          title: `concurrent-meeting-${i}`,
        }, 201),
      );
      const results = await Promise.all(promises);
      expect(results).toHaveLength(5);
    });

    it('should handle sequential meeting creation', async () => {
      for (let i = 0; i < 10; i++) {
        const resp = await httpPost('/api/meetings', {
          title: `sequential-meeting-${i}`,
        }, 201);
        expect(resp).toBeTruthy();
      }
    });

    it('should handle invalid JSON', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/meetings', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: '{invalid}',
      });
      expect(response.status).toBe(400);
    });

    it('should handle empty JSON object', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/meetings', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: '{}',
      });
      expect([200, 201, 400]).toContain(response.status);
    });

    it('should handle long title', async () => {
      const title = 'x'.repeat(500);
      const response = await fetch('http://127.0.0.1:18080/api/meetings', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ title }),
      });
      expect([200, 201, 400]).toContain(response.status);
    });

    it('should create meeting with many attendees', async () => {
      const attendees = Array.from({ length: 20 }, (_, i) => `agent-${i}`);
      const response = await fetch('http://127.0.0.1:18080/api/meetings', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          title: 'Large Group Meeting',
          attendees,
        }),
      });
      expect([200, 201, 400]).toContain(response.status);
    });

    it('should create meeting at specific time', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/meetings', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          title: 'Scheduled Meeting',
          scheduled_at: '2024-12-15T14:30:00Z',
        }),
      });
      expect([200, 201, 400]).toContain(response.status);
    });

    it('should create meeting with duration', async () => {
      const response = await fetch('http://127.0.0.1:18080/api/meetings', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          title: 'Timed Meeting',
          duration: 1800,
        }),
      });
      expect([200, 201, 400]).toContain(response.status);
    });

    it('should handle batch meeting creation', async () => {
      const promises = Array.from({ length: 3 }, (_, i) =>
        httpPost('/api/meetings', {
          title: `batch-meeting-${i}`,
        }, 201),
      );
      const results = await Promise.all(promises);
      expect(results).toHaveLength(3);
    });
  });

  describe('Performance', () => {
    it('should complete meeting list request within 2 seconds', async () => {
      const startTime = Date.now();
      await httpGet('/api/meetings');
      const duration = Date.now() - startTime;
      expect(duration).toBeLessThan(2000);
    });
  });
});
