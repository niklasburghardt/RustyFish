import {useDroppable} from "@dnd-kit/core";
import React from "react";
import {ChessPiece} from "../types/pieces";

function Square({
  children,
  x,
  y,
}: {
  x: number;
  y: number;
  children: React.ReactNode;
}) {
  const {isOver, setNodeRef} = useDroppable({
    id: `x${x}y${y}`,
  });

  return (
    <div
      ref={setNodeRef}
      className={`${
        (x + y) % 2 == 0 ? "bg-[#b58863]" : "bg-[#ecccac]"
      } w-[100%] h-[100%]`}
    >
      {children}
    </div>
  );
}

export default Square;
