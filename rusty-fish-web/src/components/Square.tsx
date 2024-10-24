import {useDroppable} from "@dnd-kit/core";
import React from "react";
import {ChessPiece} from "../types/pieces";

function Square({
  children,
  x,
  y,
  id,
  active,
  alreadyActive,
  lastStart,
  lastEnd,
  possible,
}: {
  x: number;
  y: number;
  children: React.ReactNode;
  id: number;
  active: boolean;
  alreadyActive: boolean;
  lastStart: boolean;
  lastEnd: boolean;
  possible: boolean;
}) {
  const {isOver, setNodeRef} = useDroppable({
    id: id,
  });
  const getBackgroundColor = () => {
    if (active) {
      return (x + y) % 2 == 0 ? "bg-[#646c44]" : "bg-[#829769]";
    } else if (lastStart || lastEnd) {
      return (x + y) % 2 == 0 ? "bg-[#9e962a]" : "bg-[#ccd46c]";
    } else {
      return (x + y) % 2 == 0 ? "bg-[#b58863]" : "bg-[#f0d9b5]";
    }
    return "bg-[#b58863]";
  };

  return (
    <div
      ref={setNodeRef}
      className={`${getBackgroundColor()} w-[100%] z-20 h-[100%] transition-all duration-100`}
    >
      {children}
    </div>
  );
}

export default Square;
