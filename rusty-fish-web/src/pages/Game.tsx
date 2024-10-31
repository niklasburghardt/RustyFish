import React, {act, useEffect, useState} from "react";
import {greet, ChessEngine} from "rusty-fish";
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
  const [marked, setMarked] = useState<Array<boolean>>([]);
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
      setEngine(engine);
      setBoard(engine.get_board());
      loadAudio();
      for (let i = 0; i < 64; i++) {
        setMarked((prev) => [...prev, false]);
      }
      console.log("BARD");
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
    makeMove(active.id - 1, over.id);
  }

  function makeMove(start: number, end: number) {
    board[end] === "" ? playMoveSound() : playCaptureSound();
    engine?.make_move(start, end);
    setLastStart(start);
    setLastEnd(end);
    setActivePiece(null);
    setAlreadyActivePiece(null);
    getBoard();
    setMoves(engine?.generate_moves());
    setFriendlyColor(friendlyColor === "w" ? "b" : "w");
  }

  function resetMarked() {
    setMarked([]);
  }

  function handleSquareClick(id: number) {
  
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

  const getCircleBackground = (x: number, y: number) => {
    return (x + y) % 2 == 0 ? "bg-[#646c44]" : "bg-[#829769]";
  };

  const handleRightClick = (e: React.MouseEvent, i: number) => {
    e.preventDefault();
    console.log("right click at", i);
    if (marked.at(i) == false) {
      setMarked([...marked.slice(0, i), true, ...marked.slice(i + 1)]);
    } else {
      setMarked([...marked.slice(0, i), false, ...marked.slice(i + 1)]);
    }
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
                      className={`h-[10vh] w-[10vh] duration-50 ${getCircleBackground(
                        x,
                        y
                      )}`}
                      key={i}
                      onClick={() => handleSquareClick(i)}
                      onContextMenu={(e) => handleRightClick(e, i)}
                    >
                      <Square
                        active={activePiece === i}
                        alreadyActive={alreadyActivePiece === i}
                        id={i}
                        x={x}
                        y={y}
                        marked={marked.at(i) == true}
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
