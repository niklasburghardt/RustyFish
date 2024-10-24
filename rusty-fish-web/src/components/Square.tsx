import {useDroppable} from "@dnd-kit/core";
import React from "react";
import {ChessPiece} from "../types/pieces";

function Square({
  children,
  x,
  y,
  id,
  lastStart,
  lastEnd,
  possible
}: {
  x: number;
  y: number;
  children: React.ReactNode;
  id: number;
  lastStart: boolean;
  lastEnd: boolean;
  possible: boolean;
}) {
  const {isOver, setNodeRef} = useDroppable({
    id: id,
  });

  return (
    <div
      ref={setNodeRef}
      className={`${
        lastStart
          ? "bg-[#cdd26a]"
          : lastEnd
          ? "bg-[#aaa23a]"
          : (x + y) % 2 == 0
          ? "bg-[#b58863]"
          : "bg-[#f0d9b5]"
      } w-[100%] h-[100%]`}
    >
      {children}
      {possible && (
        <div className="z-50 w-full h-full flex items-center justify-center">
        </div>
      )}
    </div>
  );
}

export default Square;
