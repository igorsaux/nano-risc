if('serviceWorker' in navigator) {window.addEventListener('load', () => {navigator.serviceWorker.register('/nano-risc/sw.js', { scope: '/nano-risc/' })})}