let addon = require('./flv_lib');
addon.connect_sc("localhost:9003",(sc) => {

});
console.log(addon.hello()); // 'hello'