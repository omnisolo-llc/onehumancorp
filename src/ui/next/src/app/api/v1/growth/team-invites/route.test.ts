import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { POST } from "./route";
import { NextRequest } from "next/server";

describe("POST /api/v1/growth/team-invites", () => {
  const mockBackendUrl = 'http://localhost:8080';

  beforeEach(() => {
    vi.spyOn(console, "error").mockImplementation(() => {});
    process.env.OHC_BACKEND_URL = mockBackendUrl;
    global.fetch = vi.fn();
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it("should return 400 if missing required fields", async () => {
    const req = new NextRequest("http://localhost/api/v1/growth/team-invites", {
      method: "POST",
      body: JSON.stringify({}),
    });

    const res = await POST(req);
    expect(res.status).toBe(400);
    const data = await res.json();
    expect(data.error).toBe("Missing required fields");
  });

  it("should proxy to backend and return 200 on success", async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ status: 'success' }),
      status: 200,
    });

    const req = new NextRequest("http://localhost/api/v1/growth/team-invites", {
      method: "POST",
      headers: {
        'authorization': 'Bearer test-token',
        'cookie': 'session=123',
      },
      body: JSON.stringify({ team_id: "test-team", inviter_id: "test-user", invitee_id: "test-invitee" }),
    });

    const res = await POST(req);

    expect(global.fetch).toHaveBeenCalledWith(`${mockBackendUrl}/api/v1/growth/team-invites`, expect.objectContaining({
      method: 'POST',
      headers: expect.any(Headers),
      body: JSON.stringify({ team_id: "test-team", inviter_id: "test-user", invitee_id: "test-invitee" })
    }));

    expect(res.status).toBe(200);
    const data = await res.json();
    expect(data.status).toBe("success");
  });

  it("should return error if backend request fails", async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: false,
      status: 403,
    });

    const req = new NextRequest("http://localhost/api/v1/growth/team-invites", {
      method: "POST",
      body: JSON.stringify({ team_id: "test-team", inviter_id: "test-user" }),
    });

    const res = await POST(req);
    expect(res.status).toBe(403);
    const data = await res.json();
    expect(data.error).toBe("Failed to generate team invite");
  });

  it("should handle internal errors gracefully", async () => {
    const req = {
      json: vi.fn().mockRejectedValue(new Error("Syntax Error")),
    } as unknown as NextRequest;

    const res = await POST(req);
    expect(res.status).toBe(500);
    const data = await res.json();
    expect(data.error).toBe("Internal server error");
  });
});
