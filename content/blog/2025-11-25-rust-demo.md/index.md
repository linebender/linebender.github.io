+++
title = "Rust demo"
authors = ["Raph Levien"]
+++

Can we run some Rust code in here?

<div id="wasm-demo-container">
</div>

<style>
    input {
        width: 100%;
    }
</style>

<script type="module">
    // Adjust the path to your static Wasm files
    import init, { start } from '/rust/rust_demo.js'; 

    async function runWasmDemo() {
        // Initialize Wasm module
        await init();

        // Call the Rust function
        const greeting = start("argument");
    }

    // Define the function globally so the button's onclick can find it
    window.runWasmDemo = runWasmDemo; 

    // Automatically run on load (optional)
    runWasmDemo();
</script>
