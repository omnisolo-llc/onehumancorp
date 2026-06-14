import React, { useState } from 'react';
import { Box, Text } from 'ink';
import { Header } from './components/Header';
import { AgentStatus } from './components/AgentStatus';
import { ToolProgress } from './components/ToolProgress';
import { MarkdownText } from './components/MarkdownText';
import { PromptInput } from './components/PromptInput';
import { ErrorState } from './components/ErrorState';
import { MasterMenu } from './components/MasterMenu';

import { useOrchestrator } from './hooks/useOrchestrator';

export const App = () => {
  const { status, tools, error, runAgent, output } = useOrchestrator();
  const [inputs, setInputs] = useState<string[]>([]);
  const markdown = `# OHC Interactive Harness\n\n- Powered by Ink\n- React in the CLI`;

  const handleSubmit = async (val: string) => {
    setInputs([...inputs, val]);
    await runAgent(val);
  };

  return (
    <Box flexDirection="column" borderStyle="round" borderColor="blue" padding={2} width={100} dimColor>
      <Header />

      {error ? (
        <ErrorState error={error} />
      ) : (
        <>
          <AgentStatus status={status} />
          <ToolProgress tools={tools} />

          <Box borderStyle="round" borderColor="gray" padding={1} marginTop={1} marginBottom={1} dimColor>
            <MarkdownText content={markdown} />
          </Box>

          <MasterMenu />

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
