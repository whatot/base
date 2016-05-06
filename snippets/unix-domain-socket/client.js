var net = require('net')
var log = require('npmlog')
var sockfile = './communicate.sock';
var client = net.connect( { path: sockfile  } );
var processCount = 20

client
.on('connect', function () {
	log.info('client', 'client connected');
	client.write('hello server');
})
.on('data', function (data) {
	log.info('client', 'Data: %s , left :%d', data.toString(), processCount);
	processCount--;
	if (processCount < 0) {
		client.write("DONE");
		client.end();
	} else {
		client.write("recieved : " + data.toString() + ", waiting next", 'utf8');
	}
})
.on('error', function (err) {
	log.error('client', err);
})
.on('end', function () {
	log.info('client', 'client disconnected');
});
