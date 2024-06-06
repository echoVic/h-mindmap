// App.js
import React from 'react';
import { RecoilRoot } from 'recoil';
import MindMap from './components/MindMap';

function App() {
  return (
    <RecoilRoot>
      <div className="App">
        <MindMap />
      </div>
    </RecoilRoot>
  );
}

export default App;