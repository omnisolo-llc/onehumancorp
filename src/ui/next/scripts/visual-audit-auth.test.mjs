import { expect, test } from "vitest";
import { loginForVisualAudit } from "./visual-audit-auth.mjs";

test("normalizes the backend login response for an authenticated visual audit", async () => {
  const result = await loginForVisualAudit({
    authOrigin: "http://127.0.0.1:18989",
    username: "test@example.com",
    password: "password123",
    organizationId: "e2e-tenant",
    fetchImpl: async (input, init) => {
      expect(String(input)).toBe("http://127.0.0.1:18989/api/v1/auth/login");
      expect(init.method).toBe("POST");
      expect(init.headers).toEqual({
        "content-type": "application/json",
        origin: "http://127.0.0.1:18989",
        "sec-fetch-site": "same-origin",
      });
      expect(JSON.parse(init.body)).toEqual({
        username: "test@example.com",
        password: "password123",
        organization_id: "e2e-tenant",
      });
      return new Response(JSON.stringify({
        token: "backend-token",
        expires_at: 1_786_389_075,
        user: {
          id: "e2e-admin-user",
          username: "test@example.com",
          roles: ["ADMIN"],
          organization_id: "e2e-tenant",
        },
      }), { status: 200, headers: { "content-type": "application/json" } });
    },
  });

  expect(result).toEqual({
    accessToken: "backend-token",
    expiresAt: 1_786_389_075,
    user: {
      id: "e2e-admin-user",
      username: "test@example.com",
      roles: ["ADMIN"],
      organizationId: "e2e-tenant",
    },
  });
});

test("fails closed when visual-audit credentials are rejected", async () => {
  await expect(loginForVisualAudit({
    authOrigin: "http://127.0.0.1:18989",
    username: "test@example.com",
    password: "wrong",
    organizationId: "e2e-tenant",
    fetchImpl: async () => new Response("unauthorized", { status: 401 }),
  })).rejects.toThrow("visual audit login failed with HTTP 401");
});
