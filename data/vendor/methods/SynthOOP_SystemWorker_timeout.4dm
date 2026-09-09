// property read
var $receiver : 4D.SystemWorker
$receiver:=4D.SystemWorker.new("ls -l")
var $read1 : Integer
$read1:=$receiver.timeout
