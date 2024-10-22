import React, {useEffect, useState} from "react";
import {add, greet, ChessEngine} from "rusty-fish";
import Square from "../components/Square";
import Piece from "../components/Piece";
import {DndContext} from "@dnd-kit/core";

const Game = () => {
  const [parent, setParent] = useState<any>(null);
  const [engine, setEngine] = useState<ChessEngine | null>(null);

  const [board, setBoard] = useState<Array<string>>([]);

  const getBoard = () => {
    setBoard(engine?.get_board() || []);
  };

  useEffect(() => {
    const initEngine = async () => {
      const engine = await ChessEngine.new();
      engine.init();
      setBoard(engine.get_board());
      setEngine(engine);
      console.log("BOARD");
      console.log(engine.get_board());
    };
    initEngine();
  }, []);

  function handleDragEnd(event: any) {
    const {active, over} = event;

    // If the item is dropped over a container, set it as the parent
    // otherwise reset the parent to `null`
    console.log(event);
    engine?.make_move(active.id - 1, over.id);
    getBoard();
    console.log("BOARD");
    console.log(engine?.get_board());
    setParent(over ? over.id : null);
  }
  return (
    <DndContext onDragEnd={handleDragEnd}>
      <div className="w-full flex items-center justify-center pt-20">
        <div className="w-[80vh] h-[80vh] grid grid-cols-8 gap-0 ">
          {Array.from({length: 64}).map((_, i) => {
            i = 63 - i;
            const x = i % 8;
            const y = Math.floor(i / 8);
            return (
              <div className="h-[10vh] w-[10vh]">
                <Square id={i} key={i} x={x} y={y}>
                  {board[i] === "" ? null : <Piece id={i + 1} />}
                </Square>
              </div>
            );
          })}
        </div>
        <div></div>
      </div>
    </DndContext>
  );
};

export default Game;
