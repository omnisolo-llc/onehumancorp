import React, { useState } from 'react';
import { Box, Text, useInput } from 'ink';

export interface WorkflowBuilderProps {
  onBack: () => void;
  onRun: (payload: any) => void;
}

const AVAILABLE_BLOCKS = [
  { id: 'trigger_message', type: 'Trigger', label: 'Inbound Message' },
  { id: 'trigger_schedule', type: 'Trigger', label: 'Schedule (Daily)' },
  { id: 'action_research', type: 'Action', label: 'Web Research' },
  { id: 'action_analyze', type: 'Action', label: 'Analyze Sentiment' },
  { id: 'action_draft', type: 'Action', label: 'Draft Reply' },
  { id: 'condition_approval', type: 'Condition', label: 'Wait for Approval' },
  { id: 'output_send', type: 'Output', label: 'Send Message' },
  { id: 'output_save', type: 'Output', label: 'Save to Memory' },
];

export const WorkflowBuilder: React.FC<WorkflowBuilderProps> = ({ onBack, onRun }) => {
  const [blocks, setBlocks] = useState<any[]>([]);
  const [selectedIndex, setSelectedIndex] = useState(0);

  useInput((input, key) => {
    if (key.upArrow) {
      setSelectedIndex(prev => Math.max(0, prev - 1));
    }
    if (key.downArrow) {
      // plus 2 for Run and Back buttons
      setSelectedIndex(prev => Math.min(AVAILABLE_BLOCKS.length + 1, prev + 1));
    }
    if (key.return) {
      if (selectedIndex < AVAILABLE_BLOCKS.length) {
        // Add block
        const blockDef = AVAILABLE_BLOCKS[selectedIndex];
        setBlocks([...blocks, { ...blockDef, id: `${blockDef.id}_${Date.now()}` }]);
      } else if (selectedIndex === AVAILABLE_BLOCKS.length) {
        // Run workflow
        if (blocks.length > 0) {
          const nodeMap: any = {};
          for (let i = 0; i < blocks.length; i++) {
            const b = blocks[i];
            nodeMap[b.id] = {
              id: b.id,
              type: b.type,
              label: b.label,
              next: i < blocks.length - 1 ? [blocks[i+1].id] : []
            };
          }
          const payload = {
            version: '1.0',
            entrypoint: blocks[0].id,
            nodes: nodeMap
          };
          onRun(payload);
        }
      } else if (selectedIndex === AVAILABLE_BLOCKS.length + 1) {
        // Back
        onBack();
      }
    }
  });

  return (
    <Box flexDirection="column" borderStyle="round" borderColor="cyan" padding={1}>
      <Text bold color="cyan" marginBottom={1}>Visual Workflow Builder (CLI Edition)</Text>

      <Box flexDirection="row" marginBottom={1}>
        <Box flexDirection="column" width="50%">
          <Text bold underline marginBottom={1}>Palette:</Text>
          {AVAILABLE_BLOCKS.map((block, index) => {
            const isSelected = index === selectedIndex;
            return (
              <Box key={index}>
                <Text color={isSelected ? 'blue' : 'gray'}>{isSelected ? '▶ ' : '  '}</Text>
                <Text color={isSelected ? 'white' : 'gray'}>{block.label} ({block.type})</Text>
              </Box>
            );
          })}
          <Box marginTop={1}>
            <Text color={selectedIndex === AVAILABLE_BLOCKS.length ? 'green' : 'gray'}>
              {selectedIndex === AVAILABLE_BLOCKS.length ? '▶ ' : '  '}[ RUN WORKFLOW ]
            </Text>
          </Box>
          <Box>
            <Text color={selectedIndex === AVAILABLE_BLOCKS.length + 1 ? 'red' : 'gray'}>
              {selectedIndex === AVAILABLE_BLOCKS.length + 1 ? '▶ ' : '  '}[ BACK ]
            </Text>
          </Box>
        </Box>

        <Box flexDirection="column" width="50%" borderStyle="single" borderColor="gray" padding={1}>
          <Text bold underline marginBottom={1}>Canvas:</Text>
          {blocks.length === 0 ? (
            <Text dimColor>Canvas is empty.</Text>
          ) : (
            blocks.map((block, index) => (
              <Box key={index} flexDirection="column">
                <Text color="yellow">[{block.type}] {block.label}</Text>
                {index < blocks.length - 1 && <Text color="gray">  ↓</Text>}
              </Box>
            ))
          )}
        </Box>
      </Box>
    </Box>
  );
};
