;(() => {
    "use strict";

    let Module = window.Module = {
        preRun: [],
        postRun: [main],
        wasmBinaryFile: "kreida-wasm-chart.wasm",
        // onRuntimeInitialized: main,
    };

    function Chart() {
        const canvas_el = document.createElement("canvas");
        canvas_el.classList.add("chart");
        document.body.appendChild(canvas_el);
        const ctx = canvas_el.getContext('2d');

        let canvas = _canvas_new(~~(canvas_el.offsetWidth), ~~(canvas_el.offsetHeight));

        let model = _row_new();
        for (let i = 0; i < 100; i++) {
            _row_push(model, Math.random() * 2.0 - 1.0);
        }

        this.el = canvas_el;
        this.resize = resize;
        this.repaint = repaint;
        this.touch = touch;

        function touch() {
            _row_push(model, Math.random() * 2.0 - 1.0);
        }

        function resize() {
            let width = ~~(canvas_el.offsetWidth);
            let height = ~~(canvas_el.offsetHeight);
            if (width !== canvas_el.width || height !== canvas_el.height) {
                canvas_el.width = width;
                canvas_el.height = height;
                _canvas_resize(canvas, width, height);
            }
        }

        function repaint() {
            console.time('total');
            // console.time('data');
            // console.timeEnd('data');
            console.time('render');
            _sinusoid(canvas, model);
            console.timeEnd('render');
            console.time('copy');
            let buf_size = canvas_el.width * canvas_el.height * 4;
            let buf_ptr = new Uint8ClampedArray(Module.HEAPU8.buffer, _canvas_buf_as_ptr(canvas), buf_size);
            ctx.putImageData(new ImageData(buf_ptr, canvas_el.width, canvas_el.height), 0, 0);
            console.timeEnd('copy');
            console.timeEnd('total');
        }
    }

    function main() {
        window.addEventListener("resize", resize);
        document.addEventListener("tick", tick);

        const charts = [];
        for (let i = 0; i < 35; i++) {
            charts.push(new Chart());
        }

        resize();
        update();

        function update() {
            let i = Math.floor(Math.random() * charts.length);
            let chart = charts[i];
            chart.touch();
            chart.repaint();
            setTimeout(update, 500);
        }

        function resize() {
            // const canvas = document.querySelector("#canvas");
            for (let chart of charts) {
                chart.resize();
            }
            fire_tick();
        }

        function tick() {
            for (let chart of charts) {
                chart.repaint();
            }
        }

        function fire_tick() {
            let event = document.createEvent("HTMLEvents");
            event.initEvent("tick", true, true);
            document.dispatchEvent(event);
        }
    }
})();
