import { NextResponse } from 'next/server';

export async function POST(request: Request) {
    // Simulate processing delay for AI scene generation
    await new Promise(resolve => setTimeout(resolve, 2000));

    // Define some data URIs for placeholder images
    const variations = [
        "data:image/svg+xml;charset=UTF-8,%3Csvg xmlns='http://www.w3.org/2000/svg' width='400' height='400' viewBox='0 0 400 400'%3E%3Crect width='400' height='400' fill='%23ffffff'/%3E%3Ctext x='50%25' y='50%25' font-family='sans-serif' font-size='24' fill='%23333' text-anchor='middle' dy='.3em'%3EPure White Studio%3C/text%3E%3C/svg%3E",
        "data:image/svg+xml;charset=UTF-8,%3Csvg xmlns='http://www.w3.org/2000/svg' width='400' height='400' viewBox='0 0 400 400'%3E%3Crect width='400' height='400' fill='%23ffe4e1'/%3E%3Ctext x='50%25' y='50%25' font-family='sans-serif' font-size='24' fill='%23333' text-anchor='middle' dy='.3em'%3ESoft Pastel%3C/text%3E%3C/svg%3E",
        "data:image/svg+xml;charset=UTF-8,%3Csvg xmlns='http://www.w3.org/2000/svg' width='400' height='400' viewBox='0 0 400 400'%3E%3Crect width='400' height='400' fill='%23d3d3d3'/%3E%3Ctext x='50%25' y='50%25' font-family='sans-serif' font-size='24' fill='%23333' text-anchor='middle' dy='.3em'%3EMarble Countertop%3C/text%3E%3C/svg%3E",
        "data:image/svg+xml;charset=UTF-8,%3Csvg xmlns='http://www.w3.org/2000/svg' width='400' height='400' viewBox='0 0 400 400'%3E%3Crect width='400' height='400' fill='%23ffebcd'/%3E%3Ctext x='50%25' y='50%25' font-family='sans-serif' font-size='24' fill='%23333' text-anchor='middle' dy='.3em'%3EWarm Lighting%3C/text%3E%3C/svg%3E"
    ];

    return NextResponse.json({
        variations: variations
    });
}
