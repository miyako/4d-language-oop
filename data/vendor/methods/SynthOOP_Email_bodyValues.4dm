// property read
var $receiver : 4D.Email
$receiver:=4D.Email.new()
var $read1 : Object
$read1:=$receiver.bodyValues
// property write
$receiver:=4D.Email.new()
$receiver.bodyValues:=New object
