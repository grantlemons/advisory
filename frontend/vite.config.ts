import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig(({ mode }) => {
    return {
        plugins: [sveltekit()],
        server: {
            proxy: {
                '/api': {
                    target: 'https://127.0.0.1:81',
                    changeOrigin: true,
                    secure: false,
                },
            },
            watch: {
                usePolling: true,
            },
        },
    };
});
