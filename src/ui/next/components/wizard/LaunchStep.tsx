import React from 'react';
import { WizardState } from './types';
export default function LaunchStep({ state, launch, prev, launching, launched }: { state: WizardState, launch: () => void, prev: () => void, launching: boolean, launched: boolean }) {
    if (launched) {
        return (
            <div className="glass-panel p-12 text-center rounded-2xl border border-green-500 bg-green-500/10">
                <h1 className="text-5xl font-bold mb-6 text-green-400">Onboarding Complete!</h1>
                <p className="text-xl text-white mb-8">Your business "{state.name}" is now live.</p>
                <div className="text-left bg-black/30 p-6 rounded-xl inline-block text-gray-300">
                    <p>Dashboard URL: <code>{state.name.toLowerCase().replace(/\s+/g, '-')}.ohc.app</code></p>
                </div>
            </div>
        );
    }
    return (
        <div className="glass-panel p-12 text-center rounded-2xl border border-gray-700">
            <h1 className="text-5xl font-bold mb-6 outfit-font">Review & Launch</h1>
            <div className="text-left bg-gray-800 p-6 rounded-xl mb-8 border border-gray-700">
                <h3 className="text-xl font-bold mb-4 text-green-400">Business Summary</h3>
                <p className="mb-2"><span className="text-gray-400 w-32 inline-block">Name:</span> <span className="font-semibold">{state.name}</span></p>
                <p className="mb-2"><span className="text-gray-400 w-32 inline-block">Type:</span> <span className="font-semibold">{state.businessType}</span></p>
                <p className="mb-2"><span className="text-gray-400 w-32 inline-block">Template:</span> <span className="font-semibold">{state.template} Theme</span></p>
                <p className="mb-2"><span className="text-gray-400 w-32 inline-block">Payment:</span> <span className="font-semibold">{state.payment}</span></p>
                <p className="mb-2"><span className="text-gray-400 w-32 inline-block">Agents:</span> <span className="font-semibold">{state.agents.length > 0 ? state.agents.join(', ') : 'None selected'}</span></p>
            </div>
            <button onClick={launch} disabled={launching} className="bg-green-500 hover:bg-green-600 text-white px-8 py-4 rounded-xl text-xl font-bold transition-all w-full mb-4 shadow-lg shadow-green-500/20 disabled:opacity-50">
                {launching ? 'Provisioning your tenant...' : 'Launch My Business ->'}
            </button>
            <button onClick={prev} disabled={launching} className="w-full px-6 py-4 border border-gray-600 rounded-xl mt-2 hover:bg-gray-800 disabled:opacity-50">Back to Editing</button>
        </div>
    );
}