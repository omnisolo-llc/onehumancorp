import { NextResponse } from 'next/server';
import fs from 'fs';

const STATE_FILE = '/tmp/onboarding_state.json';

export async function GET() {
    try {
        if (fs.existsSync(STATE_FILE)) {
            const data = fs.readFileSync(STATE_FILE, 'utf8');
            return NextResponse.json({ state: JSON.parse(data) });
        }
        return NextResponse.json({ state: { current_step: 0, state_json: '{}' } });
    } catch (e) {
        return NextResponse.json({ state: { current_step: 0, state_json: '{}' } });
    }
}

export async function POST(request: Request) {
    try {
        const body = await request.json();
        let currentState: any = { current_step: 0, state_json: '{}' };
        if (fs.existsSync(STATE_FILE)) {
            try { currentState = JSON.parse(fs.readFileSync(STATE_FILE, 'utf8')); } catch (e) {}
        }

        if (body?.state?.current_step !== undefined) {
            currentState.current_step = body.state.current_step;
        }
        if (body?.state?.state_json !== undefined) {
            currentState.state_json = body.state.state_json;
        }

        fs.writeFileSync(STATE_FILE, JSON.stringify(currentState), 'utf8');
        return NextResponse.json({ success: true, state: currentState });
    } catch (e) {
        return NextResponse.json({ success: false }, { status: 400 });
    }
}
