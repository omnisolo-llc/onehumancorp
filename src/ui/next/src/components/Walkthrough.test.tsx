import { render, screen, fireEvent, act } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import React from 'react';
import { InteractiveWalkthrough } from './Walkthrough';

describe('InteractiveWalkthrough Component', () => {
    beforeEach(() => {
        vi.useFakeTimers();
    });

    afterEach(() => {
        vi.runOnlyPendingTimers();
        vi.useRealTimers();
        document.body.innerHTML = '';
    });

    it('renders nothing when isOpen is false', () => {
        const { container } = render(
            <InteractiveWalkthrough
                steps={[]}
                isOpen={false}
                onClose={vi.fn()}
                onComplete={vi.fn()}
            />
        );
        expect(container.firstChild).toBeNull();
    });

    it('renders nothing when steps array is empty', () => {
        const { container } = render(
            <InteractiveWalkthrough
                steps={[]}
                isOpen={true}
                onClose={vi.fn()}
                onComplete={vi.fn()}
            />
        );
        expect(container.firstChild).toBeNull();
    });

    it('renders step content when target element exists', () => {
        const targetElement = document.createElement('div');
        targetElement.id = 'target-id';
        document.body.appendChild(targetElement);
        targetElement.getBoundingClientRect = vi.fn(() => ({
            width: 100, height: 20, top: 0, left: 0, bottom: 20, right: 100, x: 0, y: 0, toJSON: () => {}
        }));
        targetElement.scrollIntoView = vi.fn();

        const steps = [
            { targetId: 'target-id', title: 'Step 1', content: 'Content 1' }
        ];

        render(
            <InteractiveWalkthrough
                steps={steps}
                isOpen={true}
                onClose={vi.fn()}
                onComplete={vi.fn()}
            />
        );

        act(() => {
            vi.advanceTimersByTime(350);
        });

        expect(screen.getByText('Step 1')).toBeInTheDocument();
        expect(screen.getByText('Content 1')).toBeInTheDocument();
        expect(screen.getByText('Finish')).toBeInTheDocument();
        expect(screen.getByText('Step 1 of 1')).toBeInTheDocument();
    });

    it('navigates to next step', () => {
        const target1 = document.createElement('div');
        target1.id = 'target-1';
        document.body.appendChild(target1);
        target1.getBoundingClientRect = vi.fn(() => ({
            width: 100, height: 20, top: 0, left: 0, bottom: 20, right: 100, x: 0, y: 0, toJSON: () => {}
        }));
        target1.scrollIntoView = vi.fn();

        const target2 = document.createElement('div');
        target2.id = 'target-2';
        document.body.appendChild(target2);
        target2.getBoundingClientRect = vi.fn(() => ({
            width: 100, height: 20, top: 0, left: 0, bottom: 20, right: 100, x: 0, y: 0, toJSON: () => {}
        }));
        target2.scrollIntoView = vi.fn();

        const steps = [
            { targetId: 'target-1', title: 'Step 1', content: 'Content 1' },
            { targetId: 'target-2', title: 'Step 2', content: 'Content 2' }
        ];

        render(
            <InteractiveWalkthrough
                steps={steps}
                isOpen={true}
                onClose={vi.fn()}
                onComplete={vi.fn()}
            />
        );

        act(() => {
            vi.advanceTimersByTime(350);
        });

        expect(screen.getByText('Step 1')).toBeInTheDocument();
        expect(screen.getByText('Next')).toBeInTheDocument();

        fireEvent.click(screen.getByText('Next'));

        act(() => {
            vi.advanceTimersByTime(350);
        });

        expect(screen.getByText('Step 2')).toBeInTheDocument();
        expect(screen.getByText('Finish')).toBeInTheDocument();
    });

    it('calls onClose and onComplete when finishing the last step', () => {
        const target1 = document.createElement('div');
        target1.id = 'target-1';
        document.body.appendChild(target1);
        target1.getBoundingClientRect = vi.fn(() => ({
            width: 100, height: 20, top: 0, left: 0, bottom: 20, right: 100, x: 0, y: 0, toJSON: () => {}
        }));
        target1.scrollIntoView = vi.fn();

        const steps = [
            { targetId: 'target-1', title: 'Step 1', content: 'Content 1' }
        ];

        const onClose = vi.fn();
        const onComplete = vi.fn();

        render(
            <InteractiveWalkthrough
                steps={steps}
                isOpen={true}
                onClose={onClose}
                onComplete={onComplete}
            />
        );

        act(() => {
            vi.advanceTimersByTime(350);
        });

        fireEvent.click(screen.getByText('Finish'));

        expect(onClose).toHaveBeenCalled();
        expect(onComplete).toHaveBeenCalled();
    });

    it('renders with top position', () => {
        const targetElement = document.createElement('div');
        targetElement.id = 'target-top';
        document.body.appendChild(targetElement);
        targetElement.getBoundingClientRect = vi.fn(() => ({
            width: 100, height: 20, top: 0, left: 0, bottom: 20, right: 100, x: 0, y: 0, toJSON: () => {}
        }));
        targetElement.scrollIntoView = vi.fn();

        const steps = [
            { targetId: 'target-top', title: 'Step Top', content: 'Content', position: 'top' as const }
        ];

        render(
            <InteractiveWalkthrough
                steps={steps}
                isOpen={true}
                onClose={vi.fn()}
                onComplete={vi.fn()}
            />
        );

        act(() => {
            vi.advanceTimersByTime(350);
        });

        expect(screen.getByText('Step Top')).toBeInTheDocument();
    });

    it('renders with left position', () => {
        const targetElement = document.createElement('div');
        targetElement.id = 'target-left';
        document.body.appendChild(targetElement);
        targetElement.getBoundingClientRect = vi.fn(() => ({
            width: 100, height: 20, top: 0, left: 0, bottom: 20, right: 100, x: 0, y: 0, toJSON: () => {}
        }));
        targetElement.scrollIntoView = vi.fn();

        const steps = [
            { targetId: 'target-left', title: 'Step Left', content: 'Content', position: 'left' as const }
        ];

        render(
            <InteractiveWalkthrough
                steps={steps}
                isOpen={true}
                onClose={vi.fn()}
                onComplete={vi.fn()}
            />
        );

        act(() => {
            vi.advanceTimersByTime(350);
        });

        expect(screen.getByText('Step Left')).toBeInTheDocument();
    });

    it('renders with right position', () => {
        const targetElement = document.createElement('div');
        targetElement.id = 'target-right';
        document.body.appendChild(targetElement);
        targetElement.getBoundingClientRect = vi.fn(() => ({
            width: 100, height: 20, top: 0, left: 0, bottom: 20, right: 100, x: 0, y: 0, toJSON: () => {}
        }));
        targetElement.scrollIntoView = vi.fn();

        const steps = [
            { targetId: 'target-right', title: 'Step Right', content: 'Content', position: 'right' as const }
        ];

        render(
            <InteractiveWalkthrough
                steps={steps}
                isOpen={true}
                onClose={vi.fn()}
                onComplete={vi.fn()}
            />
        );

        act(() => {
            vi.advanceTimersByTime(350);
        });

        expect(screen.getByText('Step Right')).toBeInTheDocument();
    });

    it('calls onClose when close button is clicked', () => {
         const target1 = document.createElement('div');
         target1.id = 'target-1';
         document.body.appendChild(target1);
         target1.getBoundingClientRect = vi.fn(() => ({
             width: 100, height: 20, top: 0, left: 0, bottom: 20, right: 100, x: 0, y: 0, toJSON: () => {}
         }));
         target1.scrollIntoView = vi.fn();

         const steps = [
             { targetId: 'target-1', title: 'Step 1', content: 'Content 1' }
         ];

         const onClose = vi.fn();

         render(
             <InteractiveWalkthrough
                 steps={steps}
                 isOpen={true}
                 onClose={onClose}
                 onComplete={vi.fn()}
             />
         );

         act(() => {
             vi.advanceTimersByTime(350);
         });

         const closeBtn = screen.getByRole('button', { name: '' }); // the X svg button doesn't have a label but is the only other button
         // Actually, let's find the SVG container button
         const buttons = screen.getAllByRole('button');
         fireEvent.click(buttons[0]); // The close button is first

         expect(onClose).toHaveBeenCalled();
     });
});
