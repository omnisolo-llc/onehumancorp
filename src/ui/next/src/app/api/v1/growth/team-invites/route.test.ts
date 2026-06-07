import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { POST, GET } from "./route";
import { NextRequest } from "next/server";

describe("GET/POST /api/v1/growth/team-invites", () => {
  beforeEach(() => {
    vi.spyOn(console, "error").mockImplementation(() => {});
    global.fetch = vi.fn();
  });
  afterEach(() => {
    vi.restoreAllMocks();
  });

  it("should return 500 on fetch error", async () => {
    (global.fetch as any).mockRejectedValue(new Error("Network Error"));
    const req = new NextRequest("http://localhost/api/v1/growth/team-invites", {
      method: "GET",
    });

    const res = await GET(req);
    expect(res.status).toBe(500);
  });

  it("should return 500 on fetch error for POST", async () => {
    (global.fetch as any).mockRejectedValue(new Error("Network Error"));
    const req = new NextRequest("http://localhost/api/v1/growth/team-invites", {
      method: "POST",
      body: JSON.stringify({ team_id: "1" }),
    });

    const res = await POST(req);
    expect(res.status).toBe(500);
  });

  it("should successfully generate a team invite and return invite_link", async () => {
    (global.fetch as any).mockResolvedValue({
      ok: true,
      json: async () => ({ invite_link: "https://ohc.app/team/join/user-123?tenant=team-1" })
    });

    const req = new NextRequest("http://localhost/api/v1/growth/team-invites", {
      method: "POST",
      body: JSON.stringify({ team_id: "team-1", inviter_id: "inviter-1", invitee_id: "user-123" }),
    });

    const res = await POST(req);
    expect(res.status).toBe(200);
    const data = await res.json();
    expect(data.invite_link).toBe("https://ohc.app/team/join/user-123?tenant=team-1");
  });

});
