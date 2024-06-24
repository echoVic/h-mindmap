import React, { useEffect, useState } from 'react';
import initWasm, { init_mindmap, add_node } from '../rust-wasm/rust_render.js';

const MindMap = ({ containerId }) => {
  const [wasmInitialized, setWasmInitialized] = useState(false);

  useEffect(() => {
    async function runWasm() {
      try {
        console.log("Initializing WebAssembly module...");
        await initWasm();
        console.log("WebAssembly module initialized successfully.");
        init_mindmap(containerId);
        console.log("init_mindmap called successfully.");
        setWasmInitialized(true);
      } catch (error) {
        console.error("Error in initializing WebAssembly or rendering:", error);
      }
    }

    if (containerId) {
      runWasm().catch(console.error);
    }
  }, [containerId]);

  const handleAddNode = () => {
    const name = prompt("Enter node name:");
    const x = parseFloat(prompt("Enter x coordinate:"));
    const y = parseFloat(prompt("Enter y coordinate:"));
    if (name && !isNaN(x) && !isNaN(y)) {
      add_node(name, x, y, false);
    }
  };

  return (
    <div>
      <div id={containerId}></div>
      {wasmInitialized && <button onClick={handleAddNode}>Add Node</button>}
    </div>
  );
};

export default MindMap;