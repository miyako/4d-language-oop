// property read
var $receiver : 4D.OutgoingMessage
$receiver:=4D.OutgoingMessage.new()
var $read1 : Variant
$read1:=$receiver.body
// property write
$receiver:=4D.OutgoingMessage.new()
$receiver.body:="synthText"
