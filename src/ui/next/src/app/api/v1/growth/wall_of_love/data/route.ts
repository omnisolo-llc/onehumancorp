import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  try {
    const { searchParams } = new URL(request.url);
    const store = searchParams.get('store') || 'My Store';

    // Mock data representing realistic 5-star reviews
    const reviews = [
      {
        id: "rev_1",
        author: "Jane D.",
        rating: 5,
        content: "Absolutely amazing! Best purchase I've made this year from " + store + ".",
        date: "Oct 12, 2023"
      },
      {
        id: "rev_2",
        author: "John S.",
        rating: 5,
        content: "Incredible quality and super fast shipping. Highly recommend!",
        date: "Nov 05, 2023"
      },
      {
        id: "rev_3",
        author: "Sarah L.",
        rating: 5,
        content: "I'm blown away by the customer service. They truly care about their customers.",
        date: "Dec 01, 2023"
      },
      {
        id: "rev_4",
        author: "Michael B.",
        rating: 5,
        content: "Exactly what I was looking for. Perfect fit and great design.",
        date: "Jan 15, 2024"
      }
    ];

    return NextResponse.json({ reviews });
  } catch (error) {
    console.error("Error fetching Wall of Love data:", error);
    return NextResponse.json(
      { error: "Failed to fetch reviews" },
      { status: 500 }
    );
  }
}
