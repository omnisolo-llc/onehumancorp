import { NextResponse } from 'next/server';
import { backendCatchAll } from "../../../../backendCatchAll";

export async function POST(req: Request) {
    return backendCatchAll(req);
}
