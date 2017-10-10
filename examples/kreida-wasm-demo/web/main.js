;(() => {
    "use strict";

    var Module = window.Module = {
        preRun: [],
        postRun: [main],
        wasmBinaryFile: "kreida-wasm-demo.wasm",
        // onRuntimeInitialized: main,
    };

    function main() {
        const canvas_el = document.createElement("canvas");
        canvas_el.setAttribute("id", "canvas");
        document.body.appendChild(canvas_el);
        // const canvas = document.querySelector("#canvas");
        const ctx = canvas_el.getContext('2d');

        let canvas = _canvas_new(~~(canvas_el.offsetWidth), ~~(canvas_el.offsetHeight));

        window.addEventListener("resize", resize);
        document.addEventListener("tick", tick);

        resize();
        fire_tick();

        function resize() {
            let width = ~~(canvas_el.offsetWidth);
            let height = ~~(canvas_el.offsetHeight);
            if (width !== canvas_el.width || height !== canvas_el.height) {
                canvas_el.width = width;
                canvas_el.height = height;
                _canvas_resize(canvas, width, height);
            }
        }

        function tick() {
            requestAnimationFrame((tm) => {
                // console.time('total');
                // console.time('render');
                let buf_ptr = _canvas_buf_as_ptr(canvas);
                let buf_len = canvas_el.width * canvas_el.height;
                _rotating(buf_len, buf_ptr, canvas_el.width, tm);
                // console.timeEnd('render');
                // console.time('copy');
                let buf = new Uint8ClampedArray(Module.HEAPU8.buffer, buf_ptr, buf_len * 4);
                ctx.putImageData(new ImageData(buf, canvas_el.width, canvas_el.height), 0, 0);
                // console.timeEnd('copy');
                // console.timeEnd('total');
                fire_tick();
            });
        }

        function fire_tick() {
            let event = document.createEvent("HTMLEvents");
            event.initEvent("tick", true, true);
            document.dispatchEvent(event);
        }
    }
})();
