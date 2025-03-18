const init = await import("./pkg/game.js");
init().then(() => console.log("WASM Loaded"));

// come back to wasm in the future
// for now focus on desktop experience
