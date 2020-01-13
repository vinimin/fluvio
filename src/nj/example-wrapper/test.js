let addon = require('./dylib');

function FindAllMethods(obj) {
    return Object.getOwnPropertyNames(obj).filter(function(property) {
        return typeof obj[property] == "function";
    });
}

console.log(FindAllMethods( addon.MyObject));

let obj = new addon.MyObject(10);

console.log("value is ",obj.value);

console.log(obj);
console.log(obj.plusOne()); // 11
