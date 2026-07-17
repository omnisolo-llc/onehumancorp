import { proxyBackendRequest } from "@/lib/auth/backendTransport";

const decoder = new TextDecoder("utf-8", { fatal: true });
const encoder = new TextEncoder();

export async function POST(request: Request) {
  return proxyBackendRequest(request, "/api/v1/growth/campaign/generate-review", {
    requestContentType: "application/json",
    transformRequestBody(body) {
      const payload = body.byteLength === 0 ? {} : JSON.parse(decoder.decode(body));
      return encoder.encode(JSON.stringify({
        order_id: payload.order_id ?? "12345",
        customer_name: payload.customer_name ?? "Customer",
        product_name: payload.product_name ?? "Product",
      }));
    },
  });
}
