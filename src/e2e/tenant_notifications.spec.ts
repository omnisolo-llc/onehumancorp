import { test, expect } from '@playwright/test';
import { execSync } from 'child_process';
import net from 'net';

async function publishToRedis(topic: string, payload: string) {
  const redisUrlStr = process.env.REDIS_URL || 'redis://127.0.0.1:6379';
  const url = new URL(redisUrlStr);
  const client = net.createConnection({ host: url.hostname, port: url.port ? parseInt(url.port) : 6379 });

  await new Promise((resolve, reject) => {
    client.once('connect', resolve);
    client.once('error', reject);
  });

  const respCommand = `*3\r\n$7\r\nPUBLISH\r\n$${topic.length}\r\n${topic}\r\n$${payload.length}\r\n${payload}\r\n`;
  client.write(respCommand);

  await new Promise(resolve => setTimeout(resolve, 100)); // wait for write to complete
  client.end();
}

test.describe('Real-Time Multi-Tenant Edge Notifications', () => {
  test('should push real-time notification to client via Redis PubSub', async ({ page }) => {
    const tenantId = 'e2e-tenant-notifications-' + Date.now();
    const spiffeId = `spiffe://ohc/org/${tenantId}/agent/browser`;

    // Configure mock auth via localStorage and mock backend proxy headers in test setup
    await page.goto('/');

    await page.evaluate(({ tenantId, spiffeId }) => {
      localStorage.setItem('tenant_id', tenantId);
      localStorage.setItem('mock_spiffe_id', spiffeId);
    }, { tenantId, spiffeId });

    // Navigate to a valid dashboard view
    await page.goto('/dashboard');

    // Wait for the app to initialize, WebSocket to connect and the notification manager to be ready
    await page.waitForTimeout(2000);

    const payload = JSON.stringify({
      event: 'critical_alert',
      message: 'A critical event happened!'
    });

    const topic = `tenant_events:${tenantId}`;

    try {
        await publishToRedis(topic, payload);
    } catch (e) {
        console.error("Failed to publish via socket", e);
        // fallback to standard API that publishes something, if any.
        // Since we are running outside Docker for the test driver, the redis container might be mapped to a random port or valkey hostname.
        // We will attempt to run docker exec if we can't connect directly.
        try {
            execSync(`docker exec deploy-valkey-1 redis-cli PUBLISH "${topic}" '${payload}'`);
        } catch (dockerErr) {
            console.error("Failed to publish via docker exec", dockerErr);
            throw e;
        }
    }

    // Verify the toast notification appears
    const notificationToast = page.locator('[data-testid="notification-toast"]');
    await expect(notificationToast).toBeVisible();
    await expect(notificationToast).toContainText('A critical event happened!');
  });
});
