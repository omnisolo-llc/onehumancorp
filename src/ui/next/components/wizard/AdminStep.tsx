import React from 'react';
import { WizardState } from './types';
export default function AdminStep({ state, update, next, prev }: { state: WizardState, update: (k: keyof WizardState, v: string) => void, next: () => void, prev: () => void }) {
    return (
        <div className="glass-panel p-8 rounded-2xl border border-gray-700">
            <h2 className="text-3xl font-bold mb-8">Administrator account</h2>
            <div className="flex flex-col gap-4 mb-8">
                <input type="text" value={state.adminName} onChange={e => update('adminName', e.target.value)} placeholder="Name" className="w-full p-4 bg-gray-800 border border-gray-600 rounded-xl" />
                <input type="email" value={state.adminEmail} onChange={e => update('adminEmail', e.target.value)} placeholder="you@email.com" className="w-full p-4 bg-gray-800 border border-gray-600 rounded-xl" />
                <input type="password" value={state.adminPass} onChange={e => update('adminPass', e.target.value)} placeholder="Password" className="w-full p-4 bg-gray-800 border border-gray-600 rounded-xl" />
            </div>
            <div className="flex justify-between">
                <button onClick={prev} className="px-6 py-2 border border-gray-600 rounded-lg">Back</button>
                <button onClick={next} disabled={!state.adminName || !state.adminEmail || !state.adminPass} className="px-6 py-2 bg-blue-500 rounded-lg disabled:opacity-50">Next</button>
            </div>
        </div>
    );
}