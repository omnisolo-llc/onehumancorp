import React, { useState } from 'react';
import { Box, Text } from 'ink';
import { Header } from './components/Header';
import { AgentStatus } from './components/AgentStatus';
import { ToolProgress } from './components/ToolProgress';
import { MarkdownText } from './components/MarkdownText';
import { PromptInput } from './components/PromptInput';
import { ErrorState } from './components/ErrorState';
import { MasterMenu } from './components/MasterMenu';
import { WorkflowBuilder } from './components/WorkflowBuilder';

import { useOrchestrator } from './hooks/useOrchestrator';

export const App = () => {
  const { status, tools, error } = useOrchestrator();
  const [inputs, setInputs] = useState<string[]>([]);
  const [activeView, setActiveView] = useState<'menu' | 'workflow_builder'>('menu');
  const markdown = `# OHC Interactive Harness\n\n- Powered by Ink\n- React in the CLI`;

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

          {activeView === 'menu' ? (
            <MasterMenu onSelect={(option) => {
              if (option === 'Build Visual Workflow') {
                setActiveView('workflow_builder');
              }
            }} />
          ) : (
            <WorkflowBuilder
              onBack={() => setActiveView('menu')}
              onRun={async (payload) => {
                try {
                  const API_BASE = process.env.API_URL || 'http://localhost:18789';
                  const response = await fetch(`${API_BASE}/api/workflow/run`, {
                    method: 'POST',
                    headers: {
                      'Content-Type': 'application/json',
                    },
                    body: JSON.stringify({
                      graph: {
                        nodes: Object.values(payload.nodes).map((n: any) => ({
                          id: n.id,
                          node_type: n.id === payload.entrypoint
                            ? { Input: { name: "input_var" } }
                            : n.next.length === 0
                              ? "Output"
                              : { Llm: { prompt_template: "Execute block: " + n.label } }
                        })),
                        edges: Object.values(payload.nodes).flatMap((n: any) =>
                          n.next.map((target: string) => ({ source: n.id, target }))
                        )
                      },
                      inputs: { input_var: "Start workflow execution" }
                    }),
                  });

                  if (!response.ok) {
                    throw new Error(`API error: ${response.status}`);
                  }

                  const data = await response.json();
                  setInputs([...inputs, `Workflow finished: ${data.result || JSON.stringify(data)}`]);
                } catch (e: any) {
                  setInputs([...inputs, `Workflow failed: ${e.message}`]);
                }
                setActiveView('menu');
              }}
            />
          )}

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
