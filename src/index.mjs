import { h, render } from 'https://esm.sh/preact';

document.addEventListener("DOMContentLoaded", () => {
    setInterval(async () => {
        let response = await fetch('/api/cpus');
        if( response.status !== 200 ) {
            throw new Error('Http error! status: ${response.status}');
        }

        let json = await response.json();

        // document.body.textContent = JSON.stringify(json, null, 2);

        const app = h('pre', null, JSON.stringify(json, null, 2));

        render(app, document.body);
    }, 1000);

});