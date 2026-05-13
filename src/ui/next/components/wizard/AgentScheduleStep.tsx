import React from 'react';
import { WizardState } from './types';
export default function AgentScheduleStep({ state, update, next, prev }: { state: WizardState, update: (k: keyof WizardState, v: any) => void, next: () => void, prev: () => void }) {
    return (
        <div className="glass-panel p-8 rounded-2xl border border-gray-700">
            <h2 className="text-3xl font-bold mb-8">Schedule / frequency</h2>
            <input type="range" min="1" max="4" value={state.agentSchedule} onChange={e => update('agentSchedule', parseInt(e.target.value))} className="w-full mt-6" />
            <div className="flex justify-between mt-2 text-sm text-gray-400"><span>Real-time</span><span>Hourly</span><span>Daily</span><span>Weekly</span></div>
            <div className="flex justify-between mt-8">
                <button onClick={prev} className="px-6 py-2 border border-gray-600 rounded-lg">Back</button>
                <button onClick={next} className="px-6 py-2 bg-blue-500 rounded-lg">Next</button>
            </div>
        </div>
    );
}