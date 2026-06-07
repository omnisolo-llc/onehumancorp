import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { POST } from "./route";
import { NextRequest } from "next/server";

describe("POST /api/v1/growth/team-invites", () => {
  beforeEach(() => {
    vi.spyOn(console, "error").mockImplementation(() => {});
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

  it("should return 200 and success status if valid payload", async () => {
    const req = new NextRequest("http://localhost/api/v1/growth/team-invites", {
      method: "POST",
      body: JSON.stringify({ team_id: "test-team", inviter_id: "test-user" }),
    });

    const res = await POST(req);
    expect(res.status).toBe(200);
    const data = await res.json();
    expect(data.status).toBe("success");
    expect(data.team_id).toBe("test-team");
    expect(data.invite_link).toBe("https://ohc.app/invite/test-team");
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
