import init, {
    World
} from 'wasm_game'

init().then(() => {
    const CELL_SIZE = 20
    const CELL_AMOUNT = 16;
    const world = World.new(CELL_AMOUNT)
    const worldWidth = world.width()

    const canvas = document.getElementById('snake-world')
    const context = canvas.getContext('2d')

    canvas.width = worldWidth * CELL_SIZE
    canvas.height = worldWidth * CELL_SIZE

    function drawWorld() {
        context.beginPath()
        for (let x = 0; x < worldWidth + 1; x++) {
            context.moveTo(CELL_SIZE * x, 0)
            context.lineTo(CELL_SIZE * x, CELL_SIZE * worldWidth)
        }
        for (let y = 0; y < worldWidth + 1; y++) {
            context.moveTo(0, CELL_SIZE * y)
            context.lineTo(CELL_SIZE * worldWidth, CELL_SIZE * y)
        }
        context.stroke();
    }
    drawWorld();
})