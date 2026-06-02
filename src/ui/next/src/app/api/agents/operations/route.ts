import { NextResponse } from 'next/server';

export async function POST(request: Request) {
    try {
        const body = await request.json();
        const { message } = body;

        if (!message) {
             return NextResponse.json({ error: "Message is required" }, { status: 400 });
        }

        // Simulate processing delay for the AutoDream pipeline
        await new Promise(resolve => setTimeout(resolve, 1500));

        let price = "10.00";
        let matchPrice = message.match(/\$(\d+(\.\d{1,2})?)/);
        if (matchPrice) {
            price = matchPrice[1];
        }

        let name = "New Catalog Item";
        if (message.toLowerCase().includes("cupcake")) {
             name = "Vanilla Cupcakes (Dozen)";
        } else if (message.toLowerCase().includes("cake")) {
             name = "Custom Cake";
        } else if (message.toLowerCase().includes("repair") || message.toLowerCase().includes("fix")) {
             name = "General Repair Service";
        } else if (message.length > 5) {
             name = message.substring(0, 30);
        }

        let category = "Product";
        if (message.toLowerCase().includes("service") || message.toLowerCase().includes("repair")) {
             category = "Service";
        }

        return NextResponse.json({
            name: name,
            description: "Automatically extracted from your message: " + message,
            price: price,
            category: category
        });
    } catch(e) {
        return NextResponse.json({ error: "Failed to process message" }, { status: 500 });
    }
}
