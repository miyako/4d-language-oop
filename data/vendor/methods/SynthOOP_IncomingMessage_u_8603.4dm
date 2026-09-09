// property read
// IncomingMessage instances are only ever supplied by 4D to a callback: Function handle($request : 4D.IncomingMessage) : 4D.OutgoingMessage
// There is no expression that constructs one, so the receiver is a bare typed declaration.
var $receiver : 4D.IncomingMessage
var $read1 : Object
$read1:=$receiver.urlQuery
