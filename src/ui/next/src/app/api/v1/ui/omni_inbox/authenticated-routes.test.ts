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

import { POST as actOnMessage } from "./action/route";
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
