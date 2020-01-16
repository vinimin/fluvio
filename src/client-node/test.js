let addon = require('./dylib');
addon.connectSc("localhost:9003").then( sc => {
    console.log("connect to sc at ",sc.addr());
});