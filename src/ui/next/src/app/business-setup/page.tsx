'use client';
import React, { useState, useEffect } from 'react';
import { Step0Welcome } from '../../components/wizard/Step0Welcome';
import { Step1Type } from '../../components/wizard/Step1Type';
import { Step2Name } from '../../components/wizard/Step2Name';
import { Step3Products } from '../../components/wizard/Step3Products';
import { Step4Payments } from '../../components/wizard/Step4Payments';
import { Step5Account } from '../../components/wizard/Step5Account';
import { Step6Template } from '../../components/wizard/Step6Template';
import { Step7ProductAdd } from '../../components/wizard/Step7ProductAdd';
import { Step8Domain } from '../../components/wizard/Step8Domain';
import { Step9Review } from '../../components/wizard/Step9Review';
import { Step10Success } from '../../components/wizard/Step10Success';

export default function BusinessSetup() {
  const [step, setStep] = useState(0);

  useEffect(() => {
    fetch('/api/onboarding/state')
      .then(res => res.json())
      .then(data => {
         if (data && data.state && data.state.current_step) {
            setStep(data.state.current_step);
         }
      })
      .catch(console.error);
  }, []);

  const handleNext = (nextStep: number) => {
    setStep(nextStep);
    fetch('/api/onboarding/state', {
       method: 'POST',
       headers: { 'Content-Type': 'application/json' },
       body: JSON.stringify({
           state: {
               organization_id: 'local-test',
               current_step: nextStep,
               state_json: '{}'
           }
       })
    }).catch(console.error);
  };

  const steps = [
     <Step0Welcome onNext={() => handleNext(1)} />,
     <Step1Type onNext={() => handleNext(2)} />,
     <Step2Name onNext={() => handleNext(3)} />,
     <Step3Products onNext={() => handleNext(4)} />,
     <Step4Payments onNext={() => handleNext(5)} />,
     <Step5Account onNext={() => handleNext(6)} />,
     <Step6Template onNext={() => handleNext(7)} />,
     <Step7ProductAdd onNext={() => handleNext(8)} />,
     <Step8Domain onNext={() => handleNext(9)} />,
     <Step9Review onNext={() => handleNext(10)} />,
     <Step10Success onNext={() => handleNext(11)} />,
  ];

  if (step < 11) {
     return steps[step];
  }

  return (
    <div className="p-8">
      <h2>Welcome Checklist</h2>
      <p>You're set up! Here's what to do next:</p>
      <ul><li>✅ Business live</li><li><input type="checkbox"/> Add 3 more products</li><li><input type="checkbox"/> Connect Instagram</li><li><input type="checkbox"/> Share your link with a friend</li></ul>
      <button className="bg-blue-600 text-white p-2 rounded mt-4">Go to Dashboard →</button>
    </div>
  );
}
