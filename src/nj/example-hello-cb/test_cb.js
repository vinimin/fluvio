let addon = require('./dylib');

addon.hello(function(msg){
  console.log(msg); // 'hello world'
});
