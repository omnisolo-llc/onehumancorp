import React from 'react';
import { WizardState } from './types';
export default function BusinessTypeStep({ state, update, next, prev }: { state: WizardState, update: (k: keyof WizardState, v: string) => void, next: () => void, prev: () => void }) {
    const types = ['Online Store', 'Service Business', 'Restaurant / Food', 'Creative / Portfolio', 'Local Business', 'Other'];
    return (
        <div className="glass-panel p-8 rounded-2xl border border-gray-700">
            <h2 className="text-3xl font-bold mb-8">What kind of business are you building?</h2>
            <div className="grid grid-cols-2 gap-4 mb-8">
                {types.map(t => (
                    <button key={t} onClick={() => update('businessType', t)} className={`p-6 border rounded-xl text-left text-lg ${state.businessType === t ? 'border-green-500 bg-green-500/10' : 'border-gray-600 hover:border-gray-400'}`}>
                        {t}
                    </button>
                ))}
            </div>
            <div className="flex justify-between">
                <button onClick={prev} className="px-6 py-2 border border-gray-600 rounded-lg">Back</button>
                <button onClick={next} disabled={!state.businessType} className="px-6 py-2 bg-blue-500 rounded-lg disabled:opacity-50">Next</button>
            </div>
        </div>
    );
}