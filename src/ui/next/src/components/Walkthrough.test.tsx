import { render, screen, fireEvent, act, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import React from 'react';
import { InteractiveWalkthrough, WalkthroughTarget } from './Walkthrough';

describe('InteractiveWalkthrough Component', () => {
    let mockElement: HTMLElement;
    let getElementByIdSpy: any;

    beforeEach(() => {
        mockElement = document.createElement('div');
        mockElement.id = 'target-id';
        mockElement.scrollIntoView = vi.fn();
        mockElement.getBoundingClientRect = vi.fn(() => ({
            width: 100, height: 50, top: 10, bottom: 60, left: 10, right: 110, x: 10, y: 10, toJSON: () => {}
        }));

        getElementByIdSpy = vi.spyOn(document, 'getElementById').mockImplementation((id) => {
            if (id === 'target-id') return mockElement;
            return null;
        });

        vi.useFakeTimers();
    });

    afterEach(() => {
        vi.restoreAllMocks();
    });

    it('renders nothing when isOpen is false', () => {
        const { container } = render(
            <InteractiveWalkthrough
                steps={[{ targetId: 'target-id', title: 'Test', content: 'Test Content' }]}
                isOpen={false}
                onClose={vi.fn()}
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
            />
        );
        expect(container.firstChild).toBeNull();
    });

    it('renders nothing when process.env.OHC_E2E is true', () => {
        const origE2E = process.env.OHC_E2E;
        process.env.OHC_E2E = 'true';
        const { container } = render(
            <InteractiveWalkthrough
                steps={[{ targetId: 'target-id', title: 'Test', content: 'Test Content' }]}
                isOpen={true}
                onClose={vi.fn()}
            />
        );
        expect(container.firstChild).toBeNull();
        process.env.OHC_E2E = origE2E;
    });

    it('renders target and bubble after timeout when target is found', async () => {
        const { container } = render(
            <InteractiveWalkthrough
                steps={[{ targetId: 'target-id', title: 'Step 1', content: 'Test Content' }]}
                isOpen={true}
                onClose={vi.fn()}
            />
        );

        act(() => {
            vi.advanceTimersByTime(300);
        });

        expect(mockElement.scrollIntoView).toHaveBeenCalled();
        expect(screen.getByText('Step 1')).toBeInTheDocument();
        expect(screen.getByText('Test Content')).toBeInTheDocument();

        // Window scroll and resize events should recalculate rect
        act(() => {
            window.dispatchEvent(new Event('scroll'));
            window.dispatchEvent(new Event('resize'));
        });
        expect(mockElement.getBoundingClientRect).toHaveBeenCalledTimes(3);
    });

    it('handles warning when target is not found', () => {
        const consoleWarnSpy = vi.spyOn(console, 'warn').mockImplementation(() => {});
        const { container } = render(
            <InteractiveWalkthrough
                steps={[{ targetId: 'unknown-id', title: 'Step 1', content: 'Test Content' }]}
                isOpen={true}
                onClose={vi.fn()}
            />
        );

        expect(consoleWarnSpy).toHaveBeenCalledWith('Walkthrough: Target element with id "unknown-id" not found.');
        expect(container.firstChild).toBeNull();
    });

    it('navigates to next step and completes', async () => {
        const onCompleteMock = vi.fn();
        const onCloseMock = vi.fn();

        // We will need a second target for step 2
        const mockElement2 = document.createElement('div');
        mockElement2.id = 'target-id-2';
        mockElement2.scrollIntoView = vi.fn();
        mockElement2.getBoundingClientRect = vi.fn(() => ({
            width: 100, height: 50, top: 10, bottom: 60, left: 10, right: 110, x: 10, y: 10, toJSON: () => {}
        }));

        getElementByIdSpy.mockImplementation((id: string) => {
            if (id === 'target-id') return mockElement;
            if (id === 'target-id-2') return mockElement2;
            return null;
        });

        const { container } = render(
            <InteractiveWalkthrough
                steps={[
                    { targetId: 'target-id', title: 'Step 1', content: 'First' },
                    { targetId: 'target-id-2', title: 'Step 2', content: 'Second' }
                ]}
                isOpen={true}
                onClose={onCloseMock}
                onComplete={onCompleteMock}
            />
        );

        act(() => {
            vi.advanceTimersByTime(300);
        });

        expect(screen.getByText('Step 1 of 2')).toBeInTheDocument();
        const nextButton = screen.getByRole('button', { name: 'Next' });

        act(() => {
            fireEvent.click(nextButton);
        });

        act(() => {
            vi.advanceTimersByTime(300);
        });

        expect(screen.getByText('Step 2 of 2')).toBeInTheDocument();
        const finishButton = screen.getByRole('button', { name: 'Finish' });

        act(() => {
            fireEvent.click(finishButton);
        });

        expect(onCompleteMock).toHaveBeenCalled();
        expect(onCloseMock).toHaveBeenCalled();
    });

    it('handles skip button click', () => {
        const onCloseMock = vi.fn();

        const { container } = render(
            <InteractiveWalkthrough
                steps={[{ targetId: 'target-id', title: 'Step 1', content: 'First' }]}
                isOpen={true}
                onClose={onCloseMock}
            />
        );

        act(() => {
            vi.advanceTimersByTime(300);
        });

        // Skip button is the X icon, which is the only other button
        const skipButton = container.querySelector('button.text-gray-500');
        act(() => {
            fireEvent.click(skipButton!);
        });

        expect(onCloseMock).toHaveBeenCalled();
    });

    describe('positions', () => {
        it('renders bottom position (default)', () => {
            render(
                <InteractiveWalkthrough
                    steps={[{ targetId: 'target-id', title: 'Test', content: 'Test Content' }]}
                    isOpen={true}
                    onClose={vi.fn()}
                />
            );
            act(() => { vi.advanceTimersByTime(300); });
            expect(screen.getByText('Test')).toBeInTheDocument();
        });

        it('renders bottom position explicitly', () => {
            render(
                <InteractiveWalkthrough
                    steps={[{ targetId: 'target-id', title: 'Test', content: 'Test Content', position: 'bottom' }]}
                    isOpen={true}
                    onClose={vi.fn()}
                />
            );
            act(() => { vi.advanceTimersByTime(300); });
            expect(screen.getByText('Test')).toBeInTheDocument();
        });

        it('renders top position', () => {
            render(
                <InteractiveWalkthrough
                    steps={[{ targetId: 'target-id', title: 'Test', content: 'Test Content', position: 'top' }]}
                    isOpen={true}
                    onClose={vi.fn()}
                />
            );
            act(() => { vi.advanceTimersByTime(300); });
            expect(screen.getByText('Test')).toBeInTheDocument();
        });

        it('renders left position', () => {
            render(
                <InteractiveWalkthrough
                    steps={[{ targetId: 'target-id', title: 'Test', content: 'Test Content', position: 'left' }]}
                    isOpen={true}
                    onClose={vi.fn()}
                />
            );
            act(() => { vi.advanceTimersByTime(300); });
            expect(screen.getByText('Test')).toBeInTheDocument();
        });

        it('renders right position', () => {
            render(
                <InteractiveWalkthrough
                    steps={[{ targetId: 'target-id', title: 'Test', content: 'Test Content', position: 'right' }]}
                    isOpen={true}
                    onClose={vi.fn()}
                />
            );
            act(() => { vi.advanceTimersByTime(300); });
            expect(screen.getByText('Test')).toBeInTheDocument();
        });
    });

    describe('WalkthroughTarget', () => {
        it('renders children with id and className', () => {
            const { container } = render(
                <WalkthroughTarget id="my-target" className="custom-class">
                    <span>Child content</span>
                </WalkthroughTarget>
            );
            const targetDiv = container.firstChild as HTMLElement;
            expect(targetDiv.id).toBe('my-target');
            expect(targetDiv.className).toContain('custom-class');
            expect(screen.getByText('Child content')).toBeInTheDocument();
        });
    });
});
