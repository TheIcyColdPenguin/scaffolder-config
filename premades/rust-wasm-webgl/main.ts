import "./style.css";
import init, { initialise, create_app } from "innards";

init().then(() => {
    initialise();

    const canvas = document.querySelector("canvas");
    if (!canvas) {
        alert("canvas not found");
        return;
    }
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;

    const context = canvas.getContext("webgl2");
    if (!context) {
        alert("Couldn't get context");
        return;
    }

    const app = create_app(context, canvas.width, canvas.height);

    requestAnimationFrame(function animate(time) {
        app.draw(time / 1000);
        requestAnimationFrame(animate);
    });
});
