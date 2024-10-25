import {useDraggable} from "@dnd-kit/core";
import React from "react";

function Piece({
  id,
  type,
  disabled = false,
}: {
  id: number;
  type: string;
  disabled?: boolean;
}) {
  const {attributes, listeners, setNodeRef, transform} = useDraggable({
    id: id,
    disabled: disabled,
  });
  const style = transform
    ? {
        transform: `translate3d(${transform.x}px, ${transform.y}px, 0)`,
      }
    : undefined;

  return (
    <button
      draggable={false}
      className="z-30"
      ref={setNodeRef}
      style={style}
      {...listeners}
      {...attributes}
    >
      <img draggable={false} src={"/pieces/" + type + ".png"} alt="Bishop" />
    </button>
  );
  return <div>Piece</div>;
}

export default Piece;
