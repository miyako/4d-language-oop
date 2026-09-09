// property read
var $receiver : 4D.FileHandle
$receiver:=File("/PACKAGE/data.txt").open("write")
var $read1 : Text
$read1:=$receiver.breakModeRead
