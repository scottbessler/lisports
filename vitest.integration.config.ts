import { defineConfig } from 'vitest/config';

export default defineConfig({
	resolve: {
		tsconfigPaths: true,
	},
	test: {
		include: ['test/integration/**/*.test.ts'],
		testTimeout: 30000,
		hookTimeout: 30000,
		env: {
			DATA_PATH: 'data',
		},
	},
});
