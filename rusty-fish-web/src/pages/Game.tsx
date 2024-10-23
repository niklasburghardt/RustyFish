import React, {useEffect, useState} from "react";
import {add, greet, ChessEngine} from "rusty-fish";
import Square from "../components/Square";
import Piece from "../components/Piece";
import {DndContext, DragOverlay} from "@dnd-kit/core";
import { snapCenterToCursor } from "@dnd-kit/modifiers";

const Game = () => {
  const [parent, setParent] = useState<any>(null);
  const [engine, setEngine] = useState<ChessEngine | null>(null);
  const [lastStart, setLastStart] = useState<number | null>(null);
  const [lastEnd, setLastEnd] = useState<number | null>(null);
  const [activePiece, setActivePiece] = useState<number | null>(null);

  const [board, setBoard] = useState<Array<string>>([]);

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

  const getBoard = () => {
    setBoard(engine?.get_board() || []);
  };

  const playMoveSound = () => {
    const audio = new Audio("/sounds/move-self.mp3");

    audio.play();
  };

  const playCaptureSound = () => {
    const audio = new Audio("/sounds/capture.mp3");

    audio.play();
  };
  function handleDragStart(event: any) {
    const {active} = event;
    setActivePiece(active.id-1);
  }
  
  function handleDragEnd(event: any) {
    const {active, over} = event;
    if(!active || !over) return;
    if(active.id -1 === over.id) return;
    setActivePiece(null);
    setLastStart(active.id - 1);
    setLastEnd(over ? over.id : null);
    // If the item is dropped over a container, set it as the parent
    // otherwise reset the parent to `null`
    console.log(event);
    engine?.make_move(active.id - 1, over.id);
    getBoard();
    console.log("BOARD");
    console.log(engine?.get_board());
    setParent(over ? over.id : null);

    board[over.id] === "" ? playMoveSound() : playCaptureSound();
  }
  return (
    <DndContext modifiers={[snapCenterToCursor]} onDragEnd={handleDragEnd} onDragStart={handleDragStart}>
      <div className="w-full flex items-center justify-center pt-20">
        <div className="w-[80vh] h-[80vh] grid grid-cols-8 gap-0 ">
          {Array.from({length: 64}).map((_, i) => {
            i = 63 - i;
            const x = i % 8;
            const y = Math.floor(i / 8);
            return (
              <div className="h-[10vh] w-[10vh]">
                <Square
                  id={i}
                  key={i}
                  x={x}
                  y={y}
                  lastStart={activePiece === i || lastStart === i}
                  lastEnd={lastEnd === i}
                  possible={activePiece !== null && i%5 == 0}
                >
                  {board[i] === "" ? null : (
                    <Piece id={i + 1} type={board[i]} />
                  )}
                </Square>
                <div className="relative z-0 bottom-[10vh] opacity-20 ">
                  {activePiece === i ? (<img src={"/pieces/"+board[i]+".png"}/>) : null}
                </div>
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
