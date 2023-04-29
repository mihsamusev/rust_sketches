const rust = import("./pkg/wasm3d");
const canvas = document.getElementById("rustCanvas");
const gl = canvas.getContext("webgl", { antialias: true });

rust.then((m) => {
  if (!gl) {
    alert("WebGL bullshit");
    return;
  }

  const FPS = 1000.0 / 30.0; // ms / frams
  const client = new m.Client();

  const initTime = Date.now();
  var lastDrawTime = -1; // ms

  function render() {
    window.requestAnimationFrame(render);
    const currTime = Date.now();

    if (currTime >= lastDrawTime + FPS) {
      lastDrawTime = currTime;

      if (
        window.innerHeight != canvas.height ||
        window.innerWidth != canvas.width
      ) {
        canvas.height = window.innerHeight;
        canvas.clientHeight = window.innerHeight;
        canvas.style.height = window.innerHeight;

        canvas.width = window.innerWidth;
        canvas.clientWidth = window.innerWidth;
        canvas.style.width = window.innerWidth;

        gl.viewport(0, 0, window.innerWidth, window.innerHeight);
      }

      let elapsedTime = currTime - initTime;
      // update data (rust)
      client.update(elapsedTime, window.innerHeight, window.innerWidth);

      // render data (rust)
      client.render();
    }
  }

  render();
});
