import React from 'react';
import { WizardState } from './types';
export default function PromptTuningStep({ state, update, next, prev }: { state: WizardState, update: (k: keyof WizardState, v: any) => void, next: () => void, prev: () => void }) {
    return (
        <div className="glass-panel p-8 rounded-2xl border border-gray-700 flex gap-8">
            <div className="flex-1">
                <h2 className="text-3xl font-bold mb-8">Tune your agent</h2>
                <h3 className="text-xl mb-4">Tone</h3>
                <select value={state.agentTone} onChange={e => update('agentTone', e.target.value)} className="w-full p-4 bg-gray-800 rounded text-white">
                    <option value="">Select Tone...</option>
                    <option value="Friendly & Warm">Friendly & Warm</option>
                    <option value="Professional">Professional</option>
                    <option value="Energetic">Energetic</option>
                    <option value="Concise">Concise</option>
                </select>
                <h3 className="text-xl mt-6 mb-4">Focus topics</h3>
                <div className="flex gap-2 flex-wrap">
                    {["Only about my products", "Avoid competitor mentions", "Always reply in language"].map(t => (
                        <button key={t} className={`p-2 border rounded ${state.agentFocus.includes(t) ? 'bg-green-500' : 'bg-transparent'}`} onClick={() => update('agentFocus', state.agentFocus.includes(t) ? state.agentFocus.filter(x => x !== t) : [...state.agentFocus, t])}>{t}</button>
                    ))}
                </div>
            </div>

            <div className="flex-1 border-l border-gray-700 pl-8 flex flex-col">
                <h3 className="text-xl mb-4">Live Preview (Chat Sandbox)</h3>
                <div className="flex-1 bg-gray-800 rounded-xl mb-4 p-4 font-mono text-sm overflow-y-auto">
                    <div className="text-gray-400">System Prompt Updated...</div>
                    <div className="text-green-400 mt-2">Agent: Hello! I am tuned to be {state.agentTone || 'helpful'}.</div>
                </div>
                <input type="text" placeholder="Test your agent..." className="w-full p-4 bg-gray-900 border border-gray-600 rounded-xl" />
            </div>
            <div className="flex flex-col justify-end ml-8">

                <button onClick={prev} className="px-6 py-2 border border-gray-600 rounded-lg mb-2">Back</button>
                <button onClick={next} className="px-6 py-2 bg-blue-500 rounded-lg">Next</button>
            </div>
        </div>
    );
}