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
  const [alreadyActive, setAlreadyActive] = useState<number | null>(null);

  const [board, setBoard] = useState<Array<string>>([]);
  const [fenInput, setFenInputs] = useState<string>("");

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
    
    
    setLastEnd(over ? over.id : null);
    // If the item is dropped over a container, set it as the parent
    // otherwise reset the parent to `null`
    console.log(event);
    
    if(active.id -1 === over.id) return;
    engine?.make_move(active.id - 1, over.id);
    getBoard();
    console.log("BOARD");
    console.log(engine?.get_board());
    setParent(over ? over.id : null);
    setActivePiece(null);

    board[over.id] === "" ? playMoveSound() : playCaptureSound();
  }

  function handleSquareClick(id: number) {
    console.log(activePiece);
    if(activePiece) 
    {
      engine?.make_move(activePiece, id);
      getBoard();
      
      setLastStart(activePiece);
      setLastEnd(id);
      setActivePiece(null); }
  }

  async function fromFen() { 
    console.log("creating from fen");
    await engine?.set_board_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    console.log("success");
    getBoard();
    
  }

  return (
    <DndContext modifiers={[snapCenterToCursor]} onDragEnd={handleDragEnd} onDragStart={handleDragStart}>
      <button onClick={fromFen}>From FEN</button>
      <div className="w-full flex items-center justify-center pt-20 flex-row-reverse">
       
        <div className=" w-[80vh] h-[80vh] flex flex-col-reverse">
  {Array.from({ length: 8 }).map((_, rowIndex) => (
    <div className="flex flex-row" key={rowIndex}>
      {Array.from({ length: 8 }).map((_, colIndex) => {
        const i = rowIndex * 8 + colIndex; // Calculate the index for the square
        const x = colIndex; // Column index
        const y = rowIndex; // Row index
        return (
          <div
            className="h-[10vh] w-[10vh]"
            key={i}
          >
            <Square
              id={i}
              x={x}
              y={y}
              lastStart={activePiece === i || lastStart === i}
              lastEnd={lastEnd === i}
              possible={activePiece !== null && i % 5 === 0}
            >
              {board[i] === "" ? null: (
                <Piece id={i + 1} type={board[i]} />
              ) }
            </Square>
            <div className="relative z-0 bottom-[10vh] opacity-20">
              {activePiece === i ? (
                <img src={`/pieces/${board[i]}.png`} />
              ) : null}
            </div>
          </div>
        );
      })}
    </div>
  ))}
</div>

      </div>
    </DndContext>
  );
};

export default Game;
