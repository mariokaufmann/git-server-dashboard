/* @refresh reload */
import { render } from 'solid-js/web';

import './index.css';
import App from './App';
import 'modern-normalize/modern-normalize.css';
import '@fortawesome/fontawesome-free/css/solid.min.css';
import '@fortawesome/fontawesome-free/css/fontawesome.min.css';

import relativeTime from 'dayjs/plugin/relativeTime';
import dayjs from 'dayjs';

dayjs.extend(relativeTime);

const root = document.getElementById('root');

if (import.meta.env.DEV && !(root instanceof HTMLElement)) {
  throw new Error(
    'Root element not found. Did you forget to add it to your index.html? Or maybe the id attribute got mispelled?'
  );
}

fetch('/api/version')
  .then((response) => response.text())
  .then((serverVersion) => console.log(`Server version: ${serverVersion}.`));
render(() => <App />, root!);
