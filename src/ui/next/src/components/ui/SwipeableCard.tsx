import React, { useState, ReactNode } from "react";
import { useDrag } from "@use-gesture/react";
import { motion, useAnimation, useMotionValue, useTransform } from "framer-motion";

interface SwipeableCardProps {
  id: string;
  onSwipeRight: () => void;
  onSwipeLeft: () => void;
  children: ReactNode;
  isProcessing?: boolean;
}

export function SwipeableCard({
  id,
  onSwipeRight,
  onSwipeLeft,
  children,
  isProcessing = false,
}: SwipeableCardProps) {
  const [exitX, setExitX] = useState<number | string>(0);
  const [isRemoving, setIsRemoving] = useState(false);

  const x = useMotionValue(0);
  const controls = useAnimation();

  // Opacity decreases as card is swiped further
  const opacity = useTransform(x, [-150, 0, 150], [0.3, 1, 0.3]);
  const scale = useTransform(x, [-150, 0, 150], [0.95, 1, 0.95]);

  const bind = useDrag(({ movement: [mx], down, velocity: [vx], direction: [dx] }) => {
    if (isProcessing || isRemoving) return;

    if (!down) {
      // Determine if swipe is significant enough
      const trigger = vx > 0.5 || Math.abs(mx) > 100;

      if (trigger) {
        if (dx > 0) {
          // Swipe right (Approve)
          setExitX(300);
          setIsRemoving(true);
          onSwipeRight();
        } else {
          // Swipe left (Dismiss)
          setExitX(-300);
          setIsRemoving(true);
          onSwipeLeft();
        }
      } else {
        // Return to center
        controls.start({ x: 0, transition: { type: "spring", stiffness: 300, damping: 20 } });
      }
    } else {
      // Follow drag
      x.set(mx);
    }
  }, { axis: "x" });

  const dismissOpacity = useTransform(x, [0, -100], [0, 1]);
  const approveOpacity = useTransform(x, [0, 100], [0, 1]);

  return (
    <motion.div
      {...bind()}
      style={{ x, opacity, scale }}
      animate={controls}
      exit={{ x: exitX, opacity: 0, height: 0, transition: { duration: 0.2 } }}
      className="relative w-full touch-pan-y"
    >
      {/* Background visual cues when swiping */}
      <div className="absolute inset-0 flex items-center justify-between px-6 rounded-[24px] pointer-events-none -z-10 bg-gradient-to-r from-red-500/20 via-transparent to-green-500/20">
         <motion.span className="text-red-500 font-bold" style={{ opacity: dismissOpacity }}>Dismiss</motion.span>
         <motion.span className="text-green-500 font-bold" style={{ opacity: approveOpacity }}>Approve</motion.span>
      </div>
      {children}
    </motion.div>
  );
}
