import init, { Game } from "./pkg/dinnye.js";

async function run() {
    await init();
    const canvas = document.getElementById("gameCanvas");
    const ctx = canvas.getContext("2d");

    const game = new Game();

    canvas.addEventListener("click", (e) => {
        const rect = canvas.getBoundingClientRect();
        game.spawn_fruit(e.clientX - rect.left, e.clientY - rect.top);
    });

    function loop() {
        game.tick();
        const fruit = game.get_fruit();

        ctx.clearRect(0, 0, canvas.width, canvas.height);
        ctx.beginPath();
        ctx.arc(fruit.x(), fruit.y(), 20, 0, 2 * Math.PI);
        ctx.fillStyle = "orange";
        ctx.fill();

        requestAnimationFrame(loop);
    }

    loop();
}

run();
