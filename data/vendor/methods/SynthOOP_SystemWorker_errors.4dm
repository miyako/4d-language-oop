// property read
var $receiver : 4D.SystemWorker
$receiver:=4D.SystemWorker.new("ls -l")
var $read1 : Collection
$read1:=$receiver.errors
// property write
$receiver:=4D.SystemWorker.new("ls -l")
$receiver.errors:=New collection
