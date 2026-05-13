import React, { useState } from 'react';
import { Box, Text } from 'ink';
import { PromptInput } from './PromptInput';

type WizardState = {
  businessType: string;
  companyName: string;
  sellingCategories: string;
  productName: string;
  productPrice: string;
  paymentPref: string;
  template: string;
  domain: string;
  adminName: string;
  adminEmail: string;
};

export const Wizard = ({ onComplete }: { onComplete: (state: WizardState) => void }) => {
  const [step, setStep] = useState(0);
  const [state, setState] = useState<WizardState>({
    businessType: '',
    companyName: '',
    sellingCategories: '',
    productName: '',
    productPrice: '',
    paymentPref: '',
    template: '',
    domain: '',
    adminName: '',
    adminEmail: ''
  });

  const nextStep = () => setStep(s => s + 1);

  if (step === 0) {
    return (
      <Box flexDirection="column" padding={1} borderStyle="round" borderColor="yellow">
        <Text color="yellow">Your business, live in minutes.</Text>
        <Text>Welcome to the Business Setup Wizard!</Text>
        <PromptInput onSubmit={() => nextStep()} promptText="Type 'Start My Business' to begin >" />
      </Box>
    );
  }

  if (step === 1) {
    return (
      <Box flexDirection="column" padding={1}>
        <Text color="green">What kind of business are you building?</Text>
        <PromptInput onSubmit={(val) => { setState({...state, businessType: val}); nextStep(); }} promptText="e.g. Online Store >" />
      </Box>
    );
  }

  if (step === 2) {
    return (
      <Box flexDirection="column" padding={1}>
        <Text color="green">What is your business called?</Text>
        <PromptInput onSubmit={(val) => { setState({...state, companyName: val}); nextStep(); }} promptText="e.g. Maya's Cakes >" />
      </Box>
    );
  }

  if (step === 3) {
    return (
      <Box flexDirection="column" padding={1}>
        <Text color="green">What do you sell?</Text>
        <PromptInput onSubmit={(val) => { setState({...state, sellingCategories: val}); nextStep(); }} promptText="e.g. Physical products >" />
      </Box>
    );
  }

  if (step === 4) {
    return (
      <Box flexDirection="column" padding={1}>
        <Text color="green">Add your first product</Text>
        <PromptInput onSubmit={(val) => { setState({...state, productName: val}); nextStep(); }} promptText="Product name >" />
      </Box>
    );
  }

  if (step === 5) {
    return (
      <Box flexDirection="column" padding={1}>
        <Text color="green">What is the price of this product?</Text>
        <PromptInput onSubmit={(val) => { setState({...state, productPrice: val}); nextStep(); }} promptText="Price >" />
      </Box>
    );
  }

  if (step === 6) {
    return (
      <Box flexDirection="column" padding={1}>
        <Text color="green">How do you want to receive payments?</Text>
        <PromptInput onSubmit={(val) => { setState({...state, paymentPref: val}); nextStep(); }} promptText="e.g. Online only >" />
      </Box>
    );
  }

  if (step === 7) {
    return (
      <Box flexDirection="column" padding={1}>
        <Text color="green">Choose a Template</Text>
        <PromptInput onSubmit={(val) => { setState({...state, template: val}); nextStep(); }} promptText="e.g. Modern >" />
      </Box>
    );
  }

  if (step === 8) {
    return (
      <Box flexDirection="column" padding={1}>
        <Text color="green">Choose a Domain</Text>
        <PromptInput onSubmit={(val) => { setState({...state, domain: val}); nextStep(); }} promptText="e.g. Free OHC Domain >" />
      </Box>
    );
  }

  if (step === 9) {
    return (
      <Box flexDirection="column" padding={1}>
        <Text color="green">Administrator account - Name</Text>
        <PromptInput onSubmit={(val) => { setState({...state, adminName: val}); nextStep(); }} promptText="e.g. Maya Smith >" />
      </Box>
    );
  }

  if (step === 10) {
    return (
      <Box flexDirection="column" padding={1}>
        <Text color="green">Administrator account - Email</Text>
        <PromptInput onSubmit={(val) => {
          const newState = {...state, adminEmail: val};
          setState(newState);
          onComplete(newState);
        }} promptText="e.g. you@email.com >" />
      </Box>
    );
  }

  return null;
};
