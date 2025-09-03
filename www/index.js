import init, { WasmGame } from "./pkg/dinnye.js";

async function run() {
    await init();
    const canvas = document.getElementById("gameCanvas");
    const ctx = canvas.getContext("2d");

    const game = new WasmGame();

    canvas.addEventListener("click", (e) => {
        const rect = canvas.getBoundingClientRect();
        game.spawn_fruit(e.clientX - rect.left, e.clientY - rect.top);
    });

    function loop() {
        game.tick();

        const buffer = game.get_fruit_buffer(); // Float32Array
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        console.log(buffer);

        for (let i = 0; i < buffer.length; i += 4) {
            const x = buffer[i];
            const y = buffer[i + 1];
            const rotation = buffer[i + 2];
            const radius = buffer[i + 3];

            ctx.save();
            ctx.translate(x, y);
            ctx.rotate(rotation);
            ctx.beginPath();
            ctx.arc(0, 0, radius, 0, 2 * Math.PI);
            ctx.fillStyle = "orange";
            ctx.fill();
            ctx.restore();
        }

        requestAnimationFrame(loop);
    }

    loop();
}

run();
