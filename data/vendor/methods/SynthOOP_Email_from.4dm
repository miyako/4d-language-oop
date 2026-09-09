// property read [Text]
var $receiver : 4D.Email
$receiver:=4D.Email.new()
var $read1 : Text
$read1:=$receiver.from
// property write [Text]
$receiver:=4D.Email.new()
$receiver.from:="synthText"
// property read [Object]
$receiver:=4D.Email.new()
var $read2 : Object
$read2:=$receiver.from
// property write [Object]
$receiver:=4D.Email.new()
$receiver.from:=New object
// property read [Collection]
$receiver:=4D.Email.new()
var $read3 : Collection
$read3:=$receiver.from
// property write [Collection]
$receiver:=4D.Email.new()
$receiver.from:=New collection
