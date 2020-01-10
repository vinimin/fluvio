let addon = require('./dylib');

addon(function(msg){
  console.log(msg); // 'hello world'
});
