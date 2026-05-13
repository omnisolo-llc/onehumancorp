import React from 'react';
import { WizardState } from './types';

export default function CompanyNameStep({ state, update, next, prev }: { state: WizardState, update: (k: keyof WizardState, v: string) => void, next: () => void, prev: () => void }) {
    const handleNameBlur = async () => {
        if (state.name && !state.desc) {
            // AI Auto-suggest description based on business name
            update('desc', `Premium ${state.businessType || 'services'} by ${state.name}.`);
        }
    };

    return (
        <div className="glass-panel p-8 rounded-2xl border border-gray-700">
            <h2 className="text-3xl font-bold mb-8">What is your business called?</h2>
            <input type="text" value={state.name} onChange={e => update('name', e.target.value)} onBlur={handleNameBlur} placeholder="e.g. Maya's Cakes" className="w-full p-4 bg-gray-800 border border-gray-600 rounded-xl text-xl mb-4" />
            <textarea value={state.desc} onChange={e => update('desc', e.target.value)} placeholder="Description..." className="w-full p-4 bg-gray-800 border border-gray-600 rounded-xl text-lg mb-8 h-32" />
            <div className="flex justify-between">
                <button onClick={prev} className="px-6 py-2 border border-gray-600 rounded-lg">Back</button>
                <button onClick={next} disabled={!state.name} className="px-6 py-2 bg-blue-500 rounded-lg disabled:opacity-50">Next</button>
            </div>
        </div>
    );
}