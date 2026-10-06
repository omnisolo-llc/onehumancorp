import { beforeEach, describe, expect, test, vi } from "vitest";

const { proxyBackendRequest } = vi.hoisted(() => ({
  proxyBackendRequest: vi.fn<
    (
      request: Request,
      path: string,
      options?: {
        forwardQuery?: boolean;
        suppressRequestBody?: true;
        requestContentType?: string;
        transformRequestBody?: (
          body: Uint8Array<ArrayBuffer>,
        ) => Uint8Array<ArrayBuffer>;
      },
    ) => Promise<Response>
  >(async () => Response.json({ ok: true })),
}));

vi.mock("@/lib/auth/backendTransport", () => ({ proxyBackendRequest }));

import { GET as readActionReceipt, POST as actOnMessage } from "./action/route";
import { GET as listInbox } from "./route";

describe("authenticated omni inbox routes", () => {
  beforeEach(() => proxyBackendRequest.mockClear());

  test("list derives identity from the server session", async () => {
    const request = new Request("http://localhost/api/v1/ui/omni_inbox?tenant_id=attacker");
    await listInbox(request);
    expect(proxyBackendRequest).toHaveBeenCalledWith(
      request,
      "/api/v1/ui/omni_inbox",
      { forwardQuery: false, suppressRequestBody: true },
    );
  });

  test("actions strip browser authority and unknown fields", async () => {
    const request = new Request("http://localhost/api/v1/ui/omni_inbox/action", {
      method: "POST",
      body: "{}",
    });
    await actOnMessage(request);
    const transform = proxyBackendRequest.mock.calls[0][2]?.transformRequestBody;
    const encoded = transform?.(
      new TextEncoder().encode(
        JSON.stringify({
          message_id: "message-1",
          approved: true,
          edited_reply: "Thanks",
          tenant_id: "attacker",
        }),
      ),
    );
    expect(JSON.parse(new TextDecoder().decode(encoded))).toEqual({
      message_id: "message-1",
      approved: true,
      edited_reply: "Thanks",
    });
  });
});

const requestId = "10b562b2-4c75-4f85-b6fa-43a99d9b655a";
test("preserves the bounded immutable request and preparation flag while removing browser identity", async () => {
  proxyBackendRequest.mockClear();
  await actOnMessage(new Request("http://localhost/api/v1/ui/omni_inbox/action", { method: "POST" }));
  const transform = proxyBackendRequest.mock.calls[0][2]?.transformRequestBody;
  const body = { message_id: "message-1", approved: true, edited_reply: "Reply", request_id: requestId, prepare_only: true };
  const result = transform!(new TextEncoder().encode(JSON.stringify({ ...body, user_id: "attacker", tenant_id: "attacker" })));
  expect(JSON.parse(new TextDecoder().decode(result))).toEqual(body);
});

test.each([
  { request_id: "bad/id" }, { request_id: 12 }, { prepare_only: "true" },
  { approved: false, prepare_only: true }, { request_id: "a".repeat(201) },
])("rejects malformed receipt controls %j", async invalid => {
  proxyBackendRequest.mockClear();
  await actOnMessage(new Request("http://localhost/api/v1/ui/omni_inbox/action", { method: "POST" }));
  const transform = proxyBackendRequest.mock.calls[0][2]?.transformRequestBody;
  expect(() => transform!(new TextEncoder().encode(JSON.stringify({ message_id: "message-1", approved: true, ...invalid })))).toThrow();
});

test.each([true, false])("reads actor-scoped receipts using only selected message and optional request ID (%s)", async withId => {
  proxyBackendRequest.mockClear();
  const query = `message_id=message-1${withId ? `&request_id=${requestId}` : ""}`;
  await readActionReceipt(new Request(`http://localhost/api/v1/ui/omni_inbox/action?${query}&tenant_id=attacker&user_id=attacker&extra=ignored`));
  const [request, path, options] = proxyBackendRequest.mock.calls[0];
  expect(new URL(request.url).search).toBe(`?${query}`);
  expect(path).toBe("/api/v1/ui/omni_inbox/action");
  expect(options).toEqual({ forwardQuery: true, suppressRequestBody: true });
});

test.each(["", "message_id=../foreign", "message_id=message-1&request_id=bad/id", "message_id=one&message_id=two", `message_id=message-1&request_id=${requestId}&request_id=${requestId}`])("rejects ambiguous or malformed receipt selectors %s", async query => {
  proxyBackendRequest.mockClear();
  const result = await readActionReceipt(new Request(`http://localhost/api/v1/ui/omni_inbox/action?${query}`));
  expect(result.status).toBe(400);
  expect(result.headers.get("cache-control")).toContain("no-store");
  expect(proxyBackendRequest).not.toHaveBeenCalled();
});


test("forwards bounded legacy request selectors for authoritative recovery", async () => {
  proxyBackendRequest.mockClear();
  const legacy = `legacy-${"a".repeat(64)}`;
  await readActionReceipt(new Request(`http://localhost/api/v1/ui/omni_inbox/action?message_id=message-1&request_id=${legacy}`));
  expect(new URL(proxyBackendRequest.mock.calls[0][0].url).searchParams.get("request_id")).toBe(legacy);
});
