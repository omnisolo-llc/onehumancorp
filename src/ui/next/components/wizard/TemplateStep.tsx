import React from 'react';
import { WizardState } from './types';
export default function TemplateStep({ state, update, next, prev }: { state: WizardState, update: (k: keyof WizardState, v: string) => void, next: () => void, prev: () => void }) {
    const tpls = ['Modern', 'Classic', 'Bold'];
    return (
        <div className="glass-panel p-8 rounded-2xl border border-gray-700">
            <h2 className="text-3xl font-bold mb-8">Template gallery</h2>
            <div className="grid grid-cols-3 gap-6 mb-8">
                {tpls.map(t => (
                    <div key={t} className={`border rounded-xl p-4 cursor-pointer transition-all ${state.template === t ? 'border-green-500 bg-green-500/10 scale-105' : 'border-gray-600 hover:border-gray-400'}`} onClick={() => update('template', t)}>
                        <div className="h-32 bg-gray-800 mb-4 rounded-lg"></div>
                        <h3 className="text-center font-bold">Use this template {t}</h3>
                    </div>
                ))}
            </div>
            <div className="flex justify-between">
                <button onClick={prev} className="px-6 py-2 border border-gray-600 rounded-lg">Back</button>
                <button onClick={next} disabled={!state.template} className="px-6 py-2 bg-blue-500 rounded-lg disabled:opacity-50">Next</button>
            </div>
        </div>
    );
}