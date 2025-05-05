import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { resolve } from 'path'
import AutoImport from 'unplugin-auto-import/vite'
import Components from 'unplugin-vue-components/vite'
import { ElementPlusResolver } from 'unplugin-vue-components/resolvers'

const host = process.env.TAURI_DEV_HOST

// https://vitejs.dev/config/
export default defineConfig(async () => ({
	server: {
		port: 1420,
		strictPort: true,
		host: host || false,
		hmr: host
			? {
					protocol: 'ws',
					host,
					port: 1421
			  }
			: undefined,
		watch: {
			// 3. tell vite to ignore watching `src-tauri`
			ignored: ['**/src-tauri/**']
		}
	},
	resolve: {
		alias: {
			'@': resolve(__dirname, './src')
		}
	},
	plugins: [
		vue(),
		AutoImport({
			imports: ['vue'],
			dts: 'src/auto-import.d.ts',
			resolvers: [ElementPlusResolver()]
		}),
		Components({
			include: [/\.vue$/, /\.vue\?vue/, /\?vue/, /\.tsx$/],
			dts: 'src/components.d.ts',
			resolvers: [ElementPlusResolver()]
		})
	],

	// Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
	//
	// 1. prevent vite from obscuring rust errors
	clearScreen: false,
	// 2. tauri expects a fixed port, fail if that port is not available
	build: {
		/** 单个 chunk 文件的大小超过 2048KB 时发出警告 */
		chunkSizeWarningLimit: 2048,
		/** 禁用 gzip 压缩大小报告 */
		reportCompressedSize: false,
		/** 打包后静态资源目录 */
		assetsDir: 'static',
		rollupOptions: {
			output: {
				manualChunks: {
					vue: ['vue'],
					element: ['element-plus']
				}
			}
		}
	}
}))
