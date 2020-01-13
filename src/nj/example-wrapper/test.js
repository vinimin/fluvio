let addon = require('./dylib');

let obj = new addon.MyObject(10);

console.log(obj.plusOne()); // 11