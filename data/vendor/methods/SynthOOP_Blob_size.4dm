// property read
var $receiver : 4D.Blob
$receiver:=4D.Blob.new()
var $read1 : Real
$read1:=$receiver.size
// property write
$receiver:=4D.Blob.new()
$receiver.size:=1
