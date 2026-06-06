import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { GET, POST } from "./route";

describe("/api/staff/timecard", () => {
  beforeEach(() => {
    vi.stubEnv("BACKEND_URL", "http://backend.internal");
    global.fetch = vi.fn();
  });

  afterEach(() => {
    vi.unstubAllEnvs();
    vi.restoreAllMocks();
  });

  it("forwards timecard reads to the backend instead of in-memory events", async () => {
    const backendResponse = [{ id: "event-real", staff_id: "staff-real", event_type: "CLOCK_IN" }];
    (global.fetch as any).mockResolvedValueOnce({ ok: true, json: async () => backendResponse });

    const req = new Request("http://localhost/api/staff/timecard", {
      headers: { authorization: "Bearer token", "x-tenant-id": "tenant-1", "x-user-id": "user-1" },
    });

    const res = await GET(req);

    expect(res.status).toBe(200);
    await expect(res.json()).resolves.toEqual(backendResponse);
    expect(global.fetch).toHaveBeenCalledWith("http://backend.internal/api/staff/timecard", {
      method: "GET",
      headers: { authorization: "Bearer token", "x-tenant-id": "tenant-1", "x-user-id": "user-1" },
    });
  });

  it("forwards timecard sync events to the backend", async () => {
    const backendResponse = { success: true };
    (global.fetch as any).mockResolvedValueOnce({ ok: true, json: async () => backendResponse });

    const body = [{ staff_id: "staff-real", event_type: "CLOCK_IN", offline_timestamp: "2026-06-06T20:00:00Z" }];
    const req = new Request("http://localhost/api/staff/timecard", {
      method: "POST",
      headers: { authorization: "Bearer token", "x-tenant-id": "tenant-1", "x-user-id": "user-1" },
      body: JSON.stringify(body),
    });

    const res = await POST(req);

    expect(res.status).toBe(200);
    await expect(res.json()).resolves.toEqual(backendResponse);
    expect(global.fetch).toHaveBeenCalledWith("http://backend.internal/api/staff/timecard", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        authorization: "Bearer token",
        "x-tenant-id": "tenant-1",
        "x-user-id": "user-1",
      },
      body: JSON.stringify(body),
    });
  });
});
