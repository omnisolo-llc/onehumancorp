import { NextResponse } from 'next/server';
import * as grpc from '@grpc/grpc-js';
import * as protoLoader from '@grpc/proto-loader';
import path from 'path';

export async function GET(req: Request) {
    // Attempting to mock the response or hit an actual grpc endpoint
    // Since there isn't a robust grpc server for Nextjs set up yet we will still return mock data
    // to keep the frontend functioning until the backend is fully deployed with real data

    const now = new Date();
    const startOfMonth = new Date(now.getFullYear(), now.getMonth(), 1);
    const endOfMonth = new Date(now.getFullYear(), now.getMonth() + 1, 0);

    const mockData = {
        total_revenue: 12000,
        total_costs: 1450,
        llm_cost: 850,
        storage_cost: 300,
        payment_fees: 300,
        period_start: startOfMonth.toLocaleDateString('en-CA'),
        period_end: endOfMonth.toLocaleDateString('en-CA'),
    };

    return NextResponse.json(mockData);
}
