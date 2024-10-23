module.exports = defineTest({
	description: 'buildStart hooks can use this.error',
	options: {
		plugins: [
			{
				name: 'test',
				resolveId() {
					this.error('nope');
				}
			}
		]
	},
	error: {
		code: 'PLUGIN_ERROR',
		plugin: 'test',
		message: '[plugin test] nope',
		hook: 'resolveId'
	}
});
