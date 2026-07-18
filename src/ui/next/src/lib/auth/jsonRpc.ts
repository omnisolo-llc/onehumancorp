import { stripBrowserIdentityJsonRequestBody } from "./backendTransport";

const decoder = new TextDecoder("utf-8", { fatal: true });
const encoder = new TextEncoder();
const SAFE_METHOD = /^[a-z][a-z0-9_.-]{0,127}$/;

type JsonObject = Record<string, unknown>;

export function jsonRpcRequestTransform(
  method: string,
  mapParams: (input: JsonObject) => JsonObject,
): (body: Uint8Array<ArrayBuffer>) => Uint8Array<ArrayBuffer> {
  if (!SAFE_METHOD.test(method)) throw new Error("invalid JSON-RPC method");
  return (body) => {
    const input = body.byteLength === 0
      ? {}
      : JSON.parse(
          decoder.decode(stripBrowserIdentityJsonRequestBody(body)),
        ) as JsonObject;
    return encoder.encode(JSON.stringify({
      jsonrpc: "2.0",
      id: crypto.randomUUID(),
      method,
      params: mapParams(input),
    }));
  };
}
