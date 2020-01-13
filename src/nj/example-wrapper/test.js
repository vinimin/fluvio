let addon = require('./dylib');

let obj = new addon.MyObject(10);

console.log("value is ",obj.value);

console.log(obj);
console.log(obj.plusOne()); // 11

let obj2 = obj.multiply(-1);
console.log("multiple ",obj2.value);