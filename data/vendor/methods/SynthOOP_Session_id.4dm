// property read
var $receiver : 4D.Session
$receiver:=Session
var $read1 : Text
$read1:=$receiver.id
// property write
$receiver:=Session
$receiver.id:="synthText"
