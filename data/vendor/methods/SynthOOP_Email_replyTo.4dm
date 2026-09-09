// property read [Text]
var $receiver : 4D.Email
$receiver:=4D.Email.new()
var $read1 : Text
$read1:=$receiver.replyTo
// property write [Text]
$receiver:=4D.Email.new()
$receiver.replyTo:="synthText"
// property read [Object]
$receiver:=4D.Email.new()
var $read2 : Object
$read2:=$receiver.replyTo
// property write [Object]
$receiver:=4D.Email.new()
$receiver.replyTo:=New object
// property read [Collection]
$receiver:=4D.Email.new()
var $read3 : Collection
$read3:=$receiver.replyTo
// property write [Collection]
$receiver:=4D.Email.new()
$receiver.replyTo:=New collection
