export async function POST(request: Request) {
  try {
    const payload = await request.json().catch(() => ({}));
    const customer = (payload && payload.customer_name) ? payload.customer_name : "there";
    const product = (payload && payload.product_name) ? payload.product_name : "Signature Coffee Blend";
    const orderId = (payload && payload.order_id) ? payload.order_id : "12345";
    return new Response(
      JSON.stringify({
        message: `Hi ${customer}! Thank you for ordering ${product} (Order #${orderId}). We'd love your review: https://cloud.omnisolo.co/review/${orderId} ⚡ OmniSolo`,
      }),
      {
        status: 200,
        headers: { "Content-Type": "application/json" },
      },
    );
  } catch {
    return new Response(
      JSON.stringify({ message: "Review campaign generation is unavailable." }),
      {
        status: 503,
        headers: { "Content-Type": "application/json" },
      },
    );
  }
}
