import { NextResponse } from 'next/server';

export async function POST(request: Request) {
    // Simulate processing delay for the AutoDream photo studio pipeline
    await new Promise(resolve => setTimeout(resolve, 2500));

    // For mocking purposes we just return 3 mocked URLs of generated photos
    return NextResponse.json({
        variations: [
            "https://images.unsplash.com/photo-1578985545062-69928b1d9587?w=500&h=500&fit=crop", // Pure white/clean
            "https://images.unsplash.com/photo-1614707267537-b85aaf00c4b7?w=500&h=500&fit=crop", // Pastel/Soft
            "https://images.unsplash.com/photo-1550617931-e17a7b70dce2?w=500&h=500&fit=crop"  // Marble/Premium
        ]
    });
}
