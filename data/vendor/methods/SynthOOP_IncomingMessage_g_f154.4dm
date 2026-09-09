// overload 0
// IncomingMessage instances are only ever supplied by 4D to a callback: Function handle($request : 4D.IncomingMessage) : 4D.OutgoingMessage
// There is no expression that constructs one, so the receiver is a bare typed declaration.
var $receiver : 4D.IncomingMessage
var $result1 : Text
$result1:=$receiver.getText()
