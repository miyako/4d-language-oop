// property read
var $receiver : 4D.FileHandle
$receiver:=File("/PACKAGE/data.txt").open("write")
var $read1 : Real
$read1:=$receiver.offset
// property write
$receiver:=File("/PACKAGE/data.txt").open("write")
$receiver.offset:=1
