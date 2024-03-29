import init, {
    World,
    Direction,
    GameStatus,
} from 'wasm_game'

import { random } from '../../utils/random.js'

init().then((wasm: any) => {
    const CELL_SIZE = 20
    const WORLD_WIDTH = 21
    const snakeIndex = random(WORLD_WIDTH * WORLD_WIDTH)
    const world = World.new(WORLD_WIDTH, snakeIndex)
    const worldWidth = world.width()
    const fps = 5

    const gameStatus = document.getElementById('game-status')
    const gameScore = document.getElementById('game-score')
    const gameControlBtn = document.getElementById("game-start-btn")

    const canvas = <HTMLCanvasElement>document.getElementById('snake-world')
    const context = canvas.getContext('2d')

    canvas.width = worldWidth * CELL_SIZE
    canvas.height = worldWidth * CELL_SIZE

    gameControlBtn.addEventListener('click', () => {
        const status = world.game_status();
        if (status == undefined) {
            gameControlBtn.textContent = "Pause"
            world.start_game();
            run();
        } else {
            location.reload();
        }
    })

    document.addEventListener('keydown', e => {
        switch (e.key) {
            case 'w':
                world.change_snake_direction(Direction.Up);
                break
            case 's':
                world.change_snake_direction(Direction.Down);
                break
            case 'a':
                world.change_snake_direction(Direction.Left);
                break
            case 'd':
                world.change_snake_direction(Direction.Right);
                break
        }
    })

    function drawWorld() {
        context.beginPath()
        for (let x = 0; x <= worldWidth; x++) {
            context.moveTo(CELL_SIZE * x, 0)
            context.lineTo(CELL_SIZE * x, CELL_SIZE * worldWidth)
        }
        for (let y = 0; y <= worldWidth; y++) {
            context.moveTo(0, CELL_SIZE * y)
            context.lineTo(CELL_SIZE * worldWidth, CELL_SIZE * y)
        }
        context.stroke()
    }

    function drawSnake() {
        const snakeCells = new Uint32Array(
            wasm.memory.buffer,
            world.snake_cells(),
            world.snake_length()
        )
        snakeCells
            .filter((cellIdx, i) => !(i > 0 && cellIdx === snakeCells[0]))
            .forEach((cellIndex, i) => {
                const row = Math.floor(cellIndex / worldWidth)
                const col = cellIndex % worldWidth
                context.beginPath()
                context.fillStyle = i === 0 ? 'grey' : 'black'
                context.fillRect(
                    col * CELL_SIZE,
                    row * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE,
                )
            })
        context.stroke()
    }

    function drawReward() {
        const index = world.reward_cell()
        const row = Math.floor(index / worldWidth)
        const col = index % worldWidth

        context.beginPath()
        context.fillStyle = 'red'
        context.fillRect(
            col * CELL_SIZE,
            row * CELL_SIZE,
            CELL_SIZE,
            CELL_SIZE,
        )
        context.stroke()
    }

    function drawScore() {
        gameStatus.textContent = world.game_status_info() === "None" ? undefined : world.game_status_info();
        gameScore.textContent = String(world.score());
    }


    function draw() {
        drawWorld()
        drawSnake()
        drawReward()
        drawScore()
    }

    function run() {
        const status = world.game_status()
        if (!(status === GameStatus.Playing)) {
            gameControlBtn.textContent = "Replay"
            return
        }
        setTimeout(() => {
            context.clearRect(0, 0, canvas.width, canvas.height)
            world.update()
            draw()
            requestAnimationFrame(run)
        }, 1000 / fps)
    }

    draw()
})