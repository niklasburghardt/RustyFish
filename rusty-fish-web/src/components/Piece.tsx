import {useDraggable} from "@dnd-kit/core";
import React from "react";

function Piece() {
  const {attributes, listeners, setNodeRef, transform} = useDraggable({
    id: "black-bishop",
  });
  const style = transform
    ? {
        transform: `translate3d(${transform.x}px, ${transform.y}px, 0)`,
      }
    : undefined;

  return (
    <button ref={setNodeRef} style={style} {...listeners} {...attributes}>
      <img src="/pieces/w-b.png" alt="Bishop" />
    </button>
  );
  return <div>Piece</div>;
}

export default Piece;
