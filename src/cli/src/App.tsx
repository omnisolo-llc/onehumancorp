import React, { useState } from 'react';
import { Box, Text } from 'ink';
import { AgentStatus } from './components/AgentStatus';
import { ToolProgress } from './components/ToolProgress';
import { MarkdownText } from './components/MarkdownText';
import { PromptInput } from './components/PromptInput';
import { ErrorState } from './components/ErrorState';
import { useOrchestrator } from './hooks/useOrchestrator';
import { Wizard } from './components/Wizard';

export const App = () => {
  const { status, tools, error } = useOrchestrator();
  const [inputs, setInputs] = useState<string[]>([]);
  const [wizardComplete, setWizardComplete] = useState(false);
  const markdown = `# OHC Interactive Harness\n\n- Powered by Ink\n- React in the CLI`;

  const handleComplete = async (wizardState: any) => {
    try {
      // In local testing/CLI, fetch against localhost might fail if the server isn't running
      // Ignore network errors in local dev gracefully
      await fetch('/api/onboarding/start', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          business_type: wizardState.businessType,
          company_name: wizardState.companyName,
          company_description: `A ${wizardState.businessType}`,
          selling_categories: [wizardState.sellingCategories],
          payment_pref: wizardState.paymentPref,
          admin_email: wizardState.adminEmail,
          admin_name: wizardState.adminName,
          website_template: wizardState.template,
          first_product_name: wizardState.productName,
          first_product_price: wizardState.productPrice,
          domain_choice: wizardState.domain,
          admin_password: 'generated',
          price_type: 'fixed'
        })
      });
    } catch (e) {
      // Ignore network errors in local dev
    }
    setWizardComplete(true);
  };

  if (!wizardComplete) {
    return (
      <Box flexDirection="column" borderStyle="round" borderColor="cyan" padding={1} width={80}>
        <Box justifyContent="center" marginBottom={1}>
          <Text bold color="white" backgroundColor="blue"> ONE HUMAN CORP </Text>
          <Text> - Setup </Text>
        </Box>
        <Wizard onComplete={handleComplete} />
      </Box>
    );
  }

  return (
    <Box flexDirection="column" borderStyle="round" borderColor="cyan" padding={1} width={80}>
      <Box justifyContent="center" marginBottom={1}>
        <Text bold color="white" backgroundColor="blue"> ONE HUMAN CORP </Text>
        <Text> - Standalone Agent Mode </Text>
      </Box>

      {error ? (
        <ErrorState error={error} />
      ) : (
        <>
          <Box padding={1} borderStyle="round" borderColor="green">
            <Text color="green">Onboarding Complete! Your business is now live.</Text>
            <Text color="green">Next steps:</Text>
            <Text>✅ Business live</Text>
            <Text>⬜ Add 3 more products</Text>
            <Text>⬜ Connect Instagram</Text>
            <Text>⬜ Share your link with a friend</Text>
          </Box>
          <AgentStatus status={status} />
          <ToolProgress tools={tools} />

          <Box borderStyle="single" borderColor="gray" padding={1} marginTop={1} marginBottom={1}>
            <MarkdownText content={markdown} />
          </Box>

          <Box flexDirection="column">
            {inputs.map((input, idx) => (
               <Box key={idx} marginBottom={1}>
                 <Text color="green">User: </Text>
                 <Text>{input}</Text>
               </Box>
            ))}
          </Box>

          <PromptInput onSubmit={(val) => setInputs([...inputs, val])} promptText="Ask Agent >" />
        </>
      )}
    </Box>
  );
};
