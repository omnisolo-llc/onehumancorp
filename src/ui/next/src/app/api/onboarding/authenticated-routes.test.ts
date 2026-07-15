import { beforeEach, describe, expect, test, vi } from "vitest";

const { proxyBackendRequest, validateJsonRequestBody } = vi.hoisted(() => ({
  proxyBackendRequest: vi.fn<
    (
      request: Request,
      backendPath: string,
      options?: {
        forwardQuery?: boolean;
        requestContentType?: "application/json";
        suppressRequestBody?: true;
        transformRequestBody?: (
          body: Uint8Array<ArrayBuffer>,
        ) => Uint8Array<ArrayBuffer>;
      },
    ) => Promise<Response>
  >(async () => Response.json({ ok: true })),
  validateJsonRequestBody: vi.fn((body: Uint8Array<ArrayBuffer>) => body),
}));

vi.mock("@/lib/auth/backendTransport", () => ({
  proxyBackendRequest,
  validateJsonRequestBody,
}));

import { POST as chat } from "./chat/route";
import { GET as getDraft, POST as saveDraft } from "./draft/route";
import { POST as intake } from "./intake/route";
import { POST as launch } from "./launch/route";
import { POST as start } from "./start/route";
import { POST as startZeroClick } from "./start_zero_click/route";
import { GET as getState, POST as saveState } from "./state/route";
import { POST as v1Chat } from "../v1/onboarding/chat/route";
import { POST as v1Start } from "../v1/onboarding/start/route";
import { POST as v1StartZeroClick } from "../v1/onboarding/start_zero_click/route";
import { POST as legacyGrowthZeroClick } from "../v1/growth/zero-click-builder/generate/route";

const jsonOptions = {
  forwardQuery: false,
  requestContentType: "application/json",
  transformRequestBody: validateJsonRequestBody,
} as const;

describe("authenticated onboarding backend routes", () => {
  beforeEach(() => proxyBackendRequest.mockClear());

  test.each([
    ["chat", chat, "/api/onboarding/chat"],
    ["intake", intake, "/api/onboarding/intake"],
  ])("%s uses the fixed authenticated JSON contract", async (_name, route, path) => {
    const request = new Request("http://localhost/route?tenant_id=attacker", {
      method: "POST",
      headers: { "content-type": "text/plain" },
      body: '{"messages":[]}',
    });

    await route(request);

    expect(proxyBackendRequest).toHaveBeenCalledWith(request, path, jsonOptions);
  });

  test.each([
    [
      "zero click",
      startZeroClick,
      "/api/onboarding/start_zero_click",
    ],
    [
      "v1 zero click",
      v1StartZeroClick,
      "/api/onboarding/start_zero_click",
    ],
    [
      "legacy growth zero click",
      legacyGrowthZeroClick,
      "/api/onboarding/start_zero_click",
    ],
  ])("%s bounds input and strips browser authority", async (_name, route, path) => {
    const request = new Request("http://localhost/route?tenant_id=attacker", {
      method: "POST",
      headers: { "content-type": "text/plain" },
      body: '{"prompt":"A bakery","tenant_id":"attacker"}',
    });

    await route(request);

    expect(proxyBackendRequest).toHaveBeenCalledWith(
      request,
      path,
      expect.objectContaining({
        forwardQuery: false,
        requestContentType: "application/json",
        transformRequestBody: expect.any(Function),
      }),
    );
    const transform = proxyBackendRequest.mock.calls.at(-1)?.[2]
      ?.transformRequestBody;
    const encoded = transform?.(
      new TextEncoder().encode(await request.clone().text()),
    );
    expect(JSON.parse(new TextDecoder().decode(encoded))).toEqual({
      prompt: "A bakery",
    });
    expect(() =>
      transform?.(
        new TextEncoder().encode(
          JSON.stringify({ prompt: "x".repeat(4_001) }),
        ),
      ),
    ).toThrow();
  });

  test.each([
    ["start", start],
    ["v1 start", v1Start],
  ])("%s strips credentials and browser authority", async (_name, route) => {
    const request = new Request("http://localhost/route?tenant_id=attacker", {
      method: "POST",
      body: JSON.stringify({
        company_name: "Bakery",
        admin_email: "attacker@example.com",
        admin_name: "Attacker",
        admin_password: "Secret123",
        tenant_id: "attacker",
      }),
    });

    await route(request);

    const transform = proxyBackendRequest.mock.calls.at(-1)?.[2]
      ?.transformRequestBody;
    const encoded = transform?.(
      new TextEncoder().encode(await request.clone().text()),
    );
    const body = JSON.parse(new TextDecoder().decode(encoded));
    expect(body.company_name).toBe("Bakery");
    expect(body.admin_email).toBeUndefined();
    expect(body.admin_name).toBeUndefined();
    expect(body.admin_password).toBeUndefined();
    expect(body.tenant_id).toBeUndefined();
  });

  test.each([
    ["draft", getDraft, "/api/onboarding/draft"],
    ["state", getState, "/api/onboarding/state"],
  ])("GET %s suppresses queries and request bodies", async (_name, route, path) => {
    const request = new Request("http://localhost/route?tenant_id=attacker");

    await route(request);

    expect(proxyBackendRequest).toHaveBeenCalledWith(request, path, {
      forwardQuery: false,
      suppressRequestBody: true,
    });
  });

  test.each([
    ["draft", saveDraft, "/api/onboarding/draft"],
    ["state", saveState, "/api/onboarding/state"],
  ])("POST %s strips authority and secret fields", async (_name, route, path) => {
    const request = new Request("http://localhost/route?tenant_id=attacker", {
      method: "POST",
      body: "{}",
    });

    await route(request);

    expect(proxyBackendRequest).toHaveBeenCalledWith(
      request,
      path,
      expect.objectContaining({
        forwardQuery: false,
        requestContentType: "application/json",
        transformRequestBody: expect.any(Function),
      }),
    );
    const transform = proxyBackendRequest.mock.calls.at(-1)?.[2]
      ?.transformRequestBody;
    const encoded = transform?.(
      new TextEncoder().encode(
        JSON.stringify({
          step: 2,
          tenant_id: "attacker",
          adminPassword: "TopSecret123",
          wizardState: {
            businessName: "Bakery",
            admin_password: "NestedSecret123",
            unknown: true,
          },
        }),
      ),
    );
    expect(JSON.parse(new TextDecoder().decode(encoded))).toEqual({
      step: 2,
      wizardState: { businessName: "Bakery" },
    });
  });

  test("launch forwards neither attacker queries nor a body", async () => {
    const request = new Request("http://localhost/route?tenant_id=attacker", {
      method: "POST",
      body: '{"tenant_id":"attacker"}',
    });

    await launch(request);

    expect(proxyBackendRequest).toHaveBeenCalledWith(
      request,
      "/api/onboarding/launch",
      { forwardQuery: false, suppressRequestBody: true },
    );
  });

  test("v1 chat retains its messages-array validation", async () => {
    const request = new Request("http://localhost/route?tenant_id=attacker", {
      method: "POST",
      body: '{"messages":[]}',
    });

    await v1Chat(request);

    expect(proxyBackendRequest).toHaveBeenCalledWith(
      request,
      "/api/onboarding/chat",
      expect.objectContaining({
        forwardQuery: false,
        requestContentType: "application/json",
        transformRequestBody: expect.any(Function),
      }),
    );
    const options = proxyBackendRequest.mock.calls.at(-1)?.[2];
    const transform = options?.transformRequestBody;
    expect(transform).toBeTypeOf("function");
    expect(() => transform?.(new TextEncoder().encode("{}"))).toThrow();
    expect(
      new TextDecoder().decode(
        transform?.(
          new TextEncoder().encode(
            '{"messages":[],"tenant_id":"attacker","extra":true}',
          ),
        ) as Uint8Array,
      ),
    ).toBe('{"messages":[]}');
  });
});
