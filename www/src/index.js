import init, {hello} from "wasm_game"

init().then(() => {
    hello("Canvas");
    console.log("OK")
})