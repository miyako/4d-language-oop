// overload 0
var $receiver : 4D.SystemWorker
$receiver:=4D.SystemWorker.new("ls -l")
$receiver.postMessage("synthText")
// overload 1
$receiver:=4D.SystemWorker.new("ls -l")
var $messageBLOB1 : Blob
$receiver.postMessage($messageBLOB1)
