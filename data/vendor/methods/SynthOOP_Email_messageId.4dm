// property read
var $receiver : 4D.Email
$receiver:=4D.Email.new()
var $read1 : Text
$read1:=$receiver.messageId
// property write
$receiver:=4D.Email.new()
$receiver.messageId:="synthText"
