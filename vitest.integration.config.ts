import tsconfigPaths from 'vite-tsconfig-paths';
import { defineConfig } from 'vitest/config';

export default defineConfig({
	plugins: [tsconfigPaths()],
	test: {
		include: ['test/integration/**/*.test.ts'],
		testTimeout: 30000,
		hookTimeout: 30000,
		env: {
			DATA_PATH: 'data',
		},
	},
});
