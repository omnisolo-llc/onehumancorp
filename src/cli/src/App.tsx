import React, { useState } from 'react';
import { Box, Text } from 'ink';
import { Header } from './components/Header.js';
import { AgentStatus } from './components/AgentStatus.js';
import { ToolProgress } from './components/ToolProgress.js';
import { MarkdownText } from './components/MarkdownText.js';
import { PromptInput } from './components/PromptInput.js';
import { ErrorState } from './components/ErrorState.js';
import { MasterMenu } from './components/MasterMenu.js';
import { Marketplace } from './components/Marketplace.js';
import { VisualWorkflowBuilder } from './components/VisualWorkflowBuilder.js';

import { useOrchestrator } from './hooks/useOrchestrator.js';

export const App = () => {
  const { status, tools, error, runAgent, output } = useOrchestrator();
  const [inputs, setInputs] = useState<string[]>([]);
  const [showMarketplace, setShowMarketplace] = useState(false);
  const [showVisualBuilder, setShowVisualBuilder] = useState(false);
  const markdown = `# OHC Interactive Harness\n\n- Powered by Ink\n- React in the CLI`;

  const handleSubmit = async (val: string) => {
    setInputs([...inputs, val]);
    await runAgent(val);
  };

  return (
    <Box flexDirection="column" borderStyle="round" borderColor="blue" padding={2} width={100} >
      <Header />

      {error ? (
        <ErrorState error={error} />
      ) : (
        <>
          <AgentStatus status={status} />
          <ToolProgress tools={tools} />

          <Box borderStyle="round" borderColor="gray" padding={1} marginTop={1} marginBottom={1} >
            <MarkdownText content={markdown} />
          </Box>

          {showMarketplace ? (
            <Marketplace onBack={() => setShowMarketplace(false)} />
          ) : showVisualBuilder ? (
            <VisualWorkflowBuilder onBack={() => setShowVisualBuilder(false)} />
          ) : (
            <MasterMenu onSelect={(option) => {
              if (option === 'Browse Agent Marketplace') {
                setShowMarketplace(true);
              } else if (option === 'Visual Workflow Builder') {
                setShowVisualBuilder(true);
              }
            }} />
          )}

          <Box flexDirection="column">
            {inputs.map((input, idx) => (
               <Box key={idx} marginBottom={1} flexDirection="column">
                 <Box>
                   <Text color="green">User: </Text>
                   <Text>{input}</Text>
                 </Box>
                 {idx === inputs.length - 1 && output && (
                    <Box marginTop={1} borderStyle="single" borderColor="cyan" padding={1}>
                      <Text color="cyan">Agent: </Text>
                      <MarkdownText content={output} />
                    </Box>
                 )}
               </Box>
            ))}
          </Box>

          <PromptInput onSubmit={handleSubmit} promptText="Ask Agent >" />
        </>
      )}
    </Box>
  );
};
