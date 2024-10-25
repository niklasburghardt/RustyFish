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

  const getCircleBackground = () => {
    return (x + y) % 2 == 0 ? "bg-[#646c44]" : "bg-[#829769]";
  };

  return (
    <div
      ref={setNodeRef}
      className={`${getBackgroundColor()} items-center justify-center  w-[100%] z-10 h-[100%] transition-all duration-100 ${
        possible && children && "rounded-full"
      }`}
    >
      {children}
      {possible && !children && (
        <div className="flex items-center justify-center w-full h-full">
          <div
            className={` w-7 h-7 rounded-full ${getCircleBackground()} opacity-100`}
          />
        </div>
      )}
    </div>
  );
}

export default Square;
