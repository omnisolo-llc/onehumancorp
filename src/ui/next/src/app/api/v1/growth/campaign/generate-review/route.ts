export async function POST(request: Request) {
  try {
    const payload = await request.json().catch(() => ({}));
    const product = (payload && payload.product_name) ? payload.product_name : "Signature Coffee Blend";
    return new Response(
      JSON.stringify({
        message: `Hi there! Thank you for ordering ${product}. We'd love your review: https://cloud.omnisolo.co/review ⚡ OmniSolo`,
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
