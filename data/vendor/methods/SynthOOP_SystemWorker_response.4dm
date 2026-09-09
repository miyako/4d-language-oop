// property read [Text]
var $receiver : 4D.SystemWorker
$receiver:=4D.SystemWorker.new("ls -l")
var $read1 : Text
$read1:=$receiver.response
// property read [Blob]
$receiver:=4D.SystemWorker.new("ls -l")
var $read2 : Blob
$read2:=$receiver.response
