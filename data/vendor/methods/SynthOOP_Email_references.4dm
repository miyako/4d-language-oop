// property read
var $receiver : 4D.Email
$receiver:=4D.Email.new()
var $read1 : Collection
$read1:=$receiver.references
// property write
$receiver:=4D.Email.new()
$receiver.references:=New collection
