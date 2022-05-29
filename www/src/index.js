import init, {
    World
} from 'wasm_game'

init().then(() => {
    const CELL_SIZE = 20
    const CELL_AMOUNT = 21
    const world = World.new(CELL_AMOUNT)
    const worldWidth = world.width()
    const fps = 5

    const canvas = document.getElementById('snake-world')
    const context = canvas.getContext('2d')

    canvas.width = worldWidth * CELL_SIZE
    canvas.height = worldWidth * CELL_SIZE

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
        const snake_index = world.snake_head_index()
        const col = snake_index % worldWidth
        const row = Math.floor(snake_index / worldWidth)

        context.beginPath()
        context.fillRect(
            col * CELL_SIZE,
            row * CELL_SIZE,
            CELL_SIZE,
            CELL_SIZE,
        )
        context.stroke()
    }

    function draw() {
        drawWorld()
        drawSnake()
    }

    function run() {
        setTimeout(() => {
            context.clearRect(0, 0, canvas.width, canvas.height)
            world.update()
            draw()
            requestAnimationFrame(run)
        }, 1000 / fps)
    }

    draw()
    run()
})