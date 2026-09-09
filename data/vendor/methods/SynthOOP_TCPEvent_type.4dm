// property read
// TCPEvent instances are only ever supplied by 4D to a callback: Function onData($event : 4D.TCPEvent)
// There is no expression that constructs one, so the receiver is a bare typed declaration.
var $receiver : 4D.TCPEvent
var $read1 : Text
$read1:=$receiver.type
// property write
// TCPEvent instances are only ever supplied by 4D to a callback: Function onData($event : 4D.TCPEvent)
// There is no expression that constructs one, so the receiver is a bare typed declaration.
$receiver.type:="synthText"
