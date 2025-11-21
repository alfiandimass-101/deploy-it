const boardElement = document.getElementById('board');
const statusElement = document.getElementById('status');
const movesListElement = document.getElementById('moves-list');
const whiteTimerElement = document.getElementById('white-timer');
const blackTimerElement = document.getElementById('black-timer');

let board = [];
let selectedSquare = null;
let turn = 'white';
let movesHistory = []; // SAN strings
let validMoves = []; // "e2e4" strings
let currentFen = "";

// Unicode Pieces
const PIECES = {
    'w': { 'k': '♔', 'q': '♕', 'r': '♖', 'b': '♗', 'n': '♘', 'p': '♙' },
    'b': { 'k': '♚', 'q': '♛', 'r': '♜', 'b': '♝', 'n': '♞', 'p': '♟' }
};

async function initGame() {
    // Fetch initial state
    await updateState();
    renderBoard();
}

async function updateState(userMove = null) {
    try {
        const payload = {
            history: movesHistory,
            user_move: userMove
        };

        const response = await fetch('/api/move', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(payload)
        });

        if (!response.ok) {
            const err = await response.json();
            console.error("Error:", err.error);
            statusElement.textContent = "Error: " + err.error;
            return false;
        }

        const data = await response.json();
        currentFen = data.fen;
        movesHistory = data.history;
        validMoves = data.valid_moves;

        // Parse FEN to update board
        parseFen(currentFen);

        // Update status
        turn = data.turn.toLowerCase().includes('white') ? 'white' : 'black';
        statusElement.textContent = `${turn.charAt(0).toUpperCase() + turn.slice(1)}'s Turn`;
        if (data.status !== 'Ongoing') {
            statusElement.textContent += ` - ${data.status}`;
        }

        // Update moves list
        movesListElement.innerHTML = '';
        movesHistory.forEach((m, i) => {
            const li = document.createElement('li');
            li.className = 'move-item';
            li.textContent = `${i % 2 === 0 ? (i / 2 + 1) + '.' : ''} ${m}`;
            movesListElement.appendChild(li);
        });

        renderBoard();
        return true;

    } catch (e) {
        console.error("Network error:", e);
        return false;
    }
}

function parseFen(fen) {
    // Simple FEN parser for board
    // rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
    const rows = fen.split(' ')[0].split('/');
    board = [];
    for (let r = 0; r < 8; r++) {
        let row = [];
        let fenRow = rows[r];
        for (let i = 0; i < fenRow.length; i++) {
            const char = fenRow[i];
            if (!isNaN(char)) {
                for (let k = 0; k < parseInt(char); k++) {
                    row.push(null);
                }
            } else {
                const color = char === char.toUpperCase() ? 'w' : 'b';
                const type = char.toLowerCase();
                row.push({ color, type });
            }
        }
        board.push(row);
    }
    // FEN ranks are 8 to 1, so row 0 is rank 8.
    // Our board array: row 0 is rank 8.
    // UI renders row 0 at top. Correct.
}

function renderBoard() {
    boardElement.innerHTML = '';
    for (let row = 0; row < 8; row++) {
        for (let col = 0; col < 8; col++) {
            const square = document.createElement('div');
            square.className = `square ${(row + col) % 2 === 0 ? 'light' : 'dark'}`;
            square.dataset.row = row;
            square.dataset.col = col;

            const piece = board[row][col];
            if (piece) {
                const pieceSpan = document.createElement('span');
                pieceSpan.className = 'piece-text';
                pieceSpan.textContent = PIECES[piece.color][piece.type];
                if (piece.color === 'w') pieceSpan.style.color = '#fff';
                else pieceSpan.style.color = '#000';
                square.appendChild(pieceSpan);
            }

            if (selectedSquare && selectedSquare.row === row && selectedSquare.col === col) {
                square.classList.add('selected');
            }

            // Highlight valid moves from selected square
            if (selectedSquare) {
                const fromStr = coordsToStr(selectedSquare.row, selectedSquare.col);
                const toStr = coordsToStr(row, col);
                const moveStr = fromStr + toStr;
                if (validMoves.includes(moveStr)) {
                    square.classList.add('highlight');
                }
            }

            square.addEventListener('click', () => handleSquareClick(row, col));
            boardElement.appendChild(square);
        }
    }
}

function coordsToStr(row, col) {
    const files = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    const ranks = ['8', '7', '6', '5', '4', '3', '2', '1'];
    return files[col] + ranks[row];
}

async function handleSquareClick(row, col) {
    if (selectedSquare) {
        if (selectedSquare.row === row && selectedSquare.col === col) {
            selectedSquare = null;
            renderBoard();
            return;
        }

        const fromStr = coordsToStr(selectedSquare.row, selectedSquare.col);
        const toStr = coordsToStr(row, col);
        const moveStr = fromStr + toStr;

        if (validMoves.includes(moveStr)) {
            // Execute move
            const success = await updateState({
                from: fromStr,
                to: toStr,
                promotion: null // TODO: Handle promotion
            });

            selectedSquare = null;

            if (success && turn === 'black') {
                // Trigger AI
                makeAIMove();
            }
        } else {
            // Select other piece
            const piece = board[row][col];
            if (piece && piece.color === turn.charAt(0)) {
                selectedSquare = { row, col };
            } else {
                selectedSquare = null;
            }
            renderBoard();
        }
    } else {
        const piece = board[row][col];
        if (piece && piece.color === turn.charAt(0)) {
            selectedSquare = { row, col };
            renderBoard();
        }
    }
}

async function makeAIMove() {
    statusElement.textContent = "AI is thinking...";
    try {
        const response = await fetch('http://127.0.0.1:8081/analyze', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ fen: currentFen, depth: 3 })
        });

        const data = await response.json();
        if (data.best_move) {
            // best_move is "e7e5" or similar (from engine.rs)
            // We need to parse it to from/to
            // Assuming engine returns "e7e5"
            const from = data.best_move.substring(0, 2);
            const to = data.best_move.substring(2, 4);

            await updateState({
                from: from,
                to: to,
                promotion: null
            });
        } else {
            console.error("AI returned no move");
        }
    } catch (e) {
        console.error("AI Error:", e);
    }
}

initGame();
