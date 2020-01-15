let addon = require('./flv_lib');
addon.connect_sc("localhost:9003").then( sc => {
    sc.find_leader().then( leader => {
        leader.produce({}).then( () => {});
        leader.consume("order"); // create emitter

        });
    });
    console.log("sc created");
});