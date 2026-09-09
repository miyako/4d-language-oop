// property read
// TCPEvent instances are only ever supplied by 4D to a callback: Function onData($event : 4D.TCPEvent)
// There is no expression that constructs one, so the receiver is a bare typed declaration.
var $receiver : 4D.TCPEvent
var $read1 : Real
$read1:=$receiver.port
// property write
// TCPEvent instances are only ever supplied by 4D to a callback: Function onData($event : 4D.TCPEvent)
// There is no expression that constructs one, so the receiver is a bare typed declaration.
$receiver.port:=1
