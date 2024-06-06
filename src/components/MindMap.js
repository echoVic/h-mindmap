import React, { useEffect } from 'react';
import initWasm, { render_mindmap } from '../rust-wasm/rust_render.js';

const MindMap = () => {
  useEffect(() => {
    async function runWasm() {
      try {
        console.log("Initializing WebAssembly module...");
        await initWasm();
        console.log("WebAssembly module initialized successfully.");
        render_mindmap();
        console.log("render_mindmap called successfully.");
      } catch (error) {
        console.error("Error in initializing WebAssembly or rendering:", error);
      }
    }
    runWasm().catch(console.error);
  }, []);

  return (
    <canvas id="mindmap-canvas" width="800" height="600"></canvas>
  );
};

export default MindMap;