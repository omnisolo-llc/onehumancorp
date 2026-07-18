import { describe, expect, it } from "vitest";
import { jsonRpcRequestTransform } from "./jsonRpc";

const decode = (value: Uint8Array<ArrayBuffer>) =>
  JSON.parse(new TextDecoder().decode(value));

describe("JSON-RPC backend request transform", () => {
  it("uses a fixed method and strips browser-selected identity before mapping params", () => {
    const transform = jsonRpcRequestTransform("run_agent", (input) => ({
      message: input.message,
      tenant_id: input.tenant_id,
    }));

    const result = decode(transform(new TextEncoder().encode(JSON.stringify({
      message: "hello",
      tenant_id: "attacker",
      authorization: "Bearer attacker",
    }))));

    expect(result).toEqual({
      jsonrpc: "2.0",
      id: expect.any(String),
      method: "run_agent",
      params: { message: "hello" },
    });
  });

  it("supports bodyless requests with fixed parameters", () => {
    const transform = jsonRpcRequestTransform("goose_mcp_list", () => ({}));
    expect(decode(transform(new Uint8Array()))).toMatchObject({
      jsonrpc: "2.0",
      method: "goose_mcp_list",
      params: {},
    });
  });
});
