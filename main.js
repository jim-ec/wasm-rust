const { add, length } = require('.');

console.log("Hello world from JS");
const x = add(1, 2);
console.log(x);

length([1, 2, 3, { 'a': 5 }]);
