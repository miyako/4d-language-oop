// property read [Text]
var $receiver : 4D.Email
$receiver:=4D.Email.new()
var $read1 : Text
$read1:=$receiver.cc
// property write [Text]
$receiver:=4D.Email.new()
$receiver.cc:="synthText"
// property read [Object]
$receiver:=4D.Email.new()
var $read2 : Object
$read2:=$receiver.cc
// property write [Object]
$receiver:=4D.Email.new()
$receiver.cc:=New object
// property read [Collection]
$receiver:=4D.Email.new()
var $read3 : Collection
$read3:=$receiver.cc
// property write [Collection]
$receiver:=4D.Email.new()
$receiver.cc:=New collection
