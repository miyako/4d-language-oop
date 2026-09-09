// property read
var $receiver : 4D.OutgoingMessage
$receiver:=4D.OutgoingMessage.new()
var $read1 : Integer
$read1:=$receiver.status
// property write
$receiver:=4D.OutgoingMessage.new()
$receiver.status:=1
