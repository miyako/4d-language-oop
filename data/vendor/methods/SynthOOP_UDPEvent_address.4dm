// property read
// UDPEvent instances are only ever supplied by 4D to a callback: Function onData($event : 4D.UDPEvent)
// There is no expression that constructs one, so the receiver is a bare typed declaration.
var $receiver : 4D.UDPEvent
var $read1 : Text
$read1:=$receiver.address
// property write
// UDPEvent instances are only ever supplied by 4D to a callback: Function onData($event : 4D.UDPEvent)
// There is no expression that constructs one, so the receiver is a bare typed declaration.
$receiver.address:="synthText"
