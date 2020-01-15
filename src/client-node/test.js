let addon = require('./flv_lib');
addon.connect_sc("localhost:9003").then( sc => {
    console.log("sc created");
});