"use client";
import React, { useState, useEffect } from 'react';
import WelcomeStep from '../../components/wizard/WelcomeStep';
import BusinessTypeStep from '../../components/wizard/BusinessTypeStep';
import CompanyNameStep from '../../components/wizard/CompanyNameStep';
import SellingStep from '../../components/wizard/SellingStep';
import PaymentStep from '../../components/wizard/PaymentStep';
import AdminStep from '../../components/wizard/AdminStep';
import TemplateStep from '../../components/wizard/TemplateStep';
import BrandColorsStep from '../../components/wizard/BrandColorsStep';
import FirstProductStep from '../../components/wizard/FirstProductStep';
import DomainStep from '../../components/wizard/DomainStep';
import AgentStep from '../../components/wizard/AgentStep';
import AgentScheduleStep from '../../components/wizard/AgentScheduleStep';
import PromptTuningStep from '../../components/wizard/PromptTuningStep';
import LaunchStep from '../../components/wizard/LaunchStep';
import { WizardState } from '../../components/wizard/types';

export default function WizardPage() {
    const [step, setStep] = useState<number>(0);
    const [advancedMode, setAdvancedMode] = useState<boolean>(false);
    const [launching, setLaunching] = useState<boolean>(false);
    const [launched, setLaunched] = useState<boolean>(false);


    useEffect(() => {
        const fetchState = async () => {
            const savedEmail = localStorage.getItem('wizardEmail');
            const sessionId = sessionStorage.getItem('wizardSession');
            const identifier = savedEmail || (sessionId ? `guest-${sessionId}@local` : null);
            if (identifier) {
                try {
                    const res = await fetch(`/api/onboarding/state?email=${encodeURIComponent(identifier)}`);
                    if (res.ok) {
                        const data = await res.json();
                        if (data.currentStep > 0) {
                            setState(s => ({ ...s, ...data }));
                            setStep(data.currentStep);
                        }
                    }
                } catch (e) {
                    console.error("Failed to load state", e);
                }
            }
        };
        fetchState();
    }, []);



    useEffect(() => {
        const savedAdv = sessionStorage.getItem('advancedMode');
        if (savedAdv === 'true') setAdvancedMode(true);
    }, []);

    const handleAdvancedToggle = (checked: boolean) => {
        setAdvancedMode(checked);
        sessionStorage.setItem('advancedMode', checked.toString());
    };

    const [state, setState] = useState<WizardState>({
        businessType: '', name: '', desc: '', sellingCats: [], payment: '',
        adminName: '', adminEmail: '', adminPass: '', template: '',
        colors: [], logo: '', products: [],
        domain: '', agents: [], agentSchedule: 1, agentTone: '', agentFocus: []
    });


    const persistState = async (newState: WizardState, currentStep: number) => {
        try {
            const sessionId = sessionStorage.getItem('wizardSession') || Math.random().toString(36).substring(7);
            sessionStorage.setItem('wizardSession', sessionId);
            if (newState.adminEmail) localStorage.setItem('wizardEmail', newState.adminEmail);

            const safeState = { ...newState, currentStep, adminEmail: newState.adminEmail || `guest-${sessionId}@local` };

            // Security: DO NOT persist plain-text passwords
            delete (safeState as any).adminPass;

            await fetch('/api/onboarding/state', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(safeState)
            });
        } catch (e) {
            console.error("Failed to persist state", e);
        }
    };

    const next = async () => {
        const nextStep = step + 1;
        setStep(nextStep);
        await persistState(state, nextStep);
    };


    const launch = async () => {
        setLaunching(true);
        try {
            const res = await fetch('/api/onboarding/launch', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(state)
            });
            if (!res.ok) throw new Error("Launch failed");
            setLaunched(true);
        } catch (e) {
            alert("Failed to launch business. Please try again.");
        } finally {
            setLaunching(false);
        }
    };


    const prev = () => setStep(s => Math.max(0, s - 1));
    const update = (k: keyof WizardState, v: any) => {
        setState(s => {
            const newState = { ...s, [k]: v };
            persistState(newState, step);
            return newState;
        });
    };

    const steps = [
        <WelcomeStep next={next} key="welcome" />,
        <BusinessTypeStep state={state} update={update} next={next} prev={prev} key="biztype" />,
        <CompanyNameStep state={state} update={update} next={next} prev={prev} key="name" />,
        <SellingStep state={state} update={update} next={next} prev={prev} key="sell" />,
        <PaymentStep state={state} update={update} next={next} prev={prev} key="pay" />,
        <AdminStep state={state} update={update} next={next} prev={prev} key="admin" />,
        <TemplateStep state={state} update={update} next={next} prev={prev} key="tpl" />,
        <BrandColorsStep state={state} update={update} next={next} prev={prev} key="colors" />,
        <FirstProductStep state={state} update={update} next={next} prev={prev} key="prod" />,
        <DomainStep state={state} update={update} next={next} prev={prev} key="domain" />,
        <AgentStep state={state} update={update} next={next} prev={prev} key="agent" />,
        <AgentScheduleStep state={state} update={update} next={next} prev={prev} key="schedule" />,
        <PromptTuningStep state={state} update={update} next={next} prev={prev} key="tuning" />,
        <LaunchStep state={state} launch={launch} prev={prev} launching={launching} launched={launched} key="launch" />
    ];

    return (
        <div className="min-h-screen bg-gray-900 text-white flex flex-col items-center justify-center p-8 font-sans">
            <div className="w-full max-w-4xl flex justify-end mb-4">
                <label className="flex items-center gap-2 cursor-pointer">
                    <input type="checkbox" checked={advancedMode} onChange={e => handleAdvancedToggle(e.target.checked)} className="form-checkbox h-5 w-5 text-green-500 rounded" />
                    <span className="text-gray-300">Advanced Mode</span>
                </label>
            </div>
            <div className="w-full max-w-4xl relative">
                {steps[step]}
                {advancedMode && (
                    <div className="mt-8 p-6 bg-black/50 border border-gray-700 rounded-xl font-mono text-sm text-green-400">
                        <h3 className="text-white mb-4 text-lg">Raw Config JSON (Advanced)</h3>
                        <pre>{JSON.stringify(state, null, 2)}</pre>
                        <div className="mt-4 pt-4 border-t border-gray-700 text-gray-500">
                            CLI Deploy: ohc deploy --tenant={state.name ? `"${state.name}"` : "auto"} --mode=advanced
                        </div>
                    </div>
                )}
            </div>
        </div>
    );
}