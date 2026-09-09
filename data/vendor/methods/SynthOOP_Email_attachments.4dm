// property read
var $receiver : 4D.Email
$receiver:=4D.Email.new()
var $read1 : Collection
$read1:=$receiver.attachments
// property write
$receiver:=4D.Email.new()
$receiver.attachments:=New collection
