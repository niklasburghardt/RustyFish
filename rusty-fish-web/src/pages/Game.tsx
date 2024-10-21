import React, {useState} from "react";
import {add, mult, greet} from "rusty-fish";
import Square from "../components/Square";
import Piece from "../components/Piece";
import {DndContext} from "@dnd-kit/core";

const Game = () => {
  const [parent, setParent] = useState(null);
  const draggableMarkup = <Piece />;
  function handleDragEnd(event: any) {
    const {active, over} = event;

    // If the item is dropped over a container, set it as the parent
    // otherwise reset the parent to `null`
    console.log(event);
    setParent(over ? over.id : null);
  }
  return (
    <DndContext onDragEnd={handleDragEnd}>
      <div className="w-full flex items-center justify-center pt-20">
        <div className="w-[80vh] h-[80vh] grid grid-cols-8 gap-0 ">
          {Array.from({length: 64}).map((_, i) => {
            const x = i % 8;
            const y = Math.floor(i / 8);
            return (
              <div className="h-[10vh] w-[10vh]">
                <Square key={i} x={x} y={y}>
                  {parent === `x${x}y${y}` ? draggableMarkup : null}
                </Square>
              </div>
            );
          })}
        </div>
        <div>{parent === null ? draggableMarkup : null}</div>
      </div>
    </DndContext>
  );
};

export default Game;
