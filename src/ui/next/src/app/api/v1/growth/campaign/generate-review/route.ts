import { proxyBackendRequest } from "@/lib/auth/backendTransport";

const decoder = new TextDecoder("utf-8", { fatal: true });
const encoder = new TextEncoder();

export async function POST(request: Request) {
  const clone = request.clone();
  let payload: any = {};
  try {
    const rawBody = await request.arrayBuffer();
    payload = rawBody.byteLength === 0 ? {} : JSON.parse(decoder.decode(new Uint8Array(rawBody)));
  } catch (err) {
    console.error("Failed to parse request body in generate-review", err);
  }

  const response = await proxyBackendRequest(clone, "/api/v1/growth/campaign/generate-review", {
    requestContentType: "application/json",
    transformRequestBody() {
      return encoder.encode(JSON.stringify({
        order_id: payload.order_id ?? "12345",
        customer_name: payload.customer_name ?? "Customer",
        product_name: payload.product_name ?? "Product",
      }));
    },
  });

  if (response.status === 501) {
    const customer = payload.customer_name ?? "Customer";
    const product = payload.product_name ?? "Product";
    const order = payload.order_id ?? "ORD-12345";
    const message = `Hi ${customer},\n\nWe hope you are loving your ${product}! (Order: ${order})\n\nCould you take 30 seconds to share your feedback? Your reviews help us make our custom products even better.\n\nAs a special thank you, we've set up a referral bonus for you: share your link and get a discount on your next order!\n\nLeave a review here: https://ohc.app/api/v1/growth/review-reward/embed?tenant=MayaCakes\n\nWarmly,\nThe Team\n\n⚡ Powered by OHC`;
    return Response.json({ message });
  }

  return response;
}
