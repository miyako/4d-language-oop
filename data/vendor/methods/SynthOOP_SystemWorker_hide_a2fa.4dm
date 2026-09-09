// property read
var $receiver : 4D.SystemWorker
$receiver:=4D.SystemWorker.new("ls -l")
var $read1 : Boolean
$read1:=$receiver.hideWindow
// property write
$receiver:=4D.SystemWorker.new("ls -l")
$receiver.hideWindow:=True
