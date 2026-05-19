import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  const { bio } = await req.json();

  // Here we would typically call the Promoter agent via JSON-RPC or similar.
  // For now, we simulate the agent's response.

  const blocks = [
    {
      type: "Hero",
      props: {
        headline: bio.includes("dog") ? "Premium Mobile Dog Grooming" : "Your Local Business",
        image: "https://images.unsplash.com/photo-1516734212186-a967f81ad0d7?auto=format&fit=crop&w=400&q=80",
        copy: "We bring the salon to you. Stress-free grooming for your furry friends."
      }
    },
    {
      type: "Catalog",
      props: {
        items: [
          { name: "Full Groom", price: "$80", description: "Bath, haircut, nail trim, and ear cleaning." },
          { name: "Bath & Brush", price: "$45", description: "Deep cleaning bath and thorough brush out." }
        ]
      }
    },
    {
      type: "Booking",
      props: {
        title: "Book an Appointment",
        availability: "Next available: Tomorrow at 2 PM"
      }
    },
    {
      type: "Contact",
      props: {
        email: "hello@example.com",
        phone: "(555) 123-4567"
      }
    },
    {
      type: "FooterBranding",
      props: {
        refId: "automated_builder"
      }
    }
  ];

  return NextResponse.json({ blocks, theme: "light" });
}
