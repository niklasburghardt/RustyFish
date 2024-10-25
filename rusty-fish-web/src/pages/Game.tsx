import React, {act, useEffect, useState} from "react";
import {add, greet, ChessEngine} from "rusty-fish";
import Square from "../components/Square";
import Piece from "../components/Piece";
import {DndContext, DragOverlay} from "@dnd-kit/core";
import {snapCenterToCursor} from "@dnd-kit/modifiers";
import {Button} from "../components/shadcn/button";
import {Input} from "../components/shadcn/input";

const Game = () => {
  const [parent, setParent] = useState<any>(null);
  const [engine, setEngine] = useState<ChessEngine | null>(null);
  const [lastStart, setLastStart] = useState<number | null>(null);
  const [lastEnd, setLastEnd] = useState<number | null>(null);
  const [activePiece, setActivePiece] = useState<number | null>(null);
  const [alreadyActivePiece, setAlreadyActivePiece] = useState<number | null>(
    null
  );
  const [lmCount, setLmCount] = useState<number>(0);
  const [moves, setMoves] = useState<any>([]);
  const [friendlyColor, setFriendlyColor] = useState<string>("w");

  const [board, setBoard] = useState<Array<string>>([]);
  const [fenInput, setFenInputs] = useState<string>("");
  const [sandbox, setSandbox] = useState<boolean>(false);

  useEffect(() => {
    const initEngine = async () => {
      const engine = await ChessEngine.new();
      engine.init();
      setBoard(engine.get_board());
      setEngine(engine);
      loadAudio();
      console.log("BOARD");
      console.log(engine.get_board());
    };
    initEngine();
  }, []);

  const loadAudio = () => {
    const audio = new Audio("/sounds/move-self.mp3");
    const audio2 = new Audio("/sounds/capture.mp3");
  };

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
    console.log(moves.at(active.id - 1));
    setActivePiece(active.id - 1);
  }

  function handleDragEnd(event: any) {
    const {active, over} = event;
    if (!active || !over) return;
    if (alreadyActivePiece == over.id) {
      setActivePiece(null);
      setAlreadyActivePiece(null);
      return;
    }

    if (active.id - 1 == activePiece) {
      setAlreadyActivePiece(active.id - 1);
    }

    if (!sandbox && !moves.at(active.id - 1)?.includes(over.id)) {
      return;
    }

    // If the item is dropped over a container, set it as the parent
    // otherwise reset the parent to `null`
    console.log(event);

    console.log("BOARD");

    if (active.id - 1 === over.id) return;
    board[over.id] === "" ? playMoveSound() : playCaptureSound();
    engine?.make_move(active.id - 1, over.id);
    setLastStart(active.id - 1);
    setLastEnd(over.id);
    setActivePiece(null);
    setAlreadyActivePiece(null);
    getBoard();
    setMoves(engine?.generate_moves());
    console.log(engine?.get_board());
  }

  function handleSquareClick(id: number) {
    console.log(activePiece);
    if (activePiece) {
      setActivePiece(null);
    }
  }

  async function fromFen() {
    console.log("creating from fen");
    await engine?.set_board_from_fen(
      fenInput || "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"
    );
    console.log("success");
    setMoves(engine?.generate_moves());
    getBoard();
  }

  function generateLegalMoves() {
    let moves = engine?.generate_moves();
    setMoves(moves);
    console.log(moves);
  }

  const getCircleBackground = (i: number) => {
    return i % 2 == 0 ? "bg-[#646c44]" : "bg-[#829769]";
  };

  return (
    <DndContext
      modifiers={[snapCenterToCursor]}
      onDragEnd={handleDragEnd}
      onDragStart={handleDragStart}
    >
      <div className="p-4">
        <div className="flex space-x-2">
          <Input
            placeholder="rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
            className="w-[450px]"
            value={fenInput}
            onChange={(e) => setFenInputs(e.target.value)}
          />
          <Button onClick={fromFen}>From FEN</Button>
          <Button onClick={generateLegalMoves}>Generate Legal Moves</Button>
          <Button
            variant={sandbox ? "destructive" : "secondary"}
            onClick={() => setSandbox(!sandbox)}
          >
            {sandbox ? "Sandbox On" : "Sandbox off"}
          </Button>
        </div>
        <div className="w-full flex items-center justify-center h-[90vh] flex-row-reverse">
          <div className=" w-[80vh] h-[80vh] flex flex-col-reverse cursor-pointer">
            {Array.from({length: 8}).map((_, rowIndex) => (
              <div className="flex flex-row" key={rowIndex}>
                {Array.from({length: 8}).map((_, colIndex) => {
                  const i = rowIndex * 8 + colIndex; // Calculate the index for the square
                  const x = colIndex; // Column index
                  const y = rowIndex; // Row index
                  return (
                    <div
                      className={`h-[10vh] w-[10vh] ${getCircleBackground(i)}`}
                      key={i}
                      onClick={() => handleSquareClick(i)}
                    >
                      <Square
                        active={activePiece === i}
                        alreadyActive={alreadyActivePiece === i}
                        id={i}
                        x={x}
                        y={y}
                        lastStart={activePiece === i || lastStart === i}
                        lastEnd={lastEnd === i}
                        possible={
                          activePiece && moves.at(activePiece)?.includes(i)
                        }
                      >
                        {board[i] === "" ? null : (
                          <Piece
                            id={i + 1}
                            type={board[i]}
                            disabled={
                              !sandbox && !board[i]?.endsWith(friendlyColor)
                            }
                          />
                        )}
                      </Square>
                      {/* <div className="relative bottom-5">{i}</div> */}
                    </div>
                  );
                })}
              </div>
            ))}
          </div>
        </div>
      </div>
    </DndContext>
  );
};

export default Game;
