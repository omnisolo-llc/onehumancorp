import React, { useState } from 'react';
import { Box, Text, useInput } from 'ink';
import Spinner from 'ink-spinner';
import { useVisualWorkflow } from '../hooks/useVisualWorkflow.js';

interface VisualWorkflowEditorProps {
  onBack: () => void;
}

export const VisualWorkflowEditor: React.FC<VisualWorkflowEditorProps> = ({ onBack }) => {
  const { result, loading, error, runWorkflow } = useVisualWorkflow();
  const [selectedWorkflow, setSelectedWorkflow] = useState(0);

  const predefinedWorkflows = [
    {
      name: "Simple Linear Workflow (Input -> LLM -> Output)",
      graph: {
        nodes: [
          { id: "in", node_type: { Input: { name: "input_var" } } },
          { id: "llm1", node_type: { Llm: { prompt_template: "Process this: {{in}}" } } },
          { id: "out", node_type: "Output" }
        ],
        edges: [
          { source: "in", target: "llm1" },
          { source: "llm1", target: "out" }
        ]
      },
      inputs: { input_var: "Hello from visual workflow block UI!" }
    },
    {
      name: "Condition Workflow (Input -> Condition -> LLM1/LLM2 -> Output)",
      graph: {
        nodes: [
          { id: "in", node_type: { Input: { name: "input_var" } } },
          { id: "cond", node_type: { Condition: { condition_expr: "true" } } },
          { id: "llm1", node_type: { Llm: { prompt_template: "True branch: {{in}}" } } },
          { id: "llm2", node_type: { Llm: { prompt_template: "False branch: {{in}}" } } },
          { id: "out", node_type: "Output" }
        ],
        edges: [
          { source: "in", target: "cond" },
          { source: "cond", target: "llm1", label: "true" },
          { source: "cond", target: "llm2", label: "false" },
          { source: "llm1", target: "out" },
          { source: "llm2", target: "out" }
        ]
      },
      inputs: { input_var: "Conditional test data" }
    }
  ];

  useInput((_input, key) => {
    if (loading) return;
    if (key.upArrow) {
      setSelectedWorkflow(prev => Math.max(0, prev - 1));
    }
    if (key.downArrow) {
      setSelectedWorkflow(prev => Math.min(predefinedWorkflows.length - 1, prev + 1));
    }
    if (key.return) {
      const wf = predefinedWorkflows[selectedWorkflow];
      runWorkflow(wf.graph, wf.inputs);
    }
    if (key.escape || _input === 'q') {
      onBack();
    }
  });

  return (
    <Box flexDirection="column" borderStyle="round" borderColor="magenta" padding={1}>
      <Text bold color="magenta">Block-Based Visual Workflow Editor</Text>
      <Text color="gray">Assemble and execute nodes via the DAG engine.</Text>

      <Box flexDirection="column" marginTop={1}>
        <Text bold>Select Workflow Template to Execute:</Text>
        {predefinedWorkflows.map((wf, index) => {
          const isSelected = index === selectedWorkflow;
          return (
            <Box key={index} marginTop={1} flexDirection="column">
              <Text color={isSelected ? 'cyan' : 'white'}>
                {isSelected ? '▶ ' : '  '}{wf.name}
              </Text>
              {isSelected && (
                <Box marginLeft={2} flexDirection="column">
                   <Text color="gray">Nodes: {wf.graph.nodes.map(n => n.id).join(', ')}</Text>
                   <Text color="gray">Edges: {wf.graph.edges.map(e => `${e.source}->${e.target}`).join(', ')}</Text>
                </Box>
              )}
            </Box>
          );
        })}
      </Box>

      {loading && (
        <Box marginTop={1}>
          <Text color="yellow"><Spinner type="dots" /> Executing Visual Workflow on Backend...</Text>
        </Box>
      )}

      {error && (
        <Box marginTop={1} borderStyle="single" borderColor="red">
          <Text color="red">Error: {error}</Text>
        </Box>
      )}

      {result && !loading && !error && (
        <Box marginTop={1} flexDirection="column" borderStyle="single" borderColor="green" padding={1}>
          <Text bold color="green">Workflow Execution Result:</Text>
          <Text>{result}</Text>
        </Box>
      )}

      <Box marginTop={1}>
        <Text color="gray" italic>Press Enter to run, 'q' to go back.</Text>
      </Box>
    </Box>
  );
};
