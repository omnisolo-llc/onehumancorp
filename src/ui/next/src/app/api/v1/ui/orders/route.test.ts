import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import {
  authenticatedRequest,
  stubAuthEnvironment,
  TEST_BACKEND_ORIGIN,
} from "@/lib/auth/authTestFixtures";
import { GET } from "./route";

describe("GET /api/v1/ui/orders", () => {
  beforeEach(() => {
    stubAuthEnvironment();
    global.fetch = vi.fn();
  });

  afterEach(() => {
    vi.unstubAllEnvs();
    vi.restoreAllMocks();
  });

  it("proxies order lists to the Rust backend", async () => {
    const backendResponse = [{ id: "order-1", total_amount: 4200, status: "pending" }];
    vi.mocked(global.fetch).mockResolvedValueOnce(Response.json(backendResponse));

    const res = await GET(
      await authenticatedRequest("/api/v1/ui/orders?tenant_id=tenant-1"),
    );

    expect(res.status).toBe(200);
    await expect(res.json()).resolves.toEqual(backendResponse);
    expect(global.fetch).toHaveBeenCalledWith(
      new URL(`${TEST_BACKEND_ORIGIN}/api/v1/ui/orders?tenant_id=tenant-7`),
      expect.objectContaining({ method: "GET" }),
    );
  });
});

describe('GET exact order through the existing authenticated UI catchall', () => {
  beforeEach(() => { vi.resetModules(); stubAuthEnvironment(); global.fetch = vi.fn(); });
  afterEach(() => { vi.unstubAllEnvs(); vi.restoreAllMocks(); });

  it('preserves an exact older order and binds the read to the sealed-session tenant', async () => {
    const { GET: detail } = await import('../[...path]/route');
    const order = { id: 'older-order', customer_name: 'Owned customer', status: 'pending' };
    vi.mocked(global.fetch).mockResolvedValueOnce(Response.json(order));
    const res = await detail(await authenticatedRequest('/api/v1/ui/orders/older-order?tenant_id=foreign'));
    expect(res.status).toBe(200);
    expect(await res.json()).toEqual(order);
    expect(global.fetch).toHaveBeenCalledWith(new URL(`${TEST_BACKEND_ORIGIN}/api/v1/ui/orders/older-order?tenant_id=tenant-7`), expect.objectContaining({ method: 'GET', cache: 'no-store' }));
    expect(res.headers.get('cache-control')).toBe('private, no-store');
  });

  it.each([404, 500])('preserves backend detail HTTP %s without substituting an empty or synthetic order', async status => {
    const { GET: detail } = await import('../[...path]/route');
    vi.mocked(global.fetch).mockResolvedValueOnce(Response.json({ error: 'unavailable' }, { status }));
    const res = await detail(await authenticatedRequest('/api/v1/ui/orders/missing'));
    expect(res.status).toBe(status);
    expect(await res.json()).toEqual({ error: 'unavailable' });
  });
});
