import { NextResponse } from 'next/server';

export async function GET() {
    return NextResponse.json({
        reviews: [
            {
                id: 'rev_1',
                customer_name: 'John D.',
                rating: 5,
                comment: 'Carlos did a great job fixing my plumbing. Fast and reliable!',
                platform: 'Google',
                ai_draft: "Hi John, thanks for trusting us with your plumbing repair! We are glad it was fixed quickly. - Carlos"
            },
            {
                id: 'rev_2',
                customer_name: 'Sarah M.',
                rating: 4,
                comment: 'The new vegan cake was amazing, highly recommend.',
                platform: 'Google',
                ai_draft: "Thank you Sarah! So happy you enjoyed the new vegan cake option. Hope to bake for you again soon! - Maya"
            }
        ]
    });
}
