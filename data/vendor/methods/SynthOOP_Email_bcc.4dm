// property read [Text]
var $receiver : 4D.Email
$receiver:=4D.Email.new()
var $read1 : Text
$read1:=$receiver.bcc
// property write [Text]
$receiver:=4D.Email.new()
$receiver.bcc:="synthText"
// property read [Object]
$receiver:=4D.Email.new()
var $read2 : Object
$read2:=$receiver.bcc
// property write [Object]
$receiver:=4D.Email.new()
$receiver.bcc:=New object
// property read [Collection]
$receiver:=4D.Email.new()
var $read3 : Collection
$read3:=$receiver.bcc
// property write [Collection]
$receiver:=4D.Email.new()
$receiver.bcc:=New collection
