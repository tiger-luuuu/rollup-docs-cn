const path = require('node:path');
const ID_MAIN = path.join(__dirname, 'main.js');

module.exports = defineTest({
	description: 'throws when redeclaring a top-level var binding as a function',
	error: {
		code: 'REDECLARATION_ERROR',
		frame: `
			1: var foo;
			2: function foo() {}
			            ^`,
		id: ID_MAIN,
		loc: {
			column: 9,
			file: ID_MAIN,
			line: 2
		},
		message: 'Identifier "foo" has already been declared',
		pos: 18,
		watchFiles: [ID_MAIN]
	}
});
