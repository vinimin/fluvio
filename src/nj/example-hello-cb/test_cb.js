let addon = require('./dylib');

addon.hello(2,function(msg){
  console.log(msg); // 'argument is {}'
});
