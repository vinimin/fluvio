let addon = require('./dylib');
addon.connectSc("localhost:9003").then( sc => {
    console.log("connect to sc at ",sc.addr());
    sc.findLeader("test",0).then( leader => {
        leader.produce("hello world").then( len => {
            console.log("message send");
        })
       
    })
});