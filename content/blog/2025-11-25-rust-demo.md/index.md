+++
title = "Rust demo"
authors = ["Raph Levien"]
+++

Can we run some Rust code in here?

<div id="wasm-demo-container">
    <input type="text" id="name-input" value="Zola User">
    <button onclick="runWasmDemo()">Greet</button>
    <p id="output"></p>
</div>

<script type="module">
    // Adjust the path to your static Wasm files
    import init, { greet } from '/rust/rust_demo.js'; 

    async function runWasmDemo() {
        // Initialize Wasm module
        await init();

        // Get input value
        const name = document.getElementById('name-input').value;

        // Call the Rust function
        const greeting = greet(name);

        // Display the result
        document.getElementById('output').textContent = greeting;
    }

    // Define the function globally so the button's onclick can find it
    window.runWasmDemo = runWasmDemo; 

    // Automatically run on load (optional)
    // runWasmDemo();
</script>
