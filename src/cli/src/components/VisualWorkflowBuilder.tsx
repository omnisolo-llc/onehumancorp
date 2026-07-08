import React, { useState } from 'react';
import { Box, Text, useInput } from 'ink';
import { useVisualWorkflow } from '../hooks/useVisualWorkflow.js';
import { MarkdownText } from './MarkdownText.js';
import { ErrorState } from './ErrorState.js';

export interface VisualWorkflowBuilderProps {
  onBack: () => void;
}

export const VisualWorkflowBuilder: React.FC<VisualWorkflowBuilderProps> = ({ onBack }) => {
  const [nodes, setNodes] = useState<{ id: string; type: string }[]>([]);
  const [selectedNodeIndex, setSelectedNodeIndex] = useState(0);
  const [mode, setMode] = useState<'view' | 'add'>('view');

  const { status, result, error, runWorkflow } = useVisualWorkflow();

  useInput((input, key) => {
    if (status === 'running') return; // Disable input while running

    if (mode === 'view') {
      if (key.escape) {
        onBack();
        return;
      }

      if (input === 'a') {
        setMode('add');
      } else if (input === 'r') {
        // Build graph and run
        const graphNodes = nodes.map(n => {
            if (n.type === 'Input') return { id: n.id, type: { Input: { name: 'in' } } };
            if (n.type === 'Llm') return { id: n.id, type: { Llm: { prompt_template: 'Process: {{in}}' } } };
            return { id: n.id, type: { Output: null } };
        });

        const edges = [];
        for (let i = 0; i < nodes.length - 1; i++) {
            edges.push({ source: nodes[i].id, target: nodes[i+1].id });
        }

        runWorkflow({ nodes: graphNodes as any, edges }, { in: 'test data' });
      } else if (key.upArrow || input === 'k') {
        if (nodes.length > 0) {
          setSelectedNodeIndex(Math.max(0, selectedNodeIndex - 1));
        }
      } else if (key.downArrow || input === 'j') {
        if (nodes.length > 0) {
          setSelectedNodeIndex(Math.min(nodes.length - 1, selectedNodeIndex + 1));
        }
      } else if (input === 'd') {
        if (nodes.length > 0) {
          const newNodes = [...nodes];
          newNodes.splice(selectedNodeIndex, 1);
          setNodes(newNodes);
          setSelectedNodeIndex(Math.max(0, Math.min(selectedNodeIndex, newNodes.length - 1)));
        }
      }
    } else if (mode === 'add') {
      if (key.escape) {
        setMode('view');
        return;
      }

      if (input === '1') {
        setNodes([...nodes, { id: `node_${nodes.length + 1}`, type: 'Input' }]);
        setMode('view');
        setSelectedNodeIndex(nodes.length);
      } else if (input === '2') {
        setNodes([...nodes, { id: `node_${nodes.length + 1}`, type: 'Llm' }]);
        setMode('view');
        setSelectedNodeIndex(nodes.length);
      } else if (input === '3') {
        setNodes([...nodes, { id: `node_${nodes.length + 1}`, type: 'Output' }]);
        setMode('view');
        setSelectedNodeIndex(nodes.length);
      }
    }
  });

  return (
    <Box flexDirection="column" borderStyle="round" borderColor="magenta" padding={1}>
      <Text color="magenta" bold>Visual Workflow Builder</Text>

      {status === 'running' && (
         <Box marginTop={1}>
            <Text color="yellow">Running workflow on backend...</Text>
         </Box>
      )}

      {status === 'error' && error && (
         <Box marginTop={1}>
            <ErrorState error={error} />
         </Box>
      )}

      {status === 'complete' && result && (
         <Box marginTop={1} borderStyle="single" borderColor="cyan" padding={1} flexDirection="column">
            <Text color="cyan">Workflow Result:</Text>
            <MarkdownText content={result} />
         </Box>
      )}

      {mode === 'view' && status !== 'running' && (
        <Box flexDirection="column" marginTop={1}>
          <Text color="gray">Commands: [a] Add Node | [d] Delete | [up/down] Select | [r] Run Workflow | [esc] Back</Text>
          <Box flexDirection="column" marginTop={1}>
            {nodes.length === 0 ? (
              <Text color="yellow">No nodes in the workflow. Press 'a' to add one.</Text>
            ) : (
              nodes.map((node, index) => (
                <Box key={index}>
                  <Text color={index === selectedNodeIndex ? "green" : "white"}>
                    {index === selectedNodeIndex ? "> " : "  "}
                    [{node.type}] {node.id}
                  </Text>
                </Box>
              ))
            )}
          </Box>
        </Box>
      )}

      {mode === 'add' && status !== 'running' && (
        <Box flexDirection="column" marginTop={1}>
          <Text color="cyan">Select Node Type to Add:</Text>
          <Text>1. Input</Text>
          <Text>2. Llm</Text>
          <Text>3. Output</Text>
          <Text color="gray">[esc] Cancel</Text>
        </Box>
      )}
    </Box>
  );
};
