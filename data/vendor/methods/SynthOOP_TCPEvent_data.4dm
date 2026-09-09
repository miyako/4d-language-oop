// property read
// TCPEvent instances are only ever supplied by 4D to a callback: Function onData($event : 4D.TCPEvent)
// There is no expression that constructs one, so the receiver is a bare typed declaration.
var $receiver : 4D.TCPEvent
var $read1 : Blob
$read1:=$receiver.data
// property write
// TCPEvent instances are only ever supplied by 4D to a callback: Function onData($event : 4D.TCPEvent)
// There is no expression that constructs one, so the receiver is a bare typed declaration.
var $data2 : Blob
$receiver.data:=$data2
