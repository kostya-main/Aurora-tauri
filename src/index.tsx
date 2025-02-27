import './runtime/assets/sass/index.sass';

import React from 'react';
import ReactDOM from 'react-dom/client';

import App from './App';

document.oncontextmenu = () => false;

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
    <React.StrictMode>
        <App />
    </React.StrictMode>,
);