import React from 'react';
import { WizardState } from './types';
export default function AgentStep({ state, update, next, prev }: { state: WizardState, update: (k: keyof WizardState, v: string[]) => void, next: () => void, prev: () => void }) {
    const agents = ['Customer Support', 'Social Media Manager', 'SEO Booster', 'Order Manager', 'Email Marketer'];
    return (
        <div className="glass-panel p-8 rounded-2xl border border-gray-700">
            <h2 className="text-3xl font-bold mb-8">Manage my AI team</h2>
            <div className="flex flex-col gap-4 mb-8">
                {agents.map(a => (
                    <label key={a} className="flex items-center gap-4 p-4 border border-gray-600 rounded-xl cursor-pointer hover:bg-gray-800">
                        <input type="checkbox" checked={state.agents.includes(a)} onChange={e => {
                            const newA = e.target.checked ? [...state.agents, a] : state.agents.filter(x => x !== a);
                            update('agents', newA);
                        }} className="w-6 h-6 rounded" />
                        <span className="text-lg">{a}</span>
                    </label>
                ))}
            </div>
            <div className="flex justify-between">
                <button onClick={prev} className="px-6 py-2 border border-gray-600 rounded-lg">Back</button>
                <button onClick={next} className="px-6 py-2 bg-blue-500 rounded-lg">Next</button>
            </div>
        </div>
    );
}