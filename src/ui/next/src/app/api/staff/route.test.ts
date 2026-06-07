import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { GET, POST } from "./route";

describe("/api/staff", () => {
  beforeEach(() => {
    vi.stubEnv("BACKEND_URL", "http://backend.internal");
    global.fetch = vi.fn();
  });

  afterEach(() => {
    vi.unstubAllEnvs();
    vi.restoreAllMocks();
  });

  it("forwards staff reads to the backend instead of in-memory staff", async () => {
    const backendResponse = [{ id: "staff-real", name: "Amina", role: "Cashier" }];
    (global.fetch as any).mockResolvedValueOnce({ ok: true, json: async () => backendResponse });

    const req = new Request("http://localhost/api/staff", {
      headers: { authorization: "Bearer token", "x-tenant-id": "tenant-1", "x-user-id": "user-1" },
    });

    const res = await GET(req);

    expect(res.status).toBe(200);
    await expect(res.json()).resolves.toEqual(backendResponse);
    expect(global.fetch).toHaveBeenCalledWith("http://backend.internal/api/staff", {
      method: "GET",
      headers: { authorization: "Bearer token", "x-tenant-id": "tenant-1", "x-user-id": "user-1" },
    });
  });

  it("forwards staff creation to the backend", async () => {
    const backendResponse = { id: "staff-real", invite_token: "invite-real" };
    (global.fetch as any).mockResolvedValueOnce({ ok: true, json: async () => backendResponse });

    const body = { name: "Amina", phone_number: "+15551234567", role: "Cashier" };
    const req = new Request("http://localhost/api/staff", {
      method: "POST",
      headers: { authorization: "Bearer token", "x-tenant-id": "tenant-1", "x-user-id": "user-1" },
      body: JSON.stringify(body),
    });

    const res = await POST(req);

    expect(res.status).toBe(200);
    await expect(res.json()).resolves.toEqual(backendResponse);
    expect(global.fetch).toHaveBeenCalledWith("http://backend.internal/api/staff", {
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
