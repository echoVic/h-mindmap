import React, { useEffect } from 'react';
import initWasm, { render_mindmap } from '../rust-wasm/rust_render.js';

const MindMap = ({ containerId }) => {
  useEffect(() => {
    async function runWasm() {
      try {
        console.log("Initializing WebAssembly module...");
        await initWasm();
        console.log("WebAssembly module initialized successfully.");
        render_mindmap(containerId); // 传入容器 ID
        console.log("render_mindmap called successfully.");
      } catch (error) {
        console.error("Error in initializing WebAssembly or rendering:", error);
      }
    }

    if (containerId) {
      runWasm().catch(console.error);
    }
  }, [containerId]);

  return <div id={containerId}></div>;
};

export default MindMap;