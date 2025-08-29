import {defineConfig, searchForWorkspaceRoot} from 'vite';

/** @type {import('vite').UserConfig} */
export default defineConfig({
    build: {
        assetsDir: 'static',
        emptyOutDir: true,
        manifest: true,
        outDir: 'dist',
    },
    resolve: {
        preserveSymlinks: false
    },
    server: {
        host: 'kreida.irbis-labs.com.local',
        port: 8003,
        cors: true,
        headers: {
            'Access-Control-Allow-Origin': '*'
        },
        fs: {
            allow: [
                searchForWorkspaceRoot(process.cwd()),
                '../../crates',
            ],
        }
    },
});
