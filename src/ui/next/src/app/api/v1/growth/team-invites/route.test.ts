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
      body: JSON.stringify({ team_id: "1", inviter_id: "2", invitee_id: "3" }),
    });

    const res = await POST(req);
    expect(res.status).toBe(500);
  });

});
