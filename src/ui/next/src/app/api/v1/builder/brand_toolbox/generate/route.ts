import { NextResponse, NextRequest } from 'next/server';

export async function POST(request: NextRequest) {
  try {
    const mockToolbox = {
        id: "mock_toolbox_id",
        brand_dna: {
            name: "Luna Loaf",
            positioning: "A local bakery.",
            colors: ["#FFFFFF", "#000000"]
        },
        brand_book: [
            { title: "Voice", guidance: ["Friendly"] }
        ],
        logo_concepts: [
            { title: "Main", svg: "<svg></svg>", usage_notes: ["Use everywhere"] }
        ],
        catalog: [
            { name: "Cake", price: "$10", description: "Yum", seo_title: "cake" }
        ],
        campaign_ideas: [
            { title: "Summer", hook: "Hot", channels: ["Instagram"] }
        ],
        social_calendar: [
            { day: "Monday", channel: "Twitter", caption: "Hello" }
        ],
        assets: [
            { asset_type: "Image", channel: "Facebook", title: "Promo", copy: "Buy now" }
        ],
        photoshoot: {
            product_source: "Real",
            shots: [
                { title: "Hero", mockup_svg: "<svg></svg>", format: "16:9", usage: "Web", prompt: "A cake" }
            ]
        },
        store_profile: {
            pages: [
                { blocks: [{}] }
            ]
        }
    };
    return NextResponse.json(mockToolbox);
  } catch (e) {
    return NextResponse.json({}, { status: 500 });
  }
}
